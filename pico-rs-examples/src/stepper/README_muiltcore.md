# RP2350 双核电机控制方案调研

## 1. 目标

将 automicro 固件拆分为双核运行：

| 核心 | 职责 |
|------|------|
| **Core 0** (应用核) | USB 通信、命令解析/分发、TMC2209 UART 配置、状态查询/上报 |
| **Core 1** (运动核) | Timer 中断驱动 STEP 脉冲、梯形加减速、限位/堵转 GPIO 中断 |

核心收益：**STEP 脉冲零抖动** — Timer ISR 在 Core 1 独占运行，USB 轮询永远不会抢占它（两核 NVIC 完全独立）。

---

## 2. pico-rs HAL 多核基础设施（已全部就绪）

### 2.1 multicore 模块 (`pico-rs/src/multicore.rs`)

| API | 说明 |
|-----|------|
| `Multicore::new(psm, ppb)` | 创建多核控制器（仅 Core 0 调用） |
| `launch_core1(entry, stack)` | Boot ROM 启动序列启动 Core 1 |
| `reset_core1()` | 通过 PSM FRCE_OFF 复位 Core 1 |
| `Stack<N>` | Core 1 栈，8 字节对齐，编译期泛型大小 |
| `core_id()` / `is_core0()` / `is_core1()` | 读 SIO CPUID 判断当前核 |

### 2.2 核间通信

| 机制 | API | 特点 |
|------|-----|------|
| **SIO FIFO** | `Fifo::write()` / `read()` / `try_read()` | 硬件队列，8 word/方向，WFE/SEV 同步 |
| **Doorbell** | `Doorbell::signal()` / `wait()` / `set_remote()` | 8-bit 中断标志，可触发对方核的 `SIO_IRQ_BELL` |
| **Spinlock** | `Spinlock::<N>::acquire()` + RAII Guard | 32 个硬件锁，单指令原子获取/释放 |
| **Atomic** | Cortex-M33 原生原子指令 | 无需 spinlock 即可跨核安全读写 |

### 2.3 Critical Section（多核安全）

`critical_section_impl.rs` 注册的实现：

```
cs_acquire:  禁本核中断 → 嵌套计数+1 → (首次) 获取 Spinlock-31
cs_release:  (末次) 释放 Spinlock-31 → 嵌套计数-1 → 恢复本核中断
```

- 每核独立的 `CS_NESTING[2]` 嵌套计数器
- `interrupt::free()` 和 `critical_section::with()` 共享同一套逻辑
- **两核可以同时安全使用 defmt 日志**（defmt-rtt 通过 critical-section 保护 RTT 缓冲区）

### 2.4 中断路由

```rust
// pico-rs/src/interrupt.rs 文档摘录：
// enable() / disable() / set_priority() 只影响 **当前核心** 的 NVIC。
// 要将中断独占路由到某一核心，只需在该核心上 enable，另一核心不 enable 即可。
```

### 2.5 每核外设单例

`Peripherals::take()` 按核心独立跟踪 (`TAKEN: [AtomicBool; 2]`)，Core 0 和 Core 1 各自调用一次。

### 2.6 现有示例

- `pico-rs-examples/src/multicore/multicore.rs` — FIFO 通信 + Spinlock
- `pico-rs-examples/src/multicore/multicore_shared.rs` — 生产者-消费者、Atomic + Spinlock 共享数据

---

## 3. automicro 当前架构（单核）

```text
Core 0:
  USBCTRL_IRQ (pri=0)      ← USB 硬件中断，poll 设备状态机，字节入 RX_QUEUE
  TIMER0_IRQ_0/1/2 (pri=0) ← X/Y/Z 轴 STEP 脉冲 + 梯形加减速
  IO_IRQ_BANK0 (pri=0)     ← 限位/堵转 GPIO 中断
  SPARE_IRQ_0 (pri=128)    ← 命令解析/分发、CDC 响应、轴完成事件推送
  main: init → loop { wfi() }
```

关键共享状态：

| 状态 | 类型 | 同步机制 |
|------|------|----------|
| `AXIS_X/Y/Z` | `AxisState`（全原子字段） | `AtomicU32/I32/Bool` |
| `RX_QUEUE` | `GenericQueue<u8, 256>` | 无锁队列 |
| `USB_DEV/USB_CDC` | `Mutex<RefCell<Option<...>>>` | `interrupt::free` 临界区 |
| `TMC_UART` | `Mutex<RefCell<Option<Uart1>>>` | 临界区 |

---

## 4. 双核分工方案

