//! # USB 双舵机控制示例（中断驱动）— GPIO0 / GPIO1
//!
//! 与 `usb_servo` 相同的硬件与协议，但 USB 状态机、命令解析和舵机
//! 更新**全部在 `USBCTRL_IRQ` 中断里完成**，主循环只 `wfi()` 休眠。
//!
//! > 面向使用者的完整接口文档（接线、上位机准备、协议、示例、排错）见
//! > [`docs/usb_servo_protocol.md`](../../../docs/usb_servo_protocol.md)。
//!
//! ---
//!
//! # 1. 架构
//!
//! ```text
//!   USB 硬件 ──IRQ──► USBCTRL_IRQ
//!                        ├─ dev.poll()                  USB 状态机
//!                        ├─ cdc.read() → 行缓冲 → 解析
//!                        │      └─ Slice::<0>::steal().set_duty(a, b)
//!                        └─ cdc.write()（WouldBlock 时保留到下次中断）
//!
//!   Main: loop { wfi() }
//! ```
//!
//! PWM 波形由硬件在配置好后自动维持，中断里只写一次 CC 寄存器，
//! 因此 CPU 绝大部分时间处于低功耗休眠。
//!
//! ## 1.1 为什么不需要 `Mutex<RefCell<...>>`
//!
//! 传统做法把 `UsbDevice` / `CdcAcm` 放进 `Mutex<RefCell<Option<...>>>`
//! 以便中断和主循环共享。本示例只在 **USBCTRL_IRQ** 中访问这些状态:
//!
//! - 外设句柄在 `main` 中写入 `static mut`，使能中断后不再被 `main` 触碰；
//! - 之后唯一的读者/写者是 USBCTRL_IRQ（NVIC 默认最高优先级，不嵌套自身）；
//! - PWM 侧用 [`Slice::steal()`] 在中断里重新拿到 `slice 0` 的寄存器访问权。
//!
//! 所以全部状态只有单一所有者，用裸指针访问 `static mut` 即可，
//! 无需运行时借用检查，也没有临界区开销。
//!
//! `static mut` 一律通过 `core::ptr::addr_of_mut!` 取指针再解引用，
//! 避免 Rust 2024 的 `static_mut_refs` 限制。
//!
//! ---
//!
//! # 2. 硬件映射
//!
//! ```text
//!   GPIO0  →  slice 0, channel A  →  舵机 0
//!   GPIO1  →  slice 0, channel B  →  舵机 1
//! ```
//!
//! 同一 slice 共享计数器、分频器和 TOP，两个舵机**频率相同**，
//! 只有脉宽独立。`set_duty()` 一次写两个通道，同一 PWM 周期内生效。
//!
//! ---
//!
//! # 3. 通讯协议
//!
//! | 方向 | 内容 | 说明 |
//! |------|------|------|
//! | 上位机 → 设备 | `<角度0> <角度1>\n` | 两个角度，单位度，精度 0.1° |
//! | 设备 → 上位机 | `OK <角度0> <角度1>\r\n` | 命令生效，回显 0.1° |
//! | 设备 → 上位机 | `ERR\r\n` | 格式错误或角度超出范围 |
//!
//! 规则：
//!
//! - 两个角度可以用**空格、制表符或逗号**分隔
//! - 行结束符支持 `\n`、`\r` 或 `\r\n`，空行忽略
//! - 角度范围 `0.0 ~ 180.0`，可写整数（`90`）或一位小数（`90.5`）
//! - 小数超过一位或超出范围时返回 `ERR`
//!
//! ```text
//!   90 90        → 两个舵机都转到中位
//!   0.0,180.0    → 舵机 0 转到 0.0°，舵机 1 转到 180.0°
//!   45.5 135.0   → OK 45.5 135.0
//!   90.25 0      → ERR（小数超过一位）
//! ```
//!
//! > USB CDC 的波特率只是兼容传统串口工具，实际速率与波特率无关，
//! > 串口工具里选 115200 8N1 即可。
//!
//! ---
//!
//! # 4. PWM 定时参数
//!
//! `TOP = 19999`、`DIV = 150`，在 150 MHz 下：
//!
//! ```text
//!   f_PWM = 150 MHz / (150 × 20000) = 50 Hz    (周期 20 ms)
//!   每个计数 = 1 µs
//! ```
//!
//! | 角度  | 脉宽 |
//! |-------|------|
//! | 0.0°  | 0.5 ms |
//! | 90.0° | 1.5 ms |
//! | 180.0°| 2.5 ms |
//!
//! 协议单位是 0.1°，而 PWM 侧为 0.09°/计数，相邻 0.1° 有时会落到
//! 同一个计数；实际可达分辨率还受舵机死区限制（模拟舵机约 0.45~0.9°）。
//!
//! > 若舵机只支持 1.0~2.0 ms，修改 [`SERVO_MIN_PULSE_US`] / [`SERVO_MAX_PULSE_US`]。
//!
//! ---
//!
//! # 5. 接线
//!
//! ```text
//!   RP2350 (Pico 2)                  舵机 0 / 舵机 1
//!   ┌──────────────┐                ┌──────────────────┐
//!   │   GP0  ──────────────────────── 舵机 0 信号线    │
//!   │   GP1  ──────────────────────── 舵机 1 信号线    │
//!   │   GND  ──────────────────────── 两个舵机的地线   │
//!   │              │                │                  │
//!   │              │     外部 5V ──── 两个舵机电源线   │
//!   └──────────────┘                └──────────────────┘
//!          GND ─────────────────────── 外部 5V 地 (必须共地)
//! ```
//!
//! 两个舵机同时动作电流更大，务必使用独立 5V 电源，并在舵机电源附近
//! 并联 470~1000 µF 电解电容。
//!
//! ---
//!
//! # 6. 编译运行
//!
//! ```bash
//! cargo run --bin usb_servo_irq --target thumbv8m.main-none-eabihf
//! ```
//!
//! ```bash
//! screen /dev/ttyACM0 115200          # 输入: 90.5 45.0
//! printf '0.0 180.0\n' > /dev/ttyACM0
//! ```

