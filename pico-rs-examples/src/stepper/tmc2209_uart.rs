//! # TMC2209 UART 通讯测试 — 4 片芯片扫描
//!
//! 使用 UART1 (GPIO4=TX, GPIO5=RX)，轮询 4 个从机地址 (0x00~0x03)，
//! 读取 IOIN 寄存器确认芯片版本号 (应为 0x21)。
//!
//! ---
//!
//! # 1. 硬件接线
//!
//! ## 1.1 UART 单线半双工拓扑 (4 片 TMC2209 共享同一 UART 总线)
//!
//! ```text
//!   Pico 2 (RP2350)
//!   ──────────────
//!   GP4 (UART1_TX) ──/\/\/──┐
//!                    1kΩ     │
//!   GP5 (UART1_RX) ──/\/\/──┤
//!                    1kΩ     │
//!                            ├──────── 所有 TMC2209 的 PDN_UART
//!   3.3V (VCC_IO) ───/\/\/──┘
//!                    1kΩ
//!    (上拉电阻 — UART 模式必须!)
//! ```
//!
//! ## 1.2 TMC2209 地址设置 (MS1/MS2)
//!
//! | 芯片编号 | MS1  | MS2  | UART 地址 |
//! |----------|------|------|-----------|
//! | Chip 0   | GND  | GND  | 0x00      |
//! | Chip 1   | VCC  | GND  | 0x01      |
//! | Chip 2   | GND  | VCC  | 0x02      |
//! | Chip 3   | VCC  | VCC  | 0x03      |
//!
//! ---
//!
//! # 2. 编译运行
//!
//! ```bash
//! cd pico-rs-examples
//! cargo run --bin tmc2209_uart --target thumbv8m.main-none-eabihf
//! ```

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;

use pico_rs::block::ImageDef;
use pico_rs::clocks::ClockBuilder;
use pico_rs::entry;
use pico_rs::gpio::{Pins, Uart1RxPin, Uart1TxPin};
use pico_rs::pac::Peripherals;
use pico_rs::ticks::Ticks;
use pico_rs::timer::Timer0;
use pico_rs::uart::{DataBits, Parity, StopBits, Uart1};

#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::exe();

const TMC_UART_BAUD: u32 = 115_200;
const SLAVE_ADDRS: [u8; 4] = [0x00, 0x01, 0x02, 0x03];

const REG_GCONF: u8 = 0x00;
const REG_GSTAT: u8 = 0x01;
const REG_IFCNT: u8 = 0x02;
const REG_IOIN: u8 = 0x06;
const REG_IHOLD_IRUN: u8 = 0x10;
const REG_TSTEP: u8 = 0x12;
const REG_SG_RESULT: u8 = 0x41;
const REG_CHOPCONF: u8 = 0x6C;
const REG_DRV_STATUS: u8 = 0x6F;
const REG_PWMCONF: u8 = 0x70;

const TMC_SYNC: u8 = 0x05;

fn tmc_crc8(data: &[u8]) -> u8 {
    let mut crc: u8 = 0;
    for &byte in data {
        let mut current = byte;
        for _ in 0..8 {
            if (crc >> 7) ^ (current & 0x01) != 0 {
                crc = (crc << 1) ^ 0x07;
            } else {
                crc <<= 1;
            }
            current >>= 1;
        }
    }
    crc
}

fn read_byte_timeout<TX: Uart1TxPin, RX: Uart1RxPin>(
    uart: &Uart1<TX, RX>,
    timeout_loops: u32,
) -> Option<u8> {
    for _ in 0..timeout_loops {
        if let Some(byte) = uart.read_byte() {
            return Some(byte);
        }
        for _ in 0..100 {
            core::hint::spin_loop();
        }
    }
    None
}

fn tmc_write_reg<TX: Uart1TxPin, RX: Uart1RxPin>(
    uart: &Uart1<TX, RX>,
    slave_addr: u8,
    reg: u8,
    value: u32,
) {
    let mut datagram = [0u8; 8];
    datagram[0] = TMC_SYNC;
    datagram[1] = slave_addr;
    datagram[2] = reg | 0x80;
    datagram[3] = (value >> 24) as u8;
    datagram[4] = (value >> 16) as u8;
    datagram[5] = (value >> 8) as u8;
    datagram[6] = value as u8;
    datagram[7] = tmc_crc8(&datagram[..7]);

    uart.write(&datagram);
    uart.flush();

    for _ in 0..8 {
        let _ = read_byte_timeout(uart, 2000);
    }
}

