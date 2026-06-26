//! # MonotonicTimer Trait Example for RP2350
//!
//! 这个示例演示如何基于 [`pico_rs::timer::MonotonicTimer`] 编写“与具体硬件计时器类型解耦”的代码。
//!
//! 和直接把函数参数写成 `&Timer0` 相比，依赖 trait 有两个实际好处：
//!
//! 1. 业务逻辑只表达“我需要一个单调递增的时间源”，而不是“我必须使用 Timer0”。
//! 2. 未来如果改成 `Timer1`，或者其他实现了同样接口的时间源，上层逻辑无需修改。
//!
//! 本示例包含三部分：
//!
//! 1. 一个通用的 `wait_until()`，等待直到某个绝对时间点。
//! 2. 一个通用的 `wait_for()`，等待一段相对时间。
//! 3. 一个通用的 `elapsed_us()`，用于统计从起始时刻到现在经过了多久。
//!
//! 主循环中会每 500ms 翻转一次板载 LED，并每 2 秒打印一次累计运行时间，
//! 用来展示“泛型时间工具函数 + 具体硬件计时器”的组合方式。
//!
//! Run with: cargo run --release --bin timer_monotonic

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;

use pico_rs::block::ImageDef;
use pico_rs::clocks::{ClockBuilder};
use pico_rs::entry;
use pico_rs::gpio::Pins;
use pico_rs::pac::Peripherals;
use pico_rs::ticks::Ticks;
use pico_rs::timer::{MonotonicTimer, Timer0};

#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::exe();

const BLINK_INTERVAL_US: u64 = 500_000;
const REPORT_INTERVAL_US: u64 = 2_000_000;

/// 等待直到绝对时间 `deadline_us`。
///
/// 这里使用的是“绝对时间等待”而不是“循环减计数”，这样更适合写超时逻辑。
/// 当上层先计算好截止时间后，可以把它传给任意实现了 `MonotonicTimer` 的对象。
///
/// 通过 `wrapping_sub()` 做比较，可以在计时器最终发生回绕时仍保持正确的先后关系。
fn wait_until<T: MonotonicTimer>(timer: &T, deadline_us: u64) {
    while timer.get_time_us().wrapping_sub(deadline_us) >= (1u64 << 63) {
        pico_rs_rt::asm::nop();
    }
}

/// 等待相对时间 `duration_us`。
///
/// 这个函数展示了一个常见模式：
/// 先读取当前时间，再计算未来的 deadline，最后复用 `wait_until()`。
///
/// 这样上层代码既可以用“等待多久”的心智模型，也能复用统一的底层等待实现。
fn wait_for<T: MonotonicTimer>(timer: &T, duration_us: u64) {
    let deadline = timer.get_time_us().wrapping_add(duration_us);
    wait_until(timer, deadline);
}

/// 计算从 `start_us` 到当前时刻已经过去了多少微秒。
///
/// 这类函数在驱动里很常见，例如：
/// - 轮询等待某个位变为 ready
/// - 实现超时退出
/// - 统计某段初始化流程耗时
fn elapsed_us<T: MonotonicTimer>(timer: &T, start_us: u64) -> u64 {
    timer.get_time_us().wrapping_sub(start_us)
}

#[entry]
fn main() -> ! {
    info!("=== Timer Monotonic Trait Example ===");

    // 获取 PAC 外设。
    // 这里仍然是具体硬件初始化，但后面的“时间逻辑”会刻意只依赖 trait。
    let dp = Peripherals::take().unwrap();

    // 初始化时钟树。
    // TIMER 的 tick 源来自 TICKS 模块，因此时钟配置必须先完成。
    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    info!("Clock system initialized: sys={} Hz", clocks.sys_freq());

    // 初始化 TIMER0 的 1MHz tick。
    // RP2350 硬件 timer 以微秒为单位计数，所以非常适合做 timeout 和 profiling。
    let ticks = Ticks::new(dp.TICKS, &clocks);
    ticks.init_timer0();

    // 创建具体的 Timer0 实例。
    // 重要的是，下面的辅助函数并不依赖 Timer0 本身，而只依赖 MonotonicTimer。
    let timer = Timer0::init(dp.TIMER0);

    // 板载 LED（GPIO25）仅用于提供一个肉眼可见的周期行为。
    let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);
    let led = pins.pin::<25>();
    led.into_output();
    led.set_low();

    // 记录示例启动时间，用于后续打印累计运行时长。
    let start = timer.get_time_us();
    let mut last_report = start;
    let mut led_on = false;
    let mut blink_count = 0u32;

    info!("Timer0 is used through the MonotonicTimer trait");

    loop {
        // 翻转 LED，制造一个稳定的周期事件。
        led_on = !led_on;
        if led_on {
            led.set_high();
        } else {
            led.set_low();
        }

        blink_count += 1;

        // 等待 500ms。
        // 这里调用的是泛型辅助函数，它只要求 `timer` 实现 MonotonicTimer。
        wait_for(&timer, BLINK_INTERVAL_US);

        let now = timer.get_time_us();
        // 每 2 秒输出一次统计信息，展示如何基于 trait 读取当前时间和累计耗时。
        if now.wrapping_sub(last_report) >= REPORT_INTERVAL_US {
            info!(
                "elapsed={} us, blink_count={}, low32={} us",
                elapsed_us(&timer, start),
                blink_count,
                timer.get_time_us_32()
            );
            last_report = now;
        }
    }
}
