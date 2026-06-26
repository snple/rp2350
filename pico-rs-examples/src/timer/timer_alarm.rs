//! # AlarmTimer Trait Example for RP2350
//!
//! 这个示例演示如何基于 [`pico_rs::timer::AlarmTimer`] 和
//! [`pico_rs::timer::MonotonicTimer`] 编写“与具体计时器类型解耦”的 alarm 调度逻辑。
//!
//! 这里刻意把职责拆开：
//!
//! - `MonotonicTimer` 用来读取当前时间，计算下一次触发的绝对时刻。
//! - `AlarmTimer` 用来设置 alarm、清中断、使能中断和强制中断。
//!
//! 这种拆分更贴近真实驱动场景：
//! 有些驱动只需要超时判断，有些只需要定时中断，而不是总想绑定整个 Timer API。
//!
//! 本示例展示：
//!
//! 1. 如何通过泛型函数 `schedule_next_alarm()` 统一计算并设置下一次 alarm。
//! 2. 如何通过泛型函数 `clear_alarm_sources()` 同时清理普通 alarm 中断和 forced interrupt。
//! 3. 如何在中断处理函数里继续通过 trait 进行重调度，而不是写死具体类型方法。
//!
//! 板载 LED 会在 TIMER0 alarm 中断中每 250ms 翻转一次。
//!
//! Run with: cargo run --release --bin timer_alarm

#![no_std]
#![no_main]

use core::cell::RefCell;
use core::sync::atomic::{AtomicBool, Ordering};

use defmt::*;
use defmt_rtt as _;
use pico_rs::block::ImageDef;
use pico_rs::clocks::{ClockBuilder, ClockResources};
use pico_rs::entry;
use pico_rs::gpio::Pins;
use pico_rs::interrupt::{self, Mutex};
use pico_rs::pac::{Interrupt, Peripherals};
use pico_rs::ticks::Ticks;
use pico_rs::timer::{AlarmTimer, MonotonicTimer, Timer0};

#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::exe();

static TIMER: Mutex<RefCell<Option<Timer0>>> = Mutex::new(RefCell::new(None));
static LED_STATE: AtomicBool = AtomicBool::new(false);
static TICK_COUNT: Mutex<RefCell<u32>> = Mutex::new(RefCell::new(0));

const ALARM_NUM: u8 = 0;
const PERIOD_US: u32 = 250_000;

/// 调度下一次 alarm。
///
/// 这个函数只要求：
/// - 能读取当前时间 (`MonotonicTimer`)
/// - 能设置带保护的 alarm (`AlarmTimer`)
///
/// 这样一来，调用方不需要关心底层究竟是 Timer0 还是 Timer1。
fn schedule_next_alarm<T>(timer: &T, alarm: u8, interval_us: u32)
where
    T: AlarmTimer + MonotonicTimer,
{
    let deadline = timer.get_time_us_32().wrapping_add(interval_us);
    timer.set_alarm_safe(alarm, deadline);
}

/// 清理与某个 alarm 相关的中断源。
///
/// `set_alarm_safe()` 在错过目标时间点时，可能会通过 forced interrupt 主动触发一次中断，
/// 因此 ISR 开头需要同时清普通 alarm 中断和 forced interrupt，避免重复进入。
fn clear_alarm_sources<T: AlarmTimer>(timer: &T, alarm: u8) {
    timer.clear_alarm_interrupt(alarm);
    timer.clear_forced_alarm_interrupt(alarm);
}

#[entry]
fn main() -> ! {
    info!("=== Timer Alarm Trait Example ===");

    // 获取芯片外设。
    let dp = Peripherals::take().unwrap();

    // 初始化时钟系统。
    // TIMER 的计数依赖 TICKS 提供的 1MHz tick，因此必须先完成时钟配置。
    let clocks = ClockBuilder::new().build(ClockResources { clocks: dp.CLOCKS, xosc: dp.XOSC, pll_sys: dp.PLL_SYS, pll_usb: dp.PLL_USB });

    // 让 TIMER0 开始以 1MHz 计数。
    let ticks = Ticks::new(dp.TICKS, &clocks);
    ticks.init_timer0();

    // 初始化板载 LED，用来观察 alarm 中断是否按预期发生。
    let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);
    let led = pins.pin::<25>();
    led.into_output();
    led.set_low();

    // 创建具体 Timer0 实例。
    // 但后面的调度逻辑和中断清理逻辑都通过 trait 函数来表达。
    let timer = Timer0::init(dp.TIMER0);

    // 设置第一次 alarm，并打开对应的中断使能。
    // 后续的每一次 alarm 都会在 ISR 内重新调度下一次。
    schedule_next_alarm(&timer, ALARM_NUM, PERIOD_US);
    timer.enable_alarm_interrupt(ALARM_NUM);

    // 把 timer 放到全局共享状态里，供中断上下文访问。
    // 使用 Mutex + RefCell 是这类 no_std 中断共享状态的常见写法。
    interrupt::free(|cs| {
        TIMER.borrow(cs).replace(Some(timer));
    });

    // 打开 NVIC 中对应的 TIMER0 alarm 0 中断线。
    unsafe {
        interrupt::enable(Interrupt::TIMER0_IRQ_0);
    }

    info!(
        "Timer0 alarm scheduled through AlarmTimer trait, period={} us",
        PERIOD_US
    );

    loop {
        // 主循环不需要主动轮询，只需等待中断。
        // LED 的翻转完全由 alarm ISR 驱动。
        pico_rs_rt::asm::wfi();
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
fn TIMER0_IRQ_0() {
    interrupt::free(|cs| {
        if let Some(ref timer) = *TIMER.borrow(cs).borrow() {
            // 先清中断源，再做其他事情。
            // 这是 timer alarm ISR 的一个重要习惯，能避免重复进入或状态残留。
            clear_alarm_sources(timer, ALARM_NUM);

            // 翻转一个软件态布尔值，并把它同步到真实 GPIO 输出。
            let new_state = !LED_STATE.load(Ordering::Relaxed);
            LED_STATE.store(new_state, Ordering::Relaxed);

            // 这里直接使用 PAC 的 SIO 寄存器做 GPIO 输出翻转，
            // 避免在中断里维护一个额外的 Pin 所有权结构。
            let sio = unsafe { pico_rs::pac::SIO::steal() };
            if new_state {
                sio.gpio_out_set.write_bits(1 << 25);
            } else {
                sio.gpio_out_clr.write_bits(1 << 25);
            }

            // 记录中断触发次数，方便通过日志确认周期行为。
            let mut tick_count = TICK_COUNT.borrow(cs).borrow_mut();
            *tick_count += 1;

            // 每 4 次打印一次，避免日志过于频繁。
            // 250ms * 4 = 1s，因此这条日志大约每秒输出一次。
            if (*tick_count).is_multiple_of(4) {
                info!(
                    "alarm tick={}, now={} us",
                    *tick_count,
                    timer.get_time_us_32()
                );
            }

            // 重新调度下一次触发。
            // 这一步同样是通过 trait 完成，因此逻辑并未绑定到 Timer0 的具体 API 形状。
            schedule_next_alarm(timer, ALARM_NUM, PERIOD_US);
        }
    });
}