#![no_std]
#![no_main]

use core::fmt::Write as _;

#[cfg(feature = "defmt")]
use defmt_rtt as _;
use rp2350_examples::log::*;

use rp2350::Steal;
use rp2350::bind;
use rp2350::clocks::ClockBuilder;
use rp2350::gpio::Pins;
use rp2350::interrupt;
use rp2350::main;
use rp2350::pac::{Interrupt, Peripherals};
use rp2350::pwm::{Pwm, Slice};

use rp2350_usb::bus::UsbBusAllocator;
use rp2350_usb::class::UsbClass;
use rp2350_usb::class::cdc_acm::CdcAcm;
use rp2350_usb::device::{UsbDevice, UsbDeviceBuilder};
use rp2350_usb::error::UsbError;
use rp2350_usb::rp2350::Rp2350Bus;

// ============================================================================
// 配置参数
// ============================================================================

/// 舵机 PWM 频率：50 Hz（周期 20 ms）
const SERVO_FREQ_HZ: u32 = 50;

/// 周期 20 ms = 20000 个 1 µs 计数，TOP = 计数个数 - 1
const SERVO_WRAP: u16 = 19_999;

/// 0° 对应的脉宽（µs）
const SERVO_MIN_PULSE_US: u16 = 500;

/// 180° 对应的脉宽（µs）
const SERVO_MAX_PULSE_US: u16 = 2_500;

/// 满量程角度：1800 表示 180.0°（0.1° 定点）
const ANGLE_FULL_DECI: u16 = 1_800;

/// 上电初始角度：90.0°
const SERVO_START_ANGLE_DECI: u16 = 900;

/// 单条命令的最大长度，超过即返回 ERR
const LINE_MAX: usize = 32;

/// 待发送缓冲大小，必须能容纳最大的一次应答与上电提示
const TX_MAX: usize = 96;

/// 单次 USB 读取的缓冲大小（一个批量包）
const READ_MAX: usize = 64;

// ============================================================================
// 全局状态
// ============================================================================

// USB_ALLOC 赋予 UsbDevice/CdcAcm `'static` 生命周期。
// Safety：仅由 main 在使能 USBCTRL_IRQ 之前写入一次，之后只被中断读取。
static mut USB_ALLOC: Option<UsbBusAllocator<Rp2350Bus>> = None;

