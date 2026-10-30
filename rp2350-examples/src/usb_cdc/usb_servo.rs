//! # USB 双舵机控制示例 — GPIO0 / GPIO1
//!
//! 通过 USB CDC (虚拟串口) 接收简单的 ASCII 命令，同时控制两个舵机。
//! 上位机只需要发送一行文本，无需自定义二进制协议或驱动。
//!
//! > 面向使用者的完整接口文档（接线、上位机准备、协议、示例、排错）见
//! > [`docs/usb_servo_protocol.md`](../../../docs/usb_servo_protocol.md)。
//!
//! ---
//!
//! # 1. 硬件映射
//!
//! 两个舵机接在同一个 PWM slice 的两个通道上：
//!
//! ```text
//!   GPIO0  →  slice 0, channel A  →  舵机 0
//!   GPIO1  →  slice 0, channel B  →  舵机 1
//! ```
//!
//! 同一 slice 共享计数器、分频器和 TOP 值，因此两个舵机**频率相同**，
//! 只有占空比 (脉宽) 独立。写入时用 `set_duty()` 一次更新两个通道，
//! 两个舵机会在同一 PWM 周期内同时生效。
//!
//! ---
//!
//! # 2. 通讯协议
//!
//! 刻意做成最简单的**按行 ASCII 文本协议**：
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
//! - 行结束符支持 `\n`、`\r` 或 `\r\n`
//! - 空行会被忽略
//! - 角度范围 `0.0 ~ 180.0`，可写整数（`90`）或一位小数（`90.5`）
//! - 小数超过一位或超出范围时返回 `ERR`
//!
//! 示例：
//!
//! ```text
//!   90 90        → 两个舵机都转到中位
//!   0.0,180.0    → 舵机 0 转到 0.0°，舵机 1 转到 180.0°
//!   45.5 135.0   → OK 45.5 135.0
//!   90.25 0      → ERR（小数超过一位）
//! ```
//!
//! > USB CDC 的波特率设置只是兼容传统串口工具，实际传输速率与波特率无关，
//! > 在串口工具中选择 115200 8N1 即可。
//!
//! ---
//!
//! # 3. PWM 定时参数
//!
//! 与单舵机示例相同，取 `TOP = 19999`、`DIV = 150`，在 150 MHz 下：
//!
//! ```text
//!   f_PWM = 150 MHz / (150 × 20000) = 50 Hz    (周期 20 ms)
//!   每个计数 = 1 µs
//! ```
//!
//! 脉宽与角度的关系（角度以 0.1° 定点表示，1800 = 180.0°）：
//!
//! ```text
//!   pulse = 500 µs + angle_deci / 1800 × (2500 µs - 500 µs)
//! ```
//!
//! | 角度  | 脉宽 |
//! |-------|------|
//! | 0.0°  | 0.5 ms |
//! | 90.0° | 1.5 ms |
//! | 180.0°| 2.5 ms |
//!
//! ## 分辨率说明
//!
//! 当前配置每个计数为 1 µs，整个行程有 2000 个计数，即 PWM 侧约
//! `180° / 2000 = 0.09°/计数`。因此 0.1° 命令是最细的**协议**单位，
//! 相邻的两个 0.1° 有时会四舍五入到同一个计数。
//!
//! 实际可达分辨率还受舵机本身限制（模拟舵机典型死区 5~10 µs，
//! 约 0.45~0.9°）。若需要更高 PWM 分辨率，可把 `TOP` 提到 65535。
//!
//! > 若舵机只支持 1.0~2.0 ms，修改 [`SERVO_MIN_PULSE_US`] / [`SERVO_MAX_PULSE_US`]。
//!
//! ---
//!
//! # 4. 接线
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
//! **两个舵机同时动作时电流更大**，务必使用独立 5V 电源：
//!
//! - 不要从 Pico 的 5V/VBUS 给舵机供电
//! - 外部电源的地必须与 Pico 的 GND 相连
//! - 在舵机电源附近并联 470~1000 µF 电解电容
//!
//! ---
//!
//! # 5. 编译运行
//!
//! ```bash
//! cargo run --bin usb_servo --target thumbv8m.main-none-eabihf
//! ```
//!
//! 设备会被识别为 `/dev/ttyACM0`，用串口工具连接后输入命令：
//!
//! ```bash
//! screen /dev/ttyACM0 115200    # 输入: 90.5 45.0
//! printf '0.0 180.0\n' > /dev/ttyACM0
//! ```

#![no_std]
#![no_main]

use core::fmt::Write as _;

use defmt::*;
use defmt_rtt as _;

use rp2350::SysTick;
use rp2350::clocks::ClockBuilder;
use rp2350::gpio::Pins;
use rp2350::main;
use rp2350::pac::Peripherals;
use rp2350::pwm::Pwm;
use rp2350::rt::peripheral::Peripherals as CorePeripherals;