```text
┌─────────────────────────────────────────────────────────────────┐
│                      RP2350 SoC                                  │
│                                                                  │
│  Core 0 (应用核)                    Core 1 (运动核)              │
│  ─────────────                     ─────────────                │
│  USBCTRL_IRQ (USB)                 TIMER0_IRQ_0/1/2 (STEP)     │
│  SPARE_IRQ_0 (命令解析)             IO_IRQ_BANK0 (限位/堵转)     │
│  SIO_IRQ_BELL (轴完成通知)                                      │
│  TMC2209 UART 配置                                              │
│  补光灯 PWM 控制                                                │
│  loop { wfi() }                    loop { wfi() }              │
│                                                                  │
│                    ┌──────────────────┐                          │
│                    │ AXIS_X/Y/Z       │                          │
│                    │ (AtomicU32/I32)  │ ← 天然跨核安全           │
│                    └──────────────────┘                          │
│                                                                  │
│         ←── FIFO/Doorbell ──→                                   │
│       运动命令 (Core0→1)    轴完成事件 (Core1→0)                │
└─────────────────────────────────────────────────────────────────┘
```

### 4.1 核间通信设计

| 数据流 | 方向 | 机制 | 说明 |
|--------|------|------|------|
| 运动命令 (MOVE/HOME/STOP) | Core 0 → Core 1 | **FIFO** | 编码为 u32 消息字 |
| 轴完成事件通知 | Core 1 → Core 0 | **Doorbell** | 触发 Core 0 的 `SIO_IRQ_BELL` |
| 轴状态 (位置/速度/phase) | Core 1 → Core 0 | **Atomic 字段直读** | AxisState 本身全是原子的 |
| 命令参数 (步数/速度等) | Core 0 → Core 1 | **共享内存 + Spinlock** 或 **Atomic 命令槽** | 大于 1 word 的参数 |

### 4.2 FIFO 命令编码示例

```rust
// 命令字编码 (32-bit):
// [31:28] 命令类型   [27:24] 轴ID   [23:0] 参数
const CMD_MOVE: u32      = 0x1000_0000;  // 梯形加减速运动
const CMD_CONTINUOUS: u32 = 0x2000_0000;  // 连续运转
const CMD_STOP: u32      = 0x3000_0000;  // 停止
const CMD_HOME: u32      = 0x4000_0000;  // 回零
const CMD_SET_POS: u32   = 0x5000_0000;  // 设置位置

// 多于 24-bit 的参数 (如步数 + 速度) 可用两个 FIFO word:
// word 0: 命令类型 + 轴ID + 步数高位
// word 1: 速度/方向等
```

---

## 5. 改造步骤

### 步骤 1: Core 1 入口与栈

```rust
use pico_rs::multicore::{Multicore, Stack};

static mut CORE1_STACK: Stack<4096> = Stack::new();

fn core1_main() -> ! {
    // Core 1 独立外设
    let _cp = CorePeripherals::take().unwrap();

    // 在 Core 1 的 NVIC 上启用 Timer 和 GPIO 中断
    unsafe {
        interrupt::enable(Interrupt::TIMER0_IRQ_0);
        interrupt::enable(Interrupt::TIMER0_IRQ_1);
        interrupt::enable(Interrupt::TIMER0_IRQ_2);
        interrupt::enable(Interrupt::IO_IRQ_BANK0);
    }

    let fifo = Fifo::new();

    loop {
        // 从 FIFO 接收运动命令
        let cmd = fifo.read();  // 阻塞等待 (WFE)
        dispatch_motion_command(cmd);
    }
}
```

### 步骤 2: Core 0 启动 Core 1

```rust
#[entry]
fn main() -> ! {
    // ... 外设初始化 (GPIO, UART, USB, Timer, PWM) ...

    // 配置三轴
    AXIS_X.configure(&axis_x_config);
    AXIS_Y.configure(&axis_y_config);
    AXIS_Z.configure(&axis_z_config);

    // 启动 Core 1
    let mut mc = Multicore::new(dp.PSM, dp.PPB);
    unsafe {
        mc.launch_core1(core1_main, &mut CORE1_STACK);
    }

    // Core 0 仅启用 USB + 软中断 + Doorbell
    unsafe {
        interrupt::enable(Interrupt::USBCTRL_IRQ);
        interrupt::set_priority(Interrupt::SPARE_IRQ_0, 128);
        interrupt::enable(Interrupt::SPARE_IRQ_0);
        interrupt::enable(Interrupt::SIO_IRQ_BELL);
    }

    // Core 0 不再启用 TIMER0_IRQ_x 和 IO_IRQ_BANK0
    loop { wfi(); }
}
```