// 这两个句柄在 main 中写入，之后只在 USBCTRL_IRQ 中访问。
static mut USB_DEV: Option<UsbDevice<'static, Rp2350Bus>> = None;
static mut USB_CDC: Option<CdcAcm<'static, Rp2350Bus>> = None;

// 协议状态（行缓冲、待发缓冲）只在 USBCTRL_IRQ 中访问。
static mut CONSOLE: Console = Console::new();

/// 获取 USB 设备与 CDC 句柄的可变引用（仅限中断上下文调用）。
#[inline]
fn usb_handles() -> Option<(
    &'static mut UsbDevice<'static, Rp2350Bus>,
    &'static mut CdcAcm<'static, Rp2350Bus>,
)> {
    // Safety: 句柄在中断使能前初始化完成，此后仅由 USBCTRL_IRQ 访问。
    unsafe {
        let dev = core::ptr::addr_of_mut!(USB_DEV).as_mut()?.as_mut()?;
        let cdc = core::ptr::addr_of_mut!(USB_CDC).as_mut()?.as_mut()?;
        Some((dev, cdc))
    }
}

/// 获取协议状态的可变引用（仅限中断上下文调用）。
#[inline]
fn console() -> &'static mut Console {
    // Safety: 仅由 USBCTRL_IRQ 访问，且每次中断只取一次。
    unsafe { core::ptr::addr_of_mut!(CONSOLE).as_mut().unwrap_unchecked() }
}

/// 取回 slice 0 的寄存器访问权（中断里无需持有 main 的所有权）。
#[inline]
fn servo_slice() -> Steal<Slice<0>> {
    // Safety: PWM 已在 main 中完成初始化，中断只写 CC 寄存器。
    unsafe { Slice::<0>::steal() }
}

// ============================================================================
// 协议实现
// ============================================================================

/// 将 0.1° 定点角度 (0~1800) 线性映射为脉宽计数 (µs)。
const fn angle_deci_to_duty(angle_deci: u16) -> u16 {
    let span = (SERVO_MAX_PULSE_US - SERVO_MIN_PULSE_US) as u32;
    let scaled = span * angle_deci as u32 + ANGLE_FULL_DECI as u32 / 2;
    (SERVO_MIN_PULSE_US as u32 + scaled / ANGLE_FULL_DECI as u32) as u16
}

/// 解析单个角度，返回 0.1° 定点值。
///
/// 接受 `90`、`90.5`、`0.0` 这类写法，最多一位小数。
fn parse_angle_deci(token: &str) -> Option<u16> {
    let (whole_str, frac_str) = match token.split_once('.') {
        Some((whole, frac)) => (whole, Some(frac)),
        None => (token, None),
    };

    let whole: u32 = whole_str.parse().ok()?;
    let frac: u32 = match frac_str {
        None => 0,
        Some(frac) if frac.len() == 1 => frac.parse().ok()?,
        Some(_) => return None,
    };

    let deci = whole * 10 + frac;
    if deci > ANGLE_FULL_DECI as u32 {
        return None;
    }
    Some(deci as u16)
}

/// 解析 `"<角度0> <角度1>"`。
///
/// 分隔符可以是空格、制表符或逗号；只接受两个 0.0~180.0 的角度。
fn parse_angles(line: &[u8]) -> Option<(u16, u16)> {
    let text = core::str::from_utf8(line).ok()?;

    let mut parts = text
        .split(|c: char| c.is_ascii_whitespace() || c == ',')
        .filter(|part| !part.is_empty());

    let angle0 = parse_angle_deci(parts.next()?)?;
    let angle1 = parse_angle_deci(parts.next()?)?;

    if parts.next().is_some() {
        return None;
    }

    Some((angle0, angle1))
}

/// 栈上的定长字符串，用于格式化应答，无需 `alloc`。
struct FixedString<const N: usize> {
    buf: [u8; N],
    len: usize,
}

impl<const N: usize> FixedString<N> {
    const fn new() -> Self {
        Self {
            buf: [0; N],
            len: 0,
        }
    }

    fn as_bytes(&self) -> &[u8] {
        &self.buf[..self.len]
    }
}

