//! # USB CDC ACM — 中断驱动示例
//!
//! 使用 `USBCTRL_IRQ` 驱动 USB，主循环仅 `wfi()` 休眠，无轮询。
//!
//! ## 行为
//! - 收到数据立即回显
//! - 每 100 ms（SysTick）发送一条 hello 消息
//!
//! ## 架构
//!
//! ```text
//! SysTick ──pend──► SPARE_IRQ_0（软中断，应用层）
//!                       └ TICK 置位时 → cdc.write("Hello")
//!
//! USB 硬件 ──IRQ──► USBCTRL_IRQ
//!                       │ poll USB
//!                       └ echo RX → cdc.write()
//!
//! Main: loop { wfi() }
//! ```
//!
//! ## 硬件
//! - Raspberry Pi Pico 2 (RP2350)
//! - USB 数据线连接到 Pico 的 USB 口

#![no_std]
#![no_main]

use core::cell::RefCell;

use defmt::*;
use defmt_rtt as _;

use roschi::block::ImageDef;
use roschi::clocks::ClockBuilder;
use roschi::interrupt::{self, Interrupt, Mutex};
use roschi::pac::Peripherals;
use roschi::rt::peripheral::Peripherals as CorePeripherals;
use roschi::{SysTick, bind, entry, exception};

use roschi_usb::bus::UsbBusAllocator;
use roschi_usb::class::UsbClass;
use roschi_usb::class::cdc_acm::CdcAcm;
use roschi_usb::device::{UsbDevice, UsbDeviceBuilder};
use roschi_usb::rp2350::Rp2350Bus;

// ──────────────────────────────────────────────────────────────────────────────
// Boot block
// ──────────────────────────────────────────────────────────────────────────────

#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::exe();

// ──────────────────────────────────────────────────────────────────────────────
// 全局状态
// ──────────────────────────────────────────────────────────────────────────────

// USB_ALLOC 赋予 UsbDevice/CdcAcm `'static` 生命周期。
// Safety：在 main 中、启用中断之前通过裸指针写入一次，之后只读。
static mut USB_ALLOC: Option<UsbBusAllocator<Rp2350Bus>> = None;

static USB_DEV: Mutex<RefCell<Option<UsbDevice<'static, Rp2350Bus>>>> =
    Mutex::new(RefCell::new(None));
static USB_CDC: Mutex<RefCell<Option<CdcAcm<'static, Rp2350Bus>>>> = Mutex::new(RefCell::new(None));

// ──────────────────────────────────────────────────────────────────────────────
// USB 中断处理程序
// ──────────────────────────────────────────────────────────────────────────────

#[bind(USBCTRL_IRQ)]
fn usbctrl_irq() {
    interrupt::free(|cs| {
        let mut dev = USB_DEV.borrow(cs).borrow_mut();
        let mut cdc = USB_CDC.borrow(cs).borrow_mut();

        if let (Some(dev), Some(cdc)) = (dev.as_mut(), cdc.as_mut()) {
            dev.poll(&mut [cdc as &mut dyn UsbClass<Rp2350Bus>]);

            if cdc.port_open() {
                // 回显接收到的数据
                let mut buf = [0u8; 64];
                loop {
                    match cdc.read(&mut buf) {
                        Ok(n) if n > 0 => {
                            let _ = cdc.write(&buf[..n]);
                        }
                        _ => break,
                    }
                }
            }
        }
    });
}

// ──────────────────────────────────────────────────────────────────────────────
// SPARE_IRQ_0 — 应用层软中断，负责定时发送
// ──────────────────────────────────────────────────────────────────────────────

#[bind(SPARE_IRQ_0)]
fn spare_irq_0() {
    interrupt::free(|cs| {
        if let Some(cdc) = USB_CDC.borrow(cs).borrow_mut().as_mut() {
            if cdc.port_open() {
                let _ = cdc.write(b"Hello from RP2350!\r\n");
            }
        }
    });
}

// ──────────────────────────────────────────────────────────────────────────────
// SysTick — 100 ms 节拍
// ──────────────────────────────────────────────────────────────────────────────

#[exception]
fn SysTick() {
    interrupt::pend(Interrupt::SPARE_IRQ_0);
}

// ──────────────────────────────────────────────────────────────────────────────
// Main
// ──────────────────────────────────────────────────────────────────────────────

#[entry]
fn main() -> ! {
    info!("USB CDC Interrupt Example starting...");

    let dp = Peripherals::take().unwrap();
    let cp = CorePeripherals::take().unwrap();

    let clocks = ClockBuilder::new()
        .enable_usb()
        .build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);

    // USB 初始化（通过裸指针写入 static mut，避免 static_mut_refs lint）
    //
    // Safety：在任何中断触发之前运行，无并发访问。
    let alloc: &'static UsbBusAllocator<Rp2350Bus> = unsafe {
        core::ptr::addr_of_mut!(USB_ALLOC).write(Some(UsbBusAllocator::new(Rp2350Bus::steal())));
        (*core::ptr::addr_of!(USB_ALLOC)).as_ref().unwrap()
    };

    let cdc = CdcAcm::new(alloc);
    let usb_dev = UsbDeviceBuilder::new(alloc, 0x2E8A, 0x000A)
        .device_release(0x0100)
        .manufacturer("Roschi")
        .product("USB Serial")
        .serial_number("001")
        .composite_with_iads()
        .build();

    usb_dev.bus().connect();

    interrupt::free(|cs| {
        *USB_CDC.borrow(cs).borrow_mut() = Some(cdc);
        *USB_DEV.borrow(cs).borrow_mut() = Some(usb_dev);
    });

    // SPARE_IRQ_0 优先级设为 128（中等），允许 USBCTRL_IRQ（默认 0，最高）抢占。
    // 有效值为 0,16,32,...,240；数字越小优先级越高。
    unsafe {
        interrupt::set_priority(Interrupt::SPARE_IRQ_0, 128);
    }

    interrupt::unpend(Interrupt::USBCTRL_IRQ);
    interrupt::unpend(Interrupt::SPARE_IRQ_0);
    unsafe {
        interrupt::enable(Interrupt::USBCTRL_IRQ);
        interrupt::enable(Interrupt::SPARE_IRQ_0);
    }
    info!("USB interrupt enabled");

    // SysTick：100 ms 节拍（sys_freq / 10 = 15_000_000，在 24-bit 上限 16_777_215 内）
    let mut systick = SysTick::new(cp.SYST, clocks.sys_freq());
    systick.disable();
    systick.set_reload(clocks.sys_freq() / 10);
    systick.enable_interrupt();
    systick.enable();

    info!("Ready");
    loop {
        roschi::rt::asm::wfi();
    }
}