### 步骤 3: 命令分发改为 FIFO

```rust
// 原来在 SPARE_IRQ_0 中直接调用:
//   AXIS_X.start_move(&timer, steps, dir, speed);
//
// 改为通过 FIFO 发送:
fn send_move_command(axis: AxisId, steps: u32, dir: Direction, speed: u32) {
    let fifo = Fifo::new();
    let cmd_word = CMD_MOVE | ((axis as u32) << 24) | (steps & 0x00FF_FFFF);
    let param_word = (speed & 0xFFFF) | ((dir as u32) << 16);
    fifo.write(cmd_word);
    fifo.write(param_word);
}
```

### 步骤 4: 轴完成通知改为 Doorbell

```rust
// Core 1 端: 运动完成时通知 Core 0
// (在 stepper.rs 的 AxisState 完成回调中)
fn on_motion_done() {
    Doorbell::new().signal();  // 触发 Core 0 的 SIO_IRQ_BELL
}

// Core 0 端: SIO_IRQ_BELL ISR
fn SIO_IRQ_BELL() {
    let db = Doorbell::new();
    db.clear_local(1);
    // 检查哪些轴完成，推送 EVT AXIS_DONE 给上位机
    interrupt::pend(Interrupt::SPARE_IRQ_0);
}
```

---

## 6. AxisState 跨核安全性分析

**无需任何修改。** AxisState 的所有字段已经是原子类型：

| 字段 | 类型 | 写者 (Core 1) | 读者 (Core 0) |
|------|------|--------------|--------------|
| `position` | `AtomicI32` | Timer ISR 每步更新 | 状态查询直读 |
| `steps_done` | `AtomicU32` | Timer ISR 每步更新 | 状态查询直读 |
| `phase` | `AtomicU8` (MotionPhase) | Timer ISR 状态转换 | 完成检测直读 |
| `running` | `AtomicBool` | start/stop 时写 | 状态查询直读 |

Cortex-M33 的原子指令保证跨核可见性，`Ordering::Relaxed` 足够（单生产者-单消费者模式）。

---

## 7. 注意事项

### 7.1 Timer0 竞争

Timer0 只有一个外设实例，但有 4 个独立 alarm。改造后：
- **Core 1** 使用 alarm 0/1/2（X/Y/Z 轴 STEP 脉冲），通过 `Timer0::steal()` 访问
- alarm 的读写是寄存器级别操作，不同 alarm 之间无竞争

### 7.2 GPIO 外设

GPIO 控制寄存器（SET/CLR/XOR）是原子的（addr+0x2000 = SET，addr+0x3000 = CLR），
**不同引脚之间没有竞争**。Core 1 操作 STEP/DIR 引脚，Core 0 操作 ENABLE/PWM，互不干扰。

### 7.3 TMC2209 UART

UART1 仅在 Core 0 的 SPARE_IRQ_0 中访问（配置 + 归位前清堵转标志），
**不需要额外的跨核同步**。

### 7.4 defmt 日志

pico-rs 的 critical-section 实现使用 **Spinlock-31 + 每核嵌套计数**，
defmt-rtt 通过 `critical_section::with()` 保护 RTT 缓冲区。
**两核可以同时安全使用 `info!()` 等宏**，spinlock 自动串行化。

唯一的性能考量：两核高频同时打日志会互相自旋等待。实践中运动核 Timer ISR 不打日志，不是问题。

### 7.5 Spinlock 分配

| Spinlock | 用途 |
|----------|------|
| **31** | 已被 critical-section 占用 |
| **0~30** | 可自由使用（如需共享命令缓冲区等） |

### 7.6 Core 1 栈大小

4096 字节对于轴控制的中断处理足够。如果 Core 1 有复杂的回零逻辑或深调用链，可增大到 8192。

---

## 8. 改造工作量评估

| 改动项 | 范围 | 复杂度 |
|--------|------|--------|
| 添加 Core 1 入口函数 + 栈 | `main.rs` | 低 |
| 中断 enable 按核分配 | `main.rs` | 低 |
| 运动命令改为 FIFO 传递 | `main.rs` 命令分发部分 | 中 |
| 轴完成通知改为 Doorbell | `stepper.rs` + `main.rs` | 中 |
| AxisState 适配 | 无需改动 | — |
| USB/CDC 逻辑 | 无需改动（仍在 Core 0） | — |
| TMC2209 UART | 无需改动（仍在 Core 0） | — |

**整体：中等规模重构，架构清晰，风险可控。**
