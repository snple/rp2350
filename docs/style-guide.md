# rp2350 项目设计风格指南

本文档整理 `rp2350`（Raspberry Pi Pico 2 / RP2350 的 Rust HAL）项目的设计约定，
供同类嵌入式 Rust 项目参考。目标是「每个新模块都长得和既有模块一样」：
一致的骨架、一致的所有权模型、一致的错误处理，以及一致的验证流程。

> 范围：本仓库位于 `/home/danc/code/rp2350/rp2350`，HAL crate 名为 `rp2350`。
> 本指南描述的是**约定**，不重复具体 API 文档。

---

## 1. 工作区结构（Crate 分层）

```
rp2350/                # 工作区根
├── rp2350/            # HAL：本指南的主角（手写，面向用户）
├── rp2350-pac/        # 由 SVD 生成的 PAC（peripheral access crate，勿手改）
├── rp2350-rt/         # 运行时：reset handler、栈、FPU 初始化
├── rp2350-boot2/      # 可选 boot2：#![no_std] 的 XIP 配置 blob + 拷到 RAM 执行的入口
├── rp2350-macro/      # 过程宏：#[main] #[exception] #[bind] pio_asm! ramfunc
├── rp2350-examples/   # 各外设的独立二进制示例
├── rp2350-drivers/    # 设备驱动（TMC2209 等），自带所需总线抽象（bus::UartBus）
├── rp2350-usb/        # USB 设备栈（独立 crate）
├── docs/              # 设计/使用说明（本文件、pwm.md）
└── xtask/             # 构建辅助
```

分层原则：

- **PAC 只做寄存器**：`rp2350-pac` 由 SVD 生成，提供 `RegisterBlock`、字段常量、
  访问器和 `peripheral!` 宏注入的 ZST 外设类型。HAL 不复制这些，只做语义封装。
- **rt 只做启动**：栈初始化、`.bss` 清零、MSPLIM、FPU（CP10+CP11）使能、然后调 `main`。
- **boot2 只做 XIP 配置**：`rp2350-boot2` 里只有位置无关的汇编 blob、寄存器常量表和
  「拷到 RAM 再调用」的入口；它不碰其它启动步骤，由 `rt` 在 `Reset` 早期调用（默认关闭）。
- **macro 只做代码生成**：入口、异常/中断绑定、PIO 汇编、RAM 函数。
- **HAL 是唯一的手写用户接口层**：每个 PAC 外设对应一个 `src/<name>.rs` 模块，
  在 `lib.rs` 里 `pub mod` 并 `pub use` 常用类型。

`lib.rs` 统一重导出入口：

```rust
pub use rp2350_pac as pac;
pub use rp2350_rt as rt;
pub use rp2350_pac::{Interrupt, Peripherals, Vector};
```

---

## 2. HAL 模块骨架（每个模块都长这样）

一个「典型」外设模块（以 `sysinfo` / `trng` / `sha256` / `qspi` 为模板）：

```rust
//! 模块文档：一句话说清外设，列出 Features，给一个 Example（` ```ignore`）。

use crate::Steal;
use crate::reset::Resettable; // 若该外设可复位

/// 外设包装结构体。拥有 PAC ZST，强制所有权语义。
pub struct Foo {
    foo: rp2350_pac::FOO,
}

impl Foo {
    /// 从 PAC 拿所有权。若可复位，先 reset + unreset。
    pub fn init(foo: rp2350_pac::FOO) -> Self {
        <rp2350_pac::FOO as Resettable>::init(); // 可选
        Self { foo }
    }

    /// 无所有权检查地拿句柄（ISR / 已初始化场景）。
    /// # Safety：调用者保证已初始化、且无数据竞争。
    pub unsafe fn steal() -> Steal<Self> { Steal(Self { foo: unsafe { rp2350_pac::FOO::steal() } }) }

    /// 逃生口：返回底层寄存器块。
    /// 可写外设通常标 `unsafe`（绕过 HAL 安全保证）；只读外设可省略。
    pub unsafe fn regs(&self) -> &rp2350_pac::foo::RegisterBlock { &self.foo }

    /// 归还 PAC 外设。
    pub fn release(self) -> rp2350_pac::FOO { self.foo }