fn tmc_read_reg<TX: Uart1TxPin, RX: Uart1RxPin>(
    uart: &Uart1<TX, RX>,
    timer: &Timer0,
    slave_addr: u8,
    reg: u8,
) -> Option<u32> {
    let mut request = [0u8; 4];
    request[0] = TMC_SYNC;
    request[1] = slave_addr;
    request[2] = reg;
    request[3] = tmc_crc8(&request[..3]);

    uart.write(&request);
    uart.flush();

    for _ in 0..4 {
        let _ = read_byte_timeout(uart, 2000);
    }

    timer.delay_us(200);

    let mut reply = [0u8; 8];
    for slot in reply.iter_mut() {
        match read_byte_timeout(uart, 5000) {
            Some(b) => *slot = b,
            None => return None,
        }
    }

    let crc = tmc_crc8(&reply[..7]);
    if crc != reply[7] {
        warn!("[addr={:#04x}] CRC error", slave_addr);
        return None;
    }
    if reply[0] != TMC_SYNC || reply[1] != 0xFF {
        warn!("[addr={:#04x}] Bad header", slave_addr);
        return None;
    }

    let value = (reply[3] as u32) << 24
        | (reply[4] as u32) << 16
        | (reply[5] as u32) << 8
        | (reply[6] as u32);

    Some(value)
}