use rp2350_usb::bus::UsbBusAllocator;
use rp2350_usb::class::UsbClass;
use rp2350_usb::class::cdc_acm::CdcAcm;
use rp2350_usb::device::{UsbDevice, UsbDeviceBuilder};
use rp2350_usb::error::UsbError;
use rp2350_usb::rp2350::Rp2350Bus;

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

/// 发送一段数据；遇到 `WouldBlock` 时继续轮询 USB，最多重试有限次数。
fn send(usb_dev: &mut UsbDevice<'_, Rp2350Bus>, cdc: &mut CdcAcm<'_, Rp2350Bus>, data: &[u8]) {
    let mut sent = 0;
    let mut retries = 0;

    while sent < data.len() {
        match cdc.write(&data[sent..]) {
            Ok(0) => return,
            Ok(n) => sent += n,
            Err(UsbError::WouldBlock) if retries < 10_000 => {
                retries += 1;
                usb_dev.poll(&mut [cdc as &mut dyn UsbClass<Rp2350Bus>]);
            }
            Err(_) => return,
        }
    }
}

#[main]
fn main() -> ! {
    info!("=== USB Dual Servo Example (GPIO0/GPIO1) ===");

    // ------------------------------------------------------------------
    // 1. 时钟、GPIO、SysTick
    // ------------------------------------------------------------------
    let dp = Peripherals::take().unwrap();
    let cp = CorePeripherals::take().unwrap();

    let clocks = ClockBuilder::new()
        .enable_usb()
        .build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    let sys_clk = clocks.sys_freq();

    let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);
    let systick = SysTick::init(cp.SYST, sys_clk);

    info!("Clocks: sys={} Hz, usb={} Hz", sys_clk, clocks.usb_freq());

    // ------------------------------------------------------------------
    // 2. PWM：GPIO0/GPIO1 → slice 0 的 A/B 通道
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

    // ------------------------------------------------------------------
    // 3. USB CDC 虚拟串口
    // ------------------------------------------------------------------
    let alloc = UsbBusAllocator::new(unsafe { Rp2350Bus::steal() });
    let mut cdc = CdcAcm::new(&alloc);

    let mut usb_dev = UsbDeviceBuilder::new(&alloc, 0x2E8A, 0x000A) // Raspberry Pi, Pico SDK CDC
        .device_release(0x0100)
        .manufacturer("Raspberry Pi")
        .product("RP2350 Servo Controller")
        .serial_number("000001")
        .composite_with_iads()
        .build();

    usb_dev.bus().connect();
    info!("USB connected, waiting for host");

    // ------------------------------------------------------------------
    // 4. 主循环：轮询 USB，解析命令行
    // ------------------------------------------------------------------
    let mut read_buf = [0u8; 64];
    let mut line = [0u8; LINE_MAX];
    let mut line_len = 0usize;
    let mut was_open = false;

    loop {
        // USB 状态机必须至少每 10 ms 调用一次
        usb_dev.poll(&mut [&mut cdc as &mut dyn UsbClass<Rp2350Bus>]);

        if cdc.port_open() {
            if !was_open {
                was_open = true;
                info!("Host opened the port");
                send(
                    &mut usb_dev,
                    &mut cdc,
                    b"RP2350 dual servo ready\r\nsend: <angle0> <angle1>, 0.0..180.0 (0.1 deg)\r\n",
                );
            }

            loop {
                match cdc.read(&mut read_buf) {
                    Ok(n) if n > 0 => {
                        for &byte in &read_buf[..n] {
                            match byte {
                                b'\r' | b'\n' => {
                                    if line_len == 0 {
                                        continue;
                                    }

                                    if let Some((angle0, angle1)) = parse_angles(&line[..line_len])
                                    {
                                        servo.set_duty(
                                            angle_deci_to_duty(angle0),
                                            angle_deci_to_duty(angle1),
                                        );

                                        let mut reply = FixedString::<32>::new();
                                        let _ = core::write!(
                                            reply,
                                            "OK {}.{} {}.{}\r\n",
                                            angle0 / 10,
                                            angle0 % 10,
                                            angle1 / 10,
                                            angle1 % 10
                                        );
                                        send(&mut usb_dev, &mut cdc, reply.as_bytes());

                                        info!(
                                            "servo0={}.{} deg, servo1={}.{} deg",
                                            angle0 / 10,
                                            angle0 % 10,
                                            angle1 / 10,
                                            angle1 % 10
                                        );
                                    } else {
                                        send(&mut usb_dev, &mut cdc, b"ERR\r\n");
                                        warn!("invalid command ({} bytes)", line_len);
                                    }

                                    line_len = 0;
                                }
                                _ => {
                                    if line_len < LINE_MAX {
                                        line[line_len] = byte;
                                        line_len += 1;
                                    } else {
                                        line_len = 0;
                                        send(&mut usb_dev, &mut cdc, b"ERR\r\n");
                                        warn!("command too long");
                                    }
                                }
                            }
                        }
                    }
                    _ => break,
                }
            }
        } else {
            was_open = false;
            line_len = 0;
        }

        systick.delay_us(10);
    }
}