    // ... 具体方法
}
```

要点：

- **每模块一个 `pub struct`，字段是 PAC 外设 ZST**（字段名常与外设同名，如 `qmi`、`trng`）。
- 方法几乎都取 `&self`（`&mut self` 只用于真正需要内部可变状态的地方，如 `Trng` 的熵缓冲）。
- 每个错误/状态用 `#[derive(Clone, Copy, PartialEq, Eq, Debug)]` +
  `#[cfg_attr(feature = "defmt", derive(defmt::Format))]`。
- `#[inline]` 标在热路径/转发方法上。

---

## 3. PAC 关键事实（别踩的坑）

- **PAC 外设结构体是非 `Copy` 的零大小类型（ZST）**：
  `pub struct FOO { _marker: PhantomData<*const ()> }` + `peripheral!` 宏注入
  `Deref<Target = RegisterBlock>` 和一个固定的 `PTR` 常量。
- 因此**不能从 `&self` 里拷贝 ZST 给子句柄**；子句柄必须**借用** `&'a rp2350_pac::FOO`。
  控制器级 `steal()` 返回 `Steal<Self>`（自己拥有 ZST，无生命周期问题）。
- **寄存器访问走 `AtomicCell<u32>` / `VolatileCell<u32>`**（生成器按寄存器类型区分）：
  - `AtomicCell`：**可写**（RW/WO）且在原子别名地址范围内的寄存器，有
    `set_bits` / `clr_bits` / `xor_bits`（硬件 SET/CLR 别名 +0x1000/+0x2000/+0x3000，天然原子）。
  - `VolatileCell`：**只读**（RO）寄存器，只有 `read_bits` / `write_bits`，不暴露原子写。
  - W1C（写 1 清）中断寄存器清中断用 `write_bits(mask)`，**别用 clr_bits**。
- 字段命名统一：`fn field()` 读、`fn write_field(v)` 写多位数、
  `fn set_field()` / `fn clr_field()` 操作单比特位。
- **禁止裸指针 + 魔法数访问寄存器**：一律用 PAC 的命名寄存器/字段。

---

## 4. 所有权模型与子句柄

核心分层（照抄 `PIO` 的 `Sm`、`QSPI` 的 `Direct`）：

```
控制器（拥有 PAC ZST）──steal()──► Steal<Self>
        │
        └── sub(&self) ──► 子句柄 X<'a> { regs: &'a PAC }
```

- **控制器拥有 ZST**：`init(pac)` 或 `steal() -> Steal<Self>`。
- **子句柄借用**：`struct X<'a> { regs: &'a rp2350_pac::XXX }`，
  `controller.sub(&self) -> X<'_>` 里写 `regs: &self.pac`。
  子句柄不能独立 `steal()`（所有权在控制器）。
- `Steal<T>`（定义在 `lib.rs`）实现 `Deref<T>` 但**不**实现 `DerefMut`，
  因此 `steal()` 出来的句柄只能调 `&self` 方法，无法调 `&mut self` 方法。
- 「基础类型 + 变体 Deref 包装」消除复制：`UART WithFlowControl`、`SPI WithCs`
  重构为 `{ inner, _pin }` + `Deref`/`DerefMut` + 委托，而不是复制整套方法。

示例（`Qspi::direct()`）：

```rust
pub struct Direct<'a> { qmi: &'a QMI }
impl Qspi {
    pub fn direct(&self) -> Direct<'_> { Direct { qmi: &self.qmi } }
}
```

---

## 5. 驱动 trait 模式（设备驱动解耦）

`rp2350` HAL **不内置** `I2cBus` / `SpiBus` / `UartBus` / `MonotonicTimer` /
`AlarmTimer` 等抽象 trait：需要抽象时由调用者（driver crate 或最终应用）自行
定义并实现，HAL 只提供具体外设类型。

`rp2350-drivers` 里的设备驱动（如 `tmc2209.rs`、`stepper`）不依赖具体 HAL 类型，
而是依赖本 crate 定义或 HAL 暴露的抽象：

- `crate::bus::UartBus`：`write` / `read` / `read_available` / `read_raw` / `flush`。
  已在 `rp2350-drivers::bus` 为 RP2350 的 `Uart0` / `Uart1` 及其流控变体实现，
  因此 `tmc2209` 等驱动开箱即用；需要 mock 或其他总线时由调用者自行实现。
