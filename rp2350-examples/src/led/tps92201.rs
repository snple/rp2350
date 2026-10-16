//! # TPS92201 恒流 LED 驱动器调光示例
//!
//! TPS92201 是 TI 的同步 Buck LED 驱动器：输入 2.5~5.5V，最大输出 1.5A，
//! 内部开关频率 1.5MHz，6 脚封装（SOT-563 / WSON）。器件**没有任何数字接口**，
//! 亮度只由一路 PWM 信号控制 —— 芯片内部把 PWM 滤成模拟基准，占空比
//! 1%~100% 线性对应 LED 电流的 1%~100%（数据手册称之为 analog dimming）。
//!
//! 所以 RP2350 侧要做的事只有两件：把 PWM 配到数据手册要求的 20kHz~200kHz，
//! 然后改占空比。本示例把这两件事展开，并演示实际会遇到的几个取舍。
//!
//! ---
//!
//! # 1. 器件要点
//!
//! | 项目 | 说明 |
//! |------|------|
//! | 输入电压 | 2.5~5.5V |
//! | 输出电流 | 最大 1.5A，由采样电阻设定：`I_LED = 100mV / R_FB` |
//! | 拓扑 | 同步 Buck，LED 串总压降必须低于 VIN |
//! | 调光 | 调光脚上的 PWM 占空比，1%~100%，输入频率 20kHz~200kHz |
//! | 使能 | EN 脚，高电平使能、低电平关断（关断电流典型 0.1µA） |
//! | 型号差异 | TPS92201 强制 PWM 模式（输出纹波小）；TPS92201A 轻载进省电模式（效率高） |
//! | 保护 | 逐周期限流、LED 开路/短路、FB 电阻开路/短路、过温、UVLO，全部内部自保护 |
//!
//! 芯片只有 VIN / GND / SW / BST / FB / EN / 调光脚这几个功能引脚，具体引脚名与
//! 编号以数据手册（SLVSH40B 第 5 节）和你的原理图为准，不同封装的丝印并不一样。
//!
//! 一个容易踩的点：器件**没有故障上报脚**。LED 开路/短路、过温都只是内部打嗝或
//! 关断，主控看不到。想感知故障，只能自己在 LED 电流回路或 VOUT 上取样接 ADC 判断。
//!
//! ---
//!
//! # 2. 接线
//!
//! ```text
//!   RP2350 板                       TPS92201 外围
//!   ───────────────                 ──────────────
//!   GP22 (PWM) ───────────────────► 调光输入
//!   GP23 (EN)  ───────────────────► EN（高电平使能）
//!   GND ────────────────────────── 共地（必须与 VIN 的源共地）
//!
//!   VIN ── 5V（VBUS 或单节锂电），就近 10µF 陶瓷电容到 GND
//!   BST ── 100nF ── SW              自举电容跨在 BST 与 SW 之间
//!   SW  ── L(约 2.2µH) ── LED 阳极
//!   LED 阴极 ── FB                  采样电阻接在 FB 与 GND 之间
//!   R_FB: FB ── GND
//! ```
//!
//! R_FB 取值：0.2Ω → 500mA，0.1Ω → 1A。它是电流采样电阻，功耗 `I² · R_FB`
//! （1A 时 0.1W），选 1% 精度的 0805/1206 金属膜电阻。
//!
//! 逻辑电平方面，调光输入和 EN 都用 3.3V 推挽输出直接驱动即可；如果板子在这两根
//! 线上有上拉到 VIN 的电阻，GPIO 侧串 1kΩ 再接，别让 5V 通过强上拉灌进引脚。
//! 另外 GP23 在 Pico 2 的排针上没有引出，本示例按自制板/模组的引脚来配。
//!
//! ---
//!
//! # 3. 调光频率与占空比分辨率
//!
//! 频率由 `f = 150MHz / (DIV × (TOP+1))` 决定（普通边沿对齐模式）。频率越高，
//! 内部滤波后的基准纹波越小、调光越干净；但同样的系统时钟下 TOP 只能更小，
//! 占空比台阶就更粗。150MHz 下的几组可行组合：
//!
//! | 调光频率 | DIV | TOP | 占空比步数 | 说明 |
//! |---------|-----|-----|-----------|------|
//! | 20 kHz | 7+8/16 | 999 | 1000 | 数据手册下限，基准纹波最大 |
//! | 50 kHz | 3 | 999 | 1000 | |
//! | 100 kHz | 1+8/16 | 999 | 1000 | 本示例默认，一档 0.1% |
//! | 200 kHz | 1 | 749 | 750 | 数据手册上限，分辨率降到 0.13% |
//!
//! 两个边界值要记住：占空比低于 1% 时器件只保证线性到 1%；要彻底没有电流、
//! 并把静态功耗压到 0.1µA 级，得拉低 EN，而不是靠 0.5% 这种小占空比。而要输出
//! 真正的 100%，compare 值要写成 `TOP + 1`（1000），比 TOP 大 1 是正常的，
//! 此时计数器永远达不到 compare，输出恒高。
//!
//! ---
//!
//! # 4. 示例内容
//!
//! 1. 使能 / 关断 / 满亮：EN 拉低关断、拉高使能，再对比 0% 与 100% 占空比
//! 2. 粗扫：1%~100% 每 1% 一档，每档 20ms
//! 3. 细扫：0~999 逐档呼吸（1000 档，0.1% 步进），三上三下
//! 4. 频率对比：同一个 20% 占空比分别在 20/50/200kHz 下输出，观察亮度与纹波
//! 5. 回到 100kHz 配置，进入无限慢速呼吸（5ms 一档，单程约 5 秒）
//!
//! 模拟调光的好处是 LED 电流本身连续，没有斩波调光的频闪和低亮度色偏，
//! 亮度斜坡直接按占空比线性走即可，不需要 gamma 曲线。
//!
//! ---
//!
//! # 5. 编译运行
//!
//! ```bash
//! cargo run --bin tps92201 --target thumbv8m.main-none-eabihf
//! ```
//!
//! defmt 日志走 RTT，用 `probe-rs run` / `probe-rs attach` 查看。

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;

