# RP2350 boot2（可选的第二阶段引导）

本文说明 `rp2350-boot2` 做了什么、为什么它是可选的，以及如何打开和验证它。

## RP2350 上「boot2」是什么

RP2040 的 boot ROM 会把 flash 起始处的 256 字节拷到 SRAM，校验最后 4 字节的 CRC32，
然后跳进去执行。那段代码负责配置 QSPI，再把控制权交回 ROM 引导用户映像，因此在 RP2040
上没有 boot2 就无法从 flash 启动。

RP2350 不是这样：

- boot ROM 自己做一次**尽力而为**的 XIP 配置，然后依据映像里的 `IMAGE_DEF` boot block
  直接进入用户映像。这个 boot block 由 `#[main]` 宏自动生成（见 `rp2350::block`）。
  也就是说，**RP2350 不需要 boot2 就能启动**。
- RP2350 的 boot2 是一个**可选**的、位置无关的「XIP 配置函数」，最大 256 字节，
  **没有 CRC**。它由映像自己在启动早期调用，把 ROM 那套保守配置换成激进的配置：更高的
  接口时钟、调好的 RX 采样延迟，以及对支持 quad 的 flash 改用 quad 连续读（`EBh` + mode
  bits）取代单比特 `03h` 读。

因为它要改写的正是「指令取指所依赖的 QMI 时序寄存器」，所以它不能在被 XIP 取指的同时
运行。`rp2350_boot2::init()` 先把 blob 拷到栈上，再从 RAM 调用——Pico SDK 的 `crt0` 用的
是同一招。

## 打开方式

默认关闭（RP2350 不打开也能启动，boot2 只是让 XIP 更快）：

```toml
[dependencies]
rp2350 = { path = "../rp2350", features = ["boot2-w25q80"] }
```

示例工程里直接用 feature 跑：

```sh
cargo run --bin blink --features boot2-w25q80
# flash 不支持 quad 读时：
cargo run --bin blink --features boot2-generic-03h
```

feature 分两层：`boot2` 是开关（引入 `rp2350-boot2`，并让 `Reset` 调用它），`boot2-<型号>`
选具体器件变体并隐含 `boot2`（型号变体各自带上它需要的附加项，比如 w25q80 的 quad-enable
编程路径）。只打开 `boot2` 而不选变体时，`rp2350-boot2` 会以 `compile_error!` 明确报错，
而不是静默链一个空的 `.boot2`；两个变体同时打开也会被拦住。名字带型号，方便以后增加别的器件。

`rp2350-boot2` crate 自身的默认变体是最安全的 `generic-03h`（让 IDE/独立构建能直接通过）；
但 `rp2350-rt` 声明依赖时关掉了默认 feature，始终按所选 feature 显式指定变体。

`rp2350-rt` 在 `Reset` 里、`.data` 初始化之前调用 `rp2350_boot2::init()`。也可以自己调用
（例如在 `__pre_init` 里）；此时它必须运行在 RAM 中，run-from-ram 配置下这一步自动满足。

## 两个变体

| feature | boot2 名称 | 适用器件 | 做法 |
| --- | --- | --- | --- |
| `boot2-w25q80` | `boot2_w25q80` | W25Q80 系列（W25Q80 / W25Q16JV / W25Q32JV），以及上游列出的 AT25SF081、S25FL132K0 | `EBh` fast quad I/O + 连续读模式 |
| `boot2-generic-03h` | `boot2_generic_03h` | 任何响应 `03h` 串行读的 flash | 保持单比特传输，最安全但约慢 3 倍 |

命名说明：上游 Pico SDK 里对应的文件是 `boot2_w25q080.S`，但 Winbond 并没有 `W25Q080`
这个型号（实际是 W25Q80 系列），所以这里按实际型号命名。启用 `binary-info`（`rp2350-rt`
的两个 boot2 feature 都已默认打开）后，变体名会以 `BINARY_INFO_ID_RP_BOOT2_NAME` 记录在
`.bi_entries` 里，picotool 可以读出来。

`boot2-w25q80` 变体默认带上 QE 编程路径（`program-status-reg`）：先读 SR2，若 QE 未置位就用 `01h`
写 SR1。注意这一步会清掉 flash 的块保护位——不是所有器件都支持 volatile status 写，这是
上游的取舍。

## 256 字节上限

`.boot2` 段由 `rp2350-rt` 的 `link.x` / `link_ram.x` 定义，链接脚本里带断言：

```
ASSERT(__boot2_end__ - __boot2_start__ <= 256, ...)
```

超标会让链接失败，而不是留到运行时才发现。当前两个变体都远低于上限：`boot2-w25q80`（含 QE 路径）
172 字节，`boot2-generic-03h` 32 字节。

## 为什么 `.boot2` 需要一个符号引用

blob 所在的编译单元除了 `__boot2_entry_point` 之外不导出任何 Rust 会调用的东西。链接器不会
从 rlib 里拉出一个没人引用的成员，所以 `init()` 里对入口符号的引用是**必需的**：删掉它
`.boot2` 会静默变成空段（链接成功，但 boot2 不生效）。Pico SDK 的 `crt0` 出于同样原因保留了
这个引用。`init()` 顺便用该符号算出入口在 blob 内的偏移，而不是假定它在偏移 0。

## 检查一次构建

```sh
cargo build -p rp2350-examples --bin blink --features boot2-w25q80 --target thumbv8m.main-none-eabihf
readelf -S --wide target/thumbv8m.main-none-eabihf/debug/blink | grep '\.boot2'
rust-objdump -d --start-address=<.boot2 的地址> target/thumbv8m.main-none-eabihf/debug/blink
```

要确认的点：

- `.boot2` 非空，且 `<= 0x100` 字节；
- `rust-objdump -r` 里 `.boot2` **没有**重定位记录：blob 自包含、位置无关，字面量池也在段内；
- 反汇编末尾是 `pop {pc}`，紧跟着 `.word` 字面量池；
- `.bi_entries` 里 12 字节记录的最后一个字指向 `"boot2_w25q80"` 字符串。

## 验证提速

最直接的证据是 QMI 窗口 0 的时序寄存器：启动后读 `QMI_M0_TIMING`（偏移 `0x0c`），打开
boot2 时应该是 `0x40000202`（CLKDIV = 2、RXDELAY = 2、cooldown = 1），关闭时是 ROM 留下的
保守值。也可以用一段读大块 `.rodata` 的循环配合 SysTick 计时，比较打开前后的耗时；
或读 bootrom 的 flash device info 确认 QE 位状态。