impl<const N: usize> core::fmt::Write for FixedString<N> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bytes = s.as_bytes();
        let end = self.len + bytes.len();
        if end > N {
            return Err(core::fmt::Error);
        }
        self.buf[self.len..end].copy_from_slice(bytes);
        self.len = end;
        Ok(())
    }
}

/// USB 侧的全部可变状态。只在 USBCTRL_IRQ 中访问。
struct Console {
    line: [u8; LINE_MAX],
    line_len: usize,
    tx: [u8; TX_MAX],
    tx_len: usize,
    tx_pos: usize,
    port_was_open: bool,
    commands: u32,
    errors: u32,
}

impl Console {
    const fn new() -> Self {
        Self {
            line: [0; LINE_MAX],
            line_len: 0,
            tx: [0; TX_MAX],
            tx_len: 0,
            tx_pos: 0,
            port_was_open: false,
            commands: 0,
            errors: 0,
        }
    }

    fn reset(&mut self) {
        self.line_len = 0;
        self.tx_len = 0;
        self.tx_pos = 0;
    }

    /// 把数据追加到待发缓冲。缓冲不足时丢弃并记一次错误（主机没在读）。
    fn enqueue(&mut self, data: &[u8]) {
        if self.tx_pos >= self.tx_len {
            self.tx_pos = 0;
            self.tx_len = 0;
        }
        if self.tx_len + data.len() > TX_MAX {
            self.errors = self.errors.wrapping_add(1);
            return;
        }
        self.tx[self.tx_len..self.tx_len + data.len()].copy_from_slice(data);
        self.tx_len += data.len();
    }

    fn reply_ok(&mut self, angle0: u16, angle1: u16) {
        let mut reply = FixedString::<32>::new();
        let _ = core::write!(
            reply,
            "OK {}.{} {}.{}\r\n",
            angle0 / 10,
            angle0 % 10,
            angle1 / 10,
            angle1 % 10
        );
        self.enqueue(reply.as_bytes());
    }

    fn reply_err(&mut self) {
        self.errors = self.errors.wrapping_add(1);
        self.enqueue(b"ERR\r\n");
    }

    /// 读取所有已到达的字节，按行解析并更新舵机。
    fn drain_rx(&mut self, cdc: &CdcAcm<'static, Rp2350Bus>, servo: &Slice<0>) {
        let mut buf = [0u8; READ_MAX];

        loop {
            match cdc.read(&mut buf) {
                Ok(n) if n > 0 => {
                    for &byte in &buf[..n] {
                        self.feed(byte, servo);
                    }
                }
                _ => break,
            }
        }
    }

    /// 处理单个接收字节。
    fn feed(&mut self, byte: u8, servo: &Slice<0>) {
        match byte {
            b'\r' | b'\n' => {
                if self.line_len == 0 {
                    return;
                }
                let result = parse_angles(&self.line[..self.line_len]);
                self.line_len = 0;

                match result {
                    Some((angle0, angle1)) => {
                        servo.set_duty(angle_deci_to_duty(angle0), angle_deci_to_duty(angle1));
                        self.commands = self.commands.wrapping_add(1);
                        self.reply_ok(angle0, angle1);
                    }
                    None => self.reply_err(),
                }
            }
            _ => {
                if self.line_len < LINE_MAX {
                    self.line[self.line_len] = byte;
                    self.line_len += 1;
                } else {
                    self.line_len = 0;
                    self.reply_err();
                }
            }
        }
    }

    /// 尽可能把待发数据写出去；`WouldBlock` 时保留，等下次中断再试。
    fn flush(&mut self, cdc: &CdcAcm<'static, Rp2350Bus>) {
        while self.tx_pos < self.tx_len {
            match cdc.write(&self.tx[self.tx_pos..self.tx_len]) {
                Ok(0) => break,
                Ok(n) => self.tx_pos += n,
                Err(UsbError::WouldBlock) => break,
                Err(_) => {
                    self.tx_pos = 0;
                    self.tx_len = 0;
                    break;
                }
            }
        }

        if self.tx_pos >= self.tx_len {
            self.tx_pos = 0;
            self.tx_len = 0;
        }
    }
}

// ============================================================================
// USB 中断
// ============================================================================