use rp2350::Peripherals;
use rp2350::clocks::ClockBuilder;
use rp2350::gpio::Pins;
use rp2350::main;
use rp2350::pwm::Pwm;
use rp2350_examples::common::board::init_delay;

// ============================================================================
// 配置参数
// ============================================================================

/// 系统时钟频率（`ClockBuilder` 默认 PLL 配置）150 MHz
const SYS_CLK_HZ: u32 = 150_000_000;

/// 调光 PWM 输出：GP22 → slice 3 通道 A（slice = (gpio / 2) % 8）
const DIM_PIN: u32 = 22;
const DIM_SLICE: u8 = 3;

/// 使能脚：GP23，高电平使能、低电平关断（典型 0.1µA）
const EN_PIN: u32 = 23;

/// 调光 PWM：100 kHz / 1000 档，算式见文件头第 3 节
const DIM_FREQ_HZ: u32 = 100_000;
const DIM_WRAP: u16 = 999;
const DIM_DIV_INT: u8 = 1;
const DIM_DIV_FRAC: u8 = 8;

/// 器件保证的调光下限（%）
const DIM_MIN_PERCENT: u32 = 1;

/// 粗扫每档停留时间
const SWEEP_STEP_MS: u32 = 20;

/// 细扫（呼吸）每档停留时间
const BREATH_STEP_US: u32 = 1000;

/// 末尾无限呼吸的每档停留时间。比 Demo 3 慢得多：1000 档 × 5ms → 单程约 5 秒、
/// 一上一下 10 秒。想更慢就加大这个值。
const BREATH_SLOW_STEP_US: u32 = 5_000;

/// 占空比百分比 → compare 值。
///
/// 基数取 `TOP + 1`，所以 100% 得到 `1000`（比 TOP 大 1），这正是 100% 输出
/// 需要的写法；其余档位就是 `(TOP + 1) × percent / 100`。
const fn duty_from_percent(wrap: u16, percent: u32) -> u16 {
    (((wrap as u32 + 1) * percent) / 100) as u16
}

