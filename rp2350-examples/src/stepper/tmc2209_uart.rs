//! # TMC2209 UART 通讯测试 — 4 片芯片扫描
//!
//! 使用 UART1 (GPIO4=TX, GPIO5=RX)，轮询 4 个从机地址 (0x00~0x03)，
//! 读取 IOIN 寄存器确认芯片版本号 (应为 0x21)。
//!
//! 本示例基于 `rp2350_drivers::tmc2209::Tmc2209` 驱动，
//! 同一 UART 总线上创建 4 个驱动实例共享 `&uart`。
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
//!                    1kΩ    │
//!   GP5 (UART1_RX) ─────────┤
//!                           │
//!                           ├──────── 所有 TMC2209 的 PDN_UART
//!   3.3V (VCC_IO) ───/\/\/──┘
//!                    10kΩ
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
//! cd rp2350-examples
//! cargo run --bin tmc2209_uart --target thumbv8m.main-none-eabihf
//! ```

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;

use rp2350::clocks::ClockBuilder;
use rp2350::gpio::Pins;
use rp2350::main;
use rp2350::pac::Peripherals;
use rp2350::ticks::Ticks;
use rp2350::timer::Timer0;
use rp2350::uart::{DataBits, Parity, StopBits, Uart1};
use rp2350_drivers::tmc2209::{Gconf, REG_TSTEP, Tmc2209};

const TMC_UART_BAUD: u32 = 115_200;

#[main]
fn main() -> ! {
    info!("=== TMC2209 UART Scan (rp2350_drivers::tmc2209) ===");
    info!("UART1: TX=GP4, RX=GP5");
    info!("");

    let dp = Peripherals::take().unwrap();
    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    info!(
        "Clock: sys={} Hz, peri={} Hz",
        clocks.sys_freq(),
        clocks.peri_freq()
    );

    let mut ticks = Ticks::init(dp.TICKS, &clocks);
    ticks.init_timer0();
    let timer = Timer0::init(dp.TIMER0);

    let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);

    let mut uart = Uart1::init(
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

    // 同一 UART 总线上创建 4 个驱动实例，不同从机地址
    let chips: [Tmc2209<'_, Uart1<_, _>>; 4] = [
        Tmc2209::new(&uart, 0x00),
        Tmc2209::new(&uart, 0x01),
        Tmc2209::new(&uart, 0x02),
        Tmc2209::new(&uart, 0x03),
    ];

    let mut ok = 0u32;
    let mut fail = 0u32;

    for (i, tmc) in chips.iter().enumerate() {
        let addr = tmc.slave_addr();
        info!(
            "--- Chip {}  addr={:#04x} (MS1={}, MS2={}) ---",
            i,
            addr,
            addr & 1,
            (addr >> 1) & 1
        );

        timer.delay_ms(5);

        // 读取 IOIN 验证芯片
        match tmc.read_ioin(&timer) {
            Ok(ioin) => {
                let ver = ioin.version();
                if ver == 0x21 {
                    info!("  IOIN={:#010x}  version={:#04x} ✓", u32::from(ioin), ver);
                    info!(
                        "    ENN={}  MS1={}  MS2={}  STEP={}  DIR={}",
                        ioin.enn(),
                        ioin.ms1(),
                        ioin.ms2(),
                        ioin.step(),
                        ioin.dir()
                    );
                } else {
                    warn!(
                        "  IOIN={:#010x}  version={:#04x} (expected 0x21)",
                        u32::from(ioin),
                        ver
                    );
                    fail += 1;
                    continue;
                }
            }
            Err(_) => {
                warn!("  No response!");
                fail += 1;
                continue;
            }
        }

        timer.delay_ms(5);

        // 清除并读 GSTAT
        let _ = tmc.clear_gstat(&timer);
        timer.delay_ms(5);
        if let Ok(gstat) = tmc.read_gstat(&timer) {
            info!(
                "  GSTAT={:#04x}  reset={}  drv_err={}  uv_cp={}",
                u32::from(gstat),
                gstat.reset(),
                gstat.drv_err(),
                gstat.uv_cp()
            );
        }

        // 写 GCONF: multistep_filt + pdn_disable
        let _ = tmc.write_gconf(
            &Gconf::new()
                .with_multistep_filt(true)
                .with_pdn_disable(true),
            &timer,
        );
        timer.delay_ms(5);

        // 读回 GCONF
        if let Ok(gconf) = tmc.read_gconf(&timer) {
            info!(
                "  GCONF={:#06x}  pdn_dis={}  multistep={}  shaft={}  spread={}",
                u32::from(gconf),
                gconf.pdn_disable(),
                gconf.multistep_filt(),
                gconf.shaft(),
                gconf.en_spread_cycle()
            );
        }

        // 读 IHOLD_IRUN
        if let Ok(ihold_irun) = tmc.read_ihold_irun(&timer) {
            info!(
                "  IHOLD_IRUN={:#08x}  ihold={}  irun={}  iholddelay={}",
                u32::from(ihold_irun),
                ihold_irun.ihold(),
                ihold_irun.irun(),
                ihold_irun.iholddelay()
            );
        }

        // 读 CHOPCONF
        if let Ok(chopconf) = tmc.read_chopconf(&timer) {
            let mres_str = match chopconf.mres() {
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
                u32::from(chopconf),
                chopconf.toff(),
                chopconf.hstrt(),
                chopconf.hend(),
                chopconf.tbl(),
                mres_str,
                chopconf.intpol()
            );
        }

        // 读 PWMCONF
        if let Ok(pwmconf) = tmc.read_pwmconf(&timer) {
            info!(
                "  PWMCONF={:#08x}  ofs={}  grad={}  freq={}  autoscale={}  autograd={}  lim={}  reg={}",
                u32::from(pwmconf),
                pwmconf.pwm_ofs(),
                pwmconf.pwm_grad(),
                pwmconf.pwm_freq(),
                pwmconf.pwm_autoscale(),
                pwmconf.pwm_autograd(),
                pwmconf.pwm_lim(),
                pwmconf.pwm_reg()
            );
        }

        // 读 DRV_STATUS
        if let Ok(drv) = tmc.read_drv_status(&timer) {
            info!(
                "  DRV_STATUS={:#08x}  stst={}  cs_actual={}  olA={}  olB={}  otpw={}  ot={}  s2ga={}  s2gb={}",
                u32::from(drv),
                drv.stst(),
                drv.cs_actual(),
                drv.ola(),
                drv.olb(),
                drv.otpw(),
                drv.ot(),
                drv.s2ga(),
                drv.s2gb()
            );
        }

        // 读 SG_RESULT
        if let Ok(sg) = tmc.read_sg_result(&timer) {
            info!("  SG_RESULT={}", sg);
        }

        // 读 TSTEP
        if let Ok(tstep) = tmc.read_reg(REG_TSTEP, &timer)
            && tstep != 0xFFFFF
        {
            info!("  TSTEP={} (speed indicator)", tstep & 0xFFFFF);
        }

        // 读 IFCNT
        match tmc.read_ifcnt(&timer) {
            Ok(ifcnt) => {
                info!("  IFCNT={}", ifcnt);
            }
            Err(_) => {
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