#[entry]
fn main() -> ! {
    info!("=== TMC2209 UART Scan (UART1: TX=GP4, RX=GP5) ===");
    info!("");

    let dp = Peripherals::take().unwrap();
    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    info!(
        "Clock: sys={} Hz, peri={} Hz",
        clocks.sys_freq(),
        clocks.peri_freq()
    );

    let ticks = Ticks::new(dp.TICKS, &clocks);
    ticks.init_timer0();
    let timer = Timer0::init(dp.TIMER0);

    let pins = Pins::new(dp.IO_BANK0, dp.PADS_BANK0);

    let uart = Uart1::new(
        dp.UART1,
        pins.pin::<4>(),
        pins.pin::<5>(),
        TMC_UART_BAUD,
        clocks.peri_freq(),
    );
    uart.set_format(DataBits::Eight, StopBits::One, Parity::None);
    info!(
        "UART1: {} baud, actual={}",
        TMC_UART_BAUD,
        uart.baudrate(clocks.peri_freq())
    );
    info!("");

    timer.delay_ms(100);

    let mut ok = 0u32;
    let mut fail = 0u32;

    for &addr in SLAVE_ADDRS.iter() {
        info!(
            "--- addr={:#04x} (MS1={}, MS2={}) ---",
            addr,
            addr & 1,
            (addr >> 1) & 1
        );

        match tmc_read_reg(&uart, &timer, addr, REG_IOIN) {
            Some(ioin) => {
                let ver = (ioin >> 24) & 0xFF;
                let enn = ioin & 1;
                let ms1 = (ioin >> 1) & 1;
                let ms2 = (ioin >> 2) & 1;
                let step = (ioin >> 4) & 1;
                let dir = (ioin >> 5) & 1;
                if ver == 0x21 {
                    info!("  IOIN={:#010x}  version={:#04x} ✓", ioin, ver);
                    info!(
                        "    ENN={}  MS1={}  MS2={}  STEP={}  DIR={}",
                        enn, ms1, ms2, step, dir
                    );
                } else {
                    warn!(
                        "  IOIN={:#010x}  version={:#04x} (expected 0x21)",
                        ioin, ver
                    );
                    fail += 1;
                    continue;
                }
            }
            None => {
                warn!("  No response!");
                fail += 1;
                continue;
            }
        }

        timer.delay_ms(5);

        // 清除并读 GSTAT
        tmc_write_reg(&uart, addr, REG_GSTAT, 0x07);
        timer.delay_ms(5);
        if let Some(gstat) = tmc_read_reg(&uart, &timer, addr, REG_GSTAT) {
            let rst = gstat & 1;
            let drv_err = (gstat >> 1) & 1;
            let uv_cp = (gstat >> 2) & 1;
            info!(
                "  GSTAT={:#04x}  reset={}  drv_err={}  uv_cp={}",
                gstat & 0x7,
                rst,
                drv_err,
                uv_cp
            );
        }

        // 写 GCONF
        tmc_write_reg(&uart, addr, REG_GCONF, (1 << 6) | (1 << 7));
        timer.delay_ms(5);

        // 读回 GCONF 并解析
        if let Some(gconf) = tmc_read_reg(&uart, &timer, addr, REG_GCONF) {
            let pdn_disable = (gconf >> 7) & 1;
            let multistep = (gconf >> 6) & 1;
            let shaft = (gconf >> 4) & 1;
            let en_spread = gconf & 1;
            info!(
                "  GCONF={:#06x}  pdn_dis={}  multistep={}  shaft={}  spread={}",
                gconf, pdn_disable, multistep, shaft, en_spread
            );
        }

        // 读 IHOLD_IRUN
        if let Some(ihold_irun) = tmc_read_reg(&uart, &timer, addr, REG_IHOLD_IRUN) {
            let ihold = ihold_irun & 0x1F;
            let irun = (ihold_irun >> 8) & 0x1F;
            let iholddelay = (ihold_irun >> 16) & 0xF;
            info!(
                "  IHOLD_IRUN={:#08x}  ihold={}  irun={}  iholddelay={}",
                ihold_irun, ihold, irun, iholddelay
            );
        }

        // 读 CHOPCONF
        if let Some(chopconf) = tmc_read_reg(&uart, &timer, addr, REG_CHOPCONF) {
            let toff = chopconf & 0xF;
            let hstrt = (chopconf >> 4) & 0x7;
            let hend = (chopconf >> 7) & 0xF;
            let tbl = (chopconf >> 15) & 0x3;
            let mres = (chopconf >> 24) & 0xF;
            let intpol = (chopconf >> 28) & 1;
            let mres_str = match mres {
                0 => "256",
                1 => "128",
                2 => "64",
                3 => "32",
                4 => "16",
                5 => "8",
                6 => "4",
                7 => "2",
                _ => "N",
            };
            info!(
                "  CHOPCONF={:#08x}  toff={}  hstrt={}  hend={}  tbl={}  mres=1/{}  intpol={}",
                chopconf, toff, hstrt, hend, tbl, mres_str, intpol
            );
        }

        // 读 PWMCONF
        if let Some(pwmconf) = tmc_read_reg(&uart, &timer, addr, REG_PWMCONF) {
            let pwm_ofs = pwmconf & 0xFF;
            let pwm_grad = (pwmconf >> 8) & 0xFF;
            let pwm_autoscale = (pwmconf >> 18) & 1;
            let pwm_autograd = (pwmconf >> 19) & 1;
            let pwm_lim = (pwmconf >> 24) & 0xF;
            let pwm_reg = (pwmconf >> 28) & 0xF;
            info!(
                "  PWMCONF={:#08x}  ofs={}  grad={}  autoscale={}  autograd={}  lim={}  reg={}",
                pwmconf, pwm_ofs, pwm_grad, pwm_autoscale, pwm_autograd, pwm_lim, pwm_reg
            );
        }

        // 读 DRV_STATUS
        if let Some(drv) = tmc_read_reg(&uart, &timer, addr, REG_DRV_STATUS) {
            let cs_actual = (drv >> 16) & 0x1F;
            let stst = (drv >> 31) & 1;
            let ol_a = (drv >> 29) & 1;
            let ol_b = (drv >> 30) & 1;
            let otpw = (drv >> 26) & 1;
            let ot = (drv >> 25) & 1;
            let s2ga = (drv >> 27) & 1;
            let s2gb = (drv >> 28) & 1;
            info!(
                "  DRV_STATUS={:#08x}  stst={}  cs_actual={}  olA={}  olB={}  otpw={}  ot={}  s2ga={}  s2gb={}",
                drv, stst, cs_actual, ol_a, ol_b, otpw, ot, s2ga, s2gb
            );
        }

        // 读 SG_RESULT & TSTEP
        if let Some(sg) = tmc_read_reg(&uart, &timer, addr, REG_SG_RESULT) {
            info!("  SG_RESULT={}", sg & 0x3FF);
        }
        if let Some(tstep) = tmc_read_reg(&uart, &timer, addr, REG_TSTEP)
            && tstep != 0xFFFFF
        {
            info!("  TSTEP={} (speed indicator)", tstep & 0xFFFFF);
        }

        // 读 IFCNT
        match tmc_read_reg(&uart, &timer, addr, REG_IFCNT) {
            Some(ifcnt) => {
                info!("  IFCNT={}", ifcnt & 0xFF);
            }
            None => {
                warn!("  IFCNT read failed!");
            }
        }

        ok += 1;
    }

    info!("");
    info!("=== Done: {} OK, {} FAIL ===", ok, fail);
    if fail == 0 {
        info!("All chips working!");
    }

    loop {
        core::hint::spin_loop();
    }
}