- `crate::timer::MonotonicTimer` / `crate::timer::AlarmTimer`：已在
  `rp2350-drivers::timer` 为 `Timer0` / `Timer1`（含 `Steal` 句柄）实现，
  供 `stepper` 等需要时间源 / alarm 调度的驱动使用。
- `rp2350::delay::DelayUs`：`delay_us(us)`，用 `&impl DelayUs` 注入延时（由 HAL 提供）。

其他需要 `I2C` / `SPI` 抽象的调用者，可参照 `rp2350-drivers::bus` 的模式
自行定义 trait 并为具体类型实现。

---

## 6. 错误处理约定

- **每个模块定义自己的 `Error` 枚举**（`#[derive(...)] + defmt`），只列本模块真正
  返回的错误。**不要**定义无人使用的 `Error`（死代码比少一个枚举更糟）。
- **阻塞方法在超时时 `panic`**：用 `crate::spin::spin_wait(what, cond)`。
- **`try_*` 方法返回 `Result<_, Error>`**：用 `crate::spin::try_spin_wait(cond)`，
  失败映射到 `Error::Timeout` / `Error::TxFull` 等。
- 对称的两层 API（见 `qspi::Direct`）：`read_byte`（panic）与 `try_read_byte`（Result）。
- `spin` 模块提供三个助手：
  - `spin_wait(what, cond)` — 有界自旋，超时 panic（hang-guard）。
  - `try_spin_wait(cond) -> bool` — 有界自旋，不 panic。
  - `spin_wait_max(max, what, cond)` — 自定义上界的自旋。

```rust
pub fn wait_idle(&self) {
    crate::spin::spin_wait("QMI idle", || !self.qmi.direct_csr.busy());
}
pub fn try_wait_idle(&self) -> Result<(), Error> {
    if crate::spin::try_spin_wait(|| !self.qmi.direct_csr.busy()) { Ok(()) } else { Err(Error::Busy) }
}
```

---

## 7. 配置结构体与构建器

- **成组的配置抽成结构体**，带 `Default`（对应硬件复位值，并注明）：
  `TimingConfig`、`Format`（qspi）、`ClockConfig`（clocks builder）。
- 复位默认值以 SVD 的 `resetValue` 为准（例如 `M0_TIMING` 复位 `0x4000_0004`
  → `clkdiv=4, cooldown=1`），注释里写明十六进制原值。
- 需要多步配置的用**构建器**（`clocks::ClocksBuilder`），提供 `const fn` 链式方法。

---

## 8. 复位与启动

- `reset.rs` 提供 `Resettable` trait，用 `impl_resettable!` 宏为每个可复位 PAC
  类型实现 `RESET_MASK`；`init()` = reset + unreset（保证干净状态）。
- **例外：与 flash 执行相关的模块不自动复位**。`Qspi`、`Xip`、`PadsQspi` 的
  `init` 都不复位 —— 从 flash 执行代码时复位 QMI / XIP 缓存 / flash 焊盘会破坏 XIP。
- `startup::init()`（由 `#[main]` 宏调用）做核级初始化：
  释放所有 spinlock + `Cpacr::enable_defaults()`（CP10+CP11/CP4/CP0）。
  双核时 Core 1 要在自己的入口里手动调这些。

---

## 9. 宏约定

- **过程宏在 `rp2350-macro`**：`#[main]`（生成入口 + 调 `startup::init`）、
  `#[exception]`、`#[bind(IRQ)]`、`pio_asm!`、`#[ramfunc]`。
- **实例化宏用 `paste`**：`impl_uart!`、gpio 的 per-pin 宏用 `paste!` 拼接标识符
  （`[<gpio $n>]`、`[<Uart $n>]`）。
- **`macro_rules!` 卫生坑**：宏内部 `let reg = ...` 绑定的标识符对调用方传入的
  `$body:block` **不可见**（E0425 "not found ... due to macro hygiene"）。
  修法：把标识符作为宏参数传进去：

```rust
// 错误：body 里看不到宏内 let 绑定的 reg
macro_rules! with_window { ($m0:ident, $m1:ident, $body:block) => {
    match w { M0 => { let reg = &qmi.$m0; $body } ... } }; }

// 正确：reg 作为 $reg:ident 参数传入
macro_rules! with_window { ($m0:ident, $m1:ident, $reg:ident, $body:block) => {
    match w { M0 => { let $reg = &qmi.$m0; $body } ... } }; }
// 调用：with_window!(m0_timing, m1_timing, reg, { reg.write_clkdiv(x); });
```