#[main]
fn main() -> ! {
    info!("=== TPS92201 LED Dimming Example ===");

    // ------------------------------------------------------------------
    // 1. 初始化
    // ------------------------------------------------------------------
    let dp = Peripherals::take().unwrap();
    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);
    let delay = init_delay(&clocks);

    info!("System clock: {} Hz", clocks.sys_freq());

    // ------------------------------------------------------------------
    // 2. 调光 PWM — 手动写 DIV/TOP/CC，数值都在文件头列过
    // ------------------------------------------------------------------
    let pwm = Pwm::init(dp.PWM);
    let mut dim = pwm.slice::<DIM_SLICE>().output_a(pins.pin::<DIM_PIN>());

    dim.set_div(DIM_DIV_INT, DIM_DIV_FRAC);
    dim.set_wrap(DIM_WRAP);
    dim.set_duty_a(0); // 先写 0%，等 EN 抬起后再加亮，免得使能瞬间闪一下
    dim.enable();

    info!(
        "Dim PWM: GP{} TOP={} DIV={}+{}/16 → {} Hz (target {} Hz), {} duty steps",
        DIM_PIN,
        DIM_WRAP,
        DIM_DIV_INT,
        DIM_DIV_FRAC,
        dim.frequency(SYS_CLK_HZ),
        DIM_FREQ_HZ,
        DIM_WRAP as u32 + 1
    );

    // ------------------------------------------------------------------
    // 3. 使能脚 — 高电平使能，低电平关断
    // ------------------------------------------------------------------
    let mut en = pins.pin::<EN_PIN>();
    en.into_output();
    en.set_low();
    info!("EN: GP{} (low = shutdown, high = enabled)", EN_PIN);

    // ==================================================================
    // Demo 1: 关断 / 使能 / 满亮
    // ==================================================================
    info!("--- Demo 1: EN shutdown, then 0% vs 100% duty ---");

    info!("EN low → shutdown (typ. 0.1uA)");
    en.set_low();
    delay.delay_ms(2000);

    info!("EN high + 0% duty → enabled but no current");
    en.set_high();
    dim.set_duty_a(0);
    delay.delay_ms(2000);

    info!("100% duty → full current, I_LED = 100mV / R_FB");
    dim.set_duty_a(duty_from_percent(DIM_WRAP, 100));
    info!(
        "Compare value {} (= TOP + 1, output stays high)",
        duty_from_percent(DIM_WRAP, 100)
    );
    delay.delay_ms(2000);

    // ==================================================================
    // Demo 2: 粗扫 1%~100%（每 1% 一档）
    // ==================================================================
    info!(
        "--- Demo 2: 1%..100% in 1% steps ({} ms each) ---",
        SWEEP_STEP_MS
    );

    for percent in DIM_MIN_PERCENT..=100u32 {
        dim.set_duty_a(duty_from_percent(DIM_WRAP, percent));
        if percent % 10 == 0 {
            info!(
                "  {}% (cc={})",
                percent,
                duty_from_percent(DIM_WRAP, percent)
            );
        }
        delay.delay_ms(SWEEP_STEP_MS);
    }

    // ==================================================================
    // Demo 3: 细扫 — 1000 档呼吸（0.1% 步进）
    // ==================================================================
    info!(
        "--- Demo 3: 1000-step breathing ({} us per step) ---",
        BREATH_STEP_US
    );

    for round in 0..3u32 {
        info!("  Round {}: fade in / fade out", round);

        for cc in 0..=DIM_WRAP {
            dim.set_duty_a(cc);
            delay.delay_us(BREATH_STEP_US);
        }

        for cc in (0..=DIM_WRAP).rev() {
            dim.set_duty_a(cc);
            delay.delay_us(BREATH_STEP_US);
        }
    }

    // ==================================================================
    // Demo 4: 同一个 20% 占空比，跑在三个调光频率上
    //   改 DIV/TOP 前先 disable，避免运行中换配置产生半个周期的怪波形。
    //   三组参数都能整除出目标频率，实际频率应打印得与目标一致。
    // ==================================================================
    info!("--- Demo 4: same 20% duty at 20/50/200 kHz ---");

    let freq_table: [(u32, u16, u8, u8); 3] = [
        (20_000, 999, 7, 8),  // 7 + 8/16 = 7.5
        (50_000, 999, 3, 0),  // 3
        (200_000, 749, 1, 0), // 1，TOP 只能取 749 → 750 档
    ];

    for &(target_hz, wrap, div_int, div_frac) in freq_table.iter() {
        dim.disable();
        dim.set_div(div_int, div_frac);
        dim.set_wrap(wrap);
        dim.set_duty_a(duty_from_percent(wrap, 20));
        dim.enable();

        info!(
            "  target {} Hz: TOP={} DIV={}+{}/16 → actual {} Hz, 20% (cc={}), {} steps",
            target_hz,
            wrap,
            div_int,
            div_frac,
            dim.frequency(SYS_CLK_HZ),
            duty_from_percent(wrap, 20),
            wrap as u32 + 1
        );

        delay.delay_ms(3000);
    }

    // ==================================================================
    // Demo 5: 恢复默认配置，无限慢速呼吸
    // ==================================================================
    info!(
        "--- Demo 5: back to 100 kHz, slow breathing forever ({} ms per step) ---",
        BREATH_SLOW_STEP_US / 1000
    );

    dim.disable();
    dim.set_div(DIM_DIV_INT, DIM_DIV_FRAC);
    dim.set_wrap(DIM_WRAP);
    dim.enable();

    loop {
        for cc in 0..=DIM_WRAP {
            dim.set_duty_a(cc);
            delay.delay_us(BREATH_SLOW_STEP_US);
        }

        for cc in (0..=DIM_WRAP).rev() {
            dim.set_duty_a(cc);
            delay.delay_us(BREATH_SLOW_STEP_US);
        }
    }
}
