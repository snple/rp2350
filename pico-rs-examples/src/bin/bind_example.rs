//! 使用 #[bind] 宏的中断绑定示例
//!
//! 展示如何使用 #[bind] 宏简化中断处理器的定义。

#![no_std]
#![no_main]

use core::sync::atomic::{AtomicBool, Ordering};

use defmt::info;
use defmt_rtt as _;
use panic_probe as _;

use pico_rs::bind;
use pico_rs::block::ImageDef;
use pico_rs::entry;
use pico_rs::gpio::{self, Pin25};
use pico_rs::interrupt::{self, spare};
use pico_rs::task::{Event, EventQueue};

// Boot block
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::exe();

// ============================================================================
// 事件定义
// ============================================================================

const EVT_LED_ON: u16 = 1;
const EVT_LED_OFF: u16 = 2;

// ============================================================================
// 全局状态
// ============================================================================

static EVENTS: EventQueue = EventQueue::new();
static LED_STATE: AtomicBool = AtomicBool::new(false);

// ============================================================================
// 使用 #[bind] 宏定义中断处理器
// ============================================================================

/// 高优先级任务 - LED 开启
/// 使用 priority = 64 指定优先级（在中断内延迟设置）
#[bind(SPARE_IRQ_0, priority = 64)]
fn high_priority_task() {
    // spare::clear(0);
    interrupt::unpend(spare::SPARE_IRQ_0);
    info!("High priority task running!");
    EVENTS.push(Event::new(EVT_LED_ON, 0));
}

/// 低优先级任务 - LED 关闭
/// 不指定 priority，使用 spare::init() 设置
#[bind(SPARE_IRQ_1)]
fn low_priority_task() {
    interrupt::unpend(spare::SPARE_IRQ_1);
    info!("Low priority task running!");
    EVENTS.push(Event::new(EVT_LED_OFF, 0));
}

// ============================================================================
// 主程序
// ============================================================================

#[entry]
fn main() -> ! {
    info!("Bind macro example starting...");

    // 初始化 GPIO
    gpio::init_gpio();

    // 获取 LED 引脚 (GPIO 25)
    let led = Pin25::new();

    // 配置为输出
    led.into_output();

    // 初始化 spare IRQs
    unsafe {
        interrupt::enable(spare::SPARE_IRQ_0);
        interrupt::enable(spare::SPARE_IRQ_1);
    }

    let mut counter: u32 = 0;

    info!("Starting main loop...");

    loop {
        // 处理事件
        while let Some(event) = EVENTS.pop() {
            match event.id {
                EVT_LED_ON => {
                    led.set_high();
                    LED_STATE.store(true, Ordering::Relaxed);
                    info!("LED ON");
                }
                EVT_LED_OFF => {
                    led.set_low();
                    LED_STATE.store(false, Ordering::Relaxed);
                    info!("LED OFF");
                }
                _ => {}
            }
        }

        // 模拟周期性触发
        counter = counter.wrapping_add(1);

        if counter % 1_000_000 == 0 {
            // 触发高优先级任务
            interrupt::pend(spare::SPARE_IRQ_0);
        }

        if counter % 2_000_000 == 0 {
            // 触发低优先级任务
            interrupt::pend(spare::SPARE_IRQ_1);
        }

        // 短暂延迟
        pico_rs::rt::asm::nop();
    }
}