- 用私有 trait + 宏消除「多个同构寄存器类型」的重复（`pads_qspi` 的 `QspiPadReg`，
  6 个 pad 寄存器类型共用一套方法）。

---

## 10. 命名与文档风格

- 模块名与外设缩写一致：`sysinfo` / `qspi` / `ppb` / `pads_qspi`。
- 枚举 `#[repr(u32)]`，变体用有意义的名字；电压/宽度类枚举注意位宽语义
  （如 `BusWidth { Single=0, Dual=1, Quad=2 }` 直接对应寄存器编码）。
- 模块文档统一结构：一段概述 → `## Features` 列表 → `## Example`（` ```ignore`），
  必要时加 `## Architecture` / `## Pin Mapping`。
- 跨模块复用的概念放回源头再引用：`xip` 复用 `crate::qspi::Window`，
  `pads_qspi` 复用 `crate::gpio::{DriveStrength, SlewRate, Pull}`，避免重复定义。

---

## 11. 验证流程（每个改动都跑这四条）

```bash
cargo check -p rp2350
cargo clippy -p rp2350 --lib      # 勿用 --all-targets：no_std panic_handler 与 test harness 冲突
cargo check -p rp2350-examples --bins
cargo fmt -p rp2350 -- --check
```

生成代码漂移检查（改过 SVD / 生成器后）：

```bash
cargo xtask gen-pac --check
```

### 构建与工具链坑

- **Rust 2024 edition**：`unsafe fn` 体内不是隐式 unsafe 块（`unsafe-op-in-unsafe-fn`）。
  PAC 生成代码里 `Peripherals::steal` 的外层 `unsafe {}` 是必需的，勿移除。
- **chip feature 互斥（rp2350a / rp2350b）**：Cargo feature 是**并集**语义，芯片选择权
  要上移到**顶层二进制**（examples）；中间层（drivers）只透传、不自己选定。否则
  `--features rp2350b` 会因 default 里的 rp2350a 冲突。
- **MSRV = 1.88**（generator 用 let-chains）；`rust-toolchain.toml` = stable + `thumbv8m.main-none-eabihf` target。
- **生成代码不要手改**：`rp2350-pac/src/generated/` 是 `cargo xtask gen-pac` 产物，
  改动走 `xtask/src/generator.rs` 再重新生成（生成后已自动 `rustfmt`）。
- `rt` 的 `link.x` `INCLUDE device.x`，`device.x` 由 PAC build.rs 提供（rt 未声明对 PAC 的依赖，隐式耦合）。

---

## 12. 常见 bug 模式（历史教训）

- **有界自旋误用于「硬件必然完成」的等待** → 该用无界 `while`；反之可能卡死的等待要加 hang-guard。
- **W1C 寄存器写反方向**（该 `write_bits` 却用 `clr_bits`）。
- **除零**（分频/计算路径）。**位移错误**（如 PIO 13 写成 12）。
- **`1u32 << 32` 是 UB**。
- **空数据不碰总线**；**事务必须等硬件完成**再返回。
- **裸指针 + 魔法数** → 改用 PAC 命名寄存器 / `core::ptr::addr_of!`。
- **硬件事实要核实，别凭印象**：如 RP2350 FPU 需要 CP10 **和** CP11 同时使能
  （CP11 镜像 CP10，漏开 CP11 会在 Core 1 上坏 FPU）；ATRANS 要先写 SIZE 再写 BASE
  （写 BASE 时锁存翻译）；QMI 只有 CLKDIV 可在传输中热改。

---

## 13. 新增一个外设模块的清单

1. `cargo` 确认 PAC 已有对应外设（`rp2350-pac/src/generated/`）。
2. 新建 `src/<name>.rs`，照第 2 节骨架写；确认是否可复位（第 8 节）。
3. 在 `lib.rs` 加 `pub mod <name>;` 和 `pub use <name>::<Type>;`。
4. 跑第 11 节的四条验证命令，全绿。
5. 需要跨模块复用的类型（如 `Window`）从源头 `pub` 并 `use crate::...` 引用。
