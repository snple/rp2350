//! # ClockBuilder 时钟配置示例
//!
//! 这个示例演示如何使用 HAL 提供的高层接口 [`rp2350::clocks::ClockBuilder`]
//! 完成 RP2350 的时钟初始化，而不是手动一步步配置 XOSC、PLL_SYS、PLL_USB
//! 以及各个下游时钟。
//!
//! 目标是把“常见开发场景下真正关心的东西”表达清楚：
//!
//! 1. 系统主频要多少，例如 150 MHz。
//! 2. USB 是否要启用，如果要，则必须提供 48 MHz。
//! 3. ADC 是否要启用，如果要，也需要 48 MHz。
//! 4. 外设时钟是否跟随系统时钟，还是固定成某个稳定频率。
//!
//! 对于绝大多数应用，并不需要亲自管理完整的时钟树切换过程。
//! `ClockBuilder` 已经把推荐初始化流程封装好了：
//!
//! - 启动 XOSC
//! - 配置 `clk_ref`
//! - 临时把 `clk_sys` 切到安全源
//! - 配置 PLL
//! - 切换到目标系统时钟
//! - 配置 `clk_peri` / `clk_usb` / `clk_adc`
//!
//! 这个示例选择的配置为：
//!
//! - `clk_sys = 150 MHz`
//! - `clk_peri = clk_sys = 150 MHz`
//! - `clk_usb = 48 MHz`
//! - `clk_adc = 48 MHz`
//!
//! 对应的时钟关系可以简单理解为：
//!
//! ```text
//! XOSC (12 MHz)
//!   ├─> PLL_SYS -> clk_sys  = 150 MHz
//!   └─> PLL_USB -> clk_usb  =  48 MHz
//!                  └──────> clk_adc  =  48 MHz
//! ```
//!
//! 示例最后会使用新的系统时钟频率创建 `SysTick` 延时器，并以 1 Hz 频率闪烁板载 LED，
//! 用来验证时钟配置已正常生效。
//!
//! Run with: cargo run --release --bin clocks

#![no_std]
#![no_main]

#[cfg(feature = "defmt")]
use defmt_rtt as _;
use rp2350_examples::log::*;

use rp2350::clocks::ClockBuilder;
use rp2350::gpio::Pins;
use rp2350::main;
use rp2350::pac::Peripherals;

///
/// RP2350 在本 HAL 中的常用高性能配置就是 150 MHz。
const SYS_CLK_HZ: u32 = 150_000_000;

#[main]
fn main() -> ! {
    info!("=== RP2350 ClockBuilder Example ===");
    info!("Target system clock: {} MHz", SYS_CLK_HZ / 1_000_000);

    // 取得芯片外设所有权。
    // 时钟配置属于最早期初始化步骤之一，因此通常一进入 main 就会完成。
    let dp = Peripherals::take().unwrap();

    // 使用 HAL 的高层 ClockBuilder 完成整套时钟初始化。
    //
    // 这里显式写出 `.sys_freq_mhz(150)`、`.enable_usb()`、`.enable_adc()`，
    // 目的是让示例读者一眼就能看出“这个应用真正关心的配置项”是什么。
    //
    // 这比手动调用 XOSC / PLL / clk_ref / clk_sys / clk_usb / clk_adc 的底层方法
    // 更适合作为日常应用模板。
    let clocks = ClockBuilder::new()
        .sys_freq_mhz(150)
        .enable_usb()
        .enable_adc()
        .build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);

    info!("Clock configuration complete");
    info!("  clk_ref : {} Hz", clocks.ref_freq());
    info!("  clk_sys : {} Hz", clocks.sys_freq());
    info!("  clk_peri: {} Hz", clocks.peri_freq());
    info!("  clk_usb : {} Hz", clocks.usb_freq());
    info!("  clk_adc : {} Hz", clocks.adc_freq());

    if clocks.sys_freq() != SYS_CLK_HZ {
        warn!(
            "System clock {} Hz differs from requested {} Hz",
            clocks.sys_freq(),
            SYS_CLK_HZ
        );
    }

    // 下面进入“结果验证”阶段。
    // 我们不再继续展开时钟树细节，而是直接使用新的系统频率创建 Delay，
    // 然后闪烁板载 LED，验证 CPU 和 SysTick 都在预期频率下工作。
    let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);
    let mut led = pins.pin::<25>();
    led.into_output();

    // Delay 依赖“当前真实的系统主频”。
    // 这里使用 clocks.sys_freq() 而不是硬编码常量，避免以后修改 builder 配置后忘记同步。
    let delay = rp2350_examples::common::board::init_delay(&clocks);
    info!("Delay initialized with {} Hz", clocks.sys_freq());
    info!(
        "SysTick ticks_per_us: {} (expected {})",
        delay.systick().ticks_per_us(),
        clocks.sys_freq() / 1_000_000
    );

    let mut blink_count = 0u32;
    loop {
        // LED 亮 500ms
        led.set_high();
        delay.delay_ms(500);

        // LED 灭 500ms
        led.set_low();
        delay.delay_ms(500);

        blink_count += 1;
        if blink_count.is_multiple_of(5) {
            info!("Blink #{}", blink_count);
        }
    }
}