#[bind(USBCTRL_IRQ)]
fn usbctrl_irq() {
    let Some((dev, cdc)) = usb_handles() else {
        return;
    };

    // 驱动 USB 状态机；BUFF_STATUS 中断保证 IN 完成也会回到这里。
    dev.poll(&mut [cdc as &mut dyn UsbClass<Rp2350Bus>]);

    let console = console();

    if !cdc.port_open() {
        if console.port_was_open {
            console.port_was_open = false;
            console.reset();
        }
        return;
    }

    if !console.port_was_open {
        console.port_was_open = true;
        console
            .enqueue(b"RP2350 dual servo ready (IRQ)\r\nsend: <a0> <a1>, 0.0..180.0 (0.1 deg)\r\n");
    }

    let servo = servo_slice();
    console.drain_rx(cdc, &servo);
    console.flush(cdc);
}

// ============================================================================
// main
// ============================================================================

#[main]
fn main() -> ! {
    info!("=== USB Dual Servo (IRQ driven), GPIO0/GPIO1 ===");

    let dp = Peripherals::take().unwrap();

    let clocks = ClockBuilder::new()
        .enable_usb()
        .build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    let sys_clk = clocks.sys_freq();

    let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);

    info!("Clocks: sys={} Hz, usb={} Hz", sys_clk, clocks.usb_freq());

    // ------------------------------------------------------------------
    // PWM：GPIO0/GPIO1 → slice 0 的 A/B 通道
    // ------------------------------------------------------------------
    let pwm = Pwm::init(dp.PWM);
    let mut servo = pwm.slice::<0>().output_ab(pins.pin::<0>(), pins.pin::<1>());

    let actual_hz = servo.configure_frequency_with_wrap(sys_clk, SERVO_FREQ_HZ, SERVO_WRAP);
    let (div_int, div_frac) = servo.div();

    // 上电先输出中位脉宽，再使能 PWM，避免舵机乱转
    let start_pulse = angle_deci_to_duty(SERVO_START_ANGLE_DECI);
    servo.set_duty(start_pulse, start_pulse);
    servo.enable();

    info!(
        "GPIO0/GPIO1: slice 0 A/B, TOP={}, DIV={}+{}/16, actual={} Hz",
        SERVO_WRAP, div_int, div_frac, actual_hz
    );

    // PWM 波形由硬件继续产生，`servo` 句柄留在 main 里即可；
    // 中断通过 Slice::<0>::steal() 取得独立的寄存器访问权。

    // ------------------------------------------------------------------
    // USB CDC：句柄写入 static，随后只在中断中使用
    // ------------------------------------------------------------------
    // Safety：中断尚未使能，此处无并发访问。
    let alloc: &'static UsbBusAllocator<Rp2350Bus> = unsafe {
        let slot = core::ptr::addr_of_mut!(USB_ALLOC)
            .as_mut()
            .unwrap_unchecked();
        *slot = Some(UsbBusAllocator::new(Rp2350Bus::steal()));
        slot.as_ref().unwrap_unchecked()
    };

    let cdc = CdcAcm::new(alloc);
    let usb_dev = UsbDeviceBuilder::new(alloc, 0x2E8A, 0x000A) // Raspberry Pi, Pico SDK CDC
        .device_release(0x0100)
        .manufacturer("Raspberry Pi")
        .product("RP2350 Servo Controller")
        .serial_number("000002")
        .composite_with_iads()
        .build();

    usb_dev.bus().connect();

    // Safety：仍在中断使能之前，写入后 main 不再访问这两个句柄。
    unsafe {
        core::ptr::addr_of_mut!(USB_DEV)
            .as_mut()
            .unwrap_unchecked()
            .replace(usb_dev);
        core::ptr::addr_of_mut!(USB_CDC)
            .as_mut()
            .unwrap_unchecked()
            .replace(cdc);
    }

    // USBCTRL_IRQ 保持默认最高优先级：命令处理路径短，无需让出。
    interrupt::unpend(Interrupt::USBCTRL_IRQ);
    unsafe { interrupt::enable(Interrupt::USBCTRL_IRQ) };

    info!("USB IRQ enabled, entering sleep loop");

    // PWM 由硬件维持，USB 与命令处理由中断完成，主循环无事可做。
    loop {
        rp2350::rt::asm::wfi();
    }
}
