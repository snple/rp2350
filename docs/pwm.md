# RP2350 PWM 使用与计算说明

本文整理 pico-rs 当前 PWM HAL 的设计、PWM 输出和输入模式的工作方式，以及频率、占空比、测速等相关计算。

本文的目标不是简单重复 API 名称，而是把下面几件事讲清楚：

- PWM slice 的硬件模型到底是什么
- 输出模式下频率和占空比如何计算
- 输入模式下计数器到底在数什么
- 如何从计数值换算出频率、占空比、RPM
- 哪些参数决定分辨率、误差和溢出边界
- pico-rs 当前 PWM API 适合怎么使用

---

## 1. 基本概念

RP2350 的 PWM 外设由多个 slice 组成。

- RP2350A: 8 个 slice，编号 0 到 7
- RP2350B: 12 个 slice，编号 0 到 11
- 每个 slice 有两个通道：A 和 B

每个 slice 的核心资源包括：

- 一个 16 位计数器
- 一个 8.4 定点分频器 `DIV`
- 一个 16 位 `TOP`/`WRAP` 寄存器
- 一个 32 位 `CC` 寄存器，低 16 位是 A 通道 compare，高 16 位是 B 通道 compare
- 一组控制位，例如使能、反相、phase-correct、div mode 等

最重要的一点是：

**A 和 B 不是两个独立的 PWM 定时器，它们共享同一个 slice 的计数器、分频器、TOP 和模式位。**

这意味着：

- A/B 两个通道的基础频率总是相同的
- A/B 可以有不同的 compare 值，因此占空比可以不同
- 如果把 slice 切到输入模式，影响的是这个共享计数器的推进方式

这也是为什么“一个 slice 同时做普通 PWM 输出和独立 tach 测速”通常不是正确模型。

---

## 2. GPIO 与 Slice 的映射

GPIO 到 PWM slice 和通道的映射规律是：

- `slice = (gpio_num / 2) % 8`，用于 RP2350A
- `slice = (gpio_num / 2) % 12`，用于 RP2350B
- `channel = gpio_num % 2`，偶数脚是 A，奇数脚是 B

例如：

- GPIO0 -> slice 0, channel A
- GPIO1 -> slice 0, channel B
- GPIO2 -> slice 1, channel A
- GPIO3 -> slice 1, channel B

因此：

- 要做 A 通道输出，必须选择该 slice 对应的 A 脚
- 要做 B 通道输入，必须选择该 slice 对应的 B 脚

---

## 3. PWM 输出模式的硬件模型

### 3.1 输出模式下有哪些关键参数

输出模式下最关键的几个参数是：

- `sys_clock`: PWM 输入时钟，通常取 `clocks.sys_freq()`
- `DIV`: 分频器，格式为 `div_int + div_frac / 16`
- `TOP`: 计数器周期上限
- `CC`: compare 值，决定高电平持续时间

可以按这个顺序理解：

1. `sys_clock` 经过 `DIV` 得到计数时钟
2. 计数器按这个时钟推进
3. 计数器走完一个周期所需的步数由 `TOP` 决定
4. 输出在 `counter < CC` 时为高，因此 `CC` 决定占空比

### 3.2 边沿对齐模式

在普通 edge-aligned 模式下，计数器从 `0` 计到 `TOP`，然后回到 `0`。

因此一个 PWM 周期包含：

```text
TOP + 1
```

个计数步。

频率公式为：

```text
f_pwm = f_sys / (DIV * (TOP + 1))
```

其中：

```text
DIV = div_int + div_frac / 16
```

### 3.3 相位修正模式

在 phase-correct 模式下，计数器先从 `0` 计到 `TOP`，再反向计回去。

按 pico-rs 当前 HAL 的计算模型，一个完整 PWM 周期包含：

```text
2 * TOP
```

个计数步。

对应公式为：

```text
f_pwm = f_sys / (DIV * TOP * 2)
```

当前 HAL 中：

- `calc_divider_phase_correct`
- `calc_phase_correct_frequency`
- `configure_frequency_with_wrap` 在 phase-correct 模式下的分支

都使用这一套公式。

### 3.4 占空比如何计算

在 edge-aligned 模式下，输出条件是：

```text
counter < CC
```

因此占空比最直观的近似是：

```text
duty ~= CC / (TOP + 1)
```

例如 `TOP = 999` 时：

- `CC = 0` -> 0%
- `CC = 250` -> 25%
- `CC = 500` -> 50%
- `CC = 750` -> 75%
- `CC = 1000` -> 100%

### 3.5 为什么不要把 TOP 设成 0xffff

要得到真正的 100% 占空比，需要：

```text
CC = TOP + 1
```

但 `CC` 只有 16 位。如果：

```text
TOP = 0xffff
```

那么理论上的满占空比 compare 值就会变成 `0x10000`，已经超出 16 位范围。

这就是为什么当前 HAL 默认把 `TOP` 初始化为：

```text
0xfffe
```

这样就还能用：

```text
CC = 0xffff
```

表示全开。

这个问题对输出模式尤其重要，做 LED 调光、电机控制、风扇控制时都应该记住。

---

## 4. 分频器与量化误差

PWM 的分频器不是任意浮点数，而是 8.4 定点格式：

```text
DIV = div_int + div_frac / 16
```

也就是说：

- 整数部分范围约为 1 到 255
- 小数部分只能按 `1/16` 的步进变化

因此请求一个目标频率时，往往无法得到完全精确的 divider，只能得到最接近的硬件可表示值。

这就是为什么 HAL 里的高层 helper：

- `configure_frequency`
- `configure_frequency_with_wrap`

都会返回“实际编程后的频率”，而不是只接受“目标频率”就当作完全精确。

理解这一点很重要：

1. 你先给出 `sys_clock`、目标频率和 `TOP`
2. HAL 计算出理想 divider
3. 理想 divider 被截断或量化为 8.4 定点可表示值
4. 再由这个量化后的 divider 反算出实际频率

如果对频率精度很敏感，就不要只看目标值，应该检查 helper 返回的实际值。

---

## 5. 输出模式下如何选参数

输出模式最实用的设计顺序是：

1. 先选 `TOP`
2. 再根据目标频率计算 `DIV`
3. 最后根据占空比计算 `CC`

### 5.1 为什么先选 TOP

`TOP` 既影响频率，也影响占空比分辨率。

更准确地说：

- `TOP` 越大，一个周期内可用的 duty 步数越多
- `TOP` 越小，可达到的频率越高

因此：

- 做 LED 调光时，通常希望 `TOP` 大一点，这样亮度变化更平滑
- 做电机或风扇控制时，常常优先要较高 PWM 频率，这时 `TOP` 可能需要更小
- 做舵机控制时，周期长且脉宽精度重要，通常会选较大的 `TOP`

### 5.2 常见场景的典型参数

假设 `sys_clock = 150 MHz`：

| 场景 | 目标频率 | TOP | DIV | 说明 |
|---|---:|---:|---:|---|
| LED 调光 | 1 kHz | 999 | 150.0 | 1000 级占空比分辨率 |
| 舵机 | 50 Hz | 65217 | 约 46.0 | 20 ms 周期，脉宽分辨率细 |
| 电机 PWM | 20 kHz | 7499 | 1.0 | 高频控制常见配置 |
| 4 线风扇控制 | 25 kHz | 5999 | 1.0 | 常见 PC 风扇控制频率 |

### 5.3 例 1：1 kHz LED 调光

目标：

- 系统时钟 `150 MHz`
- PWM 频率 `1 kHz`
- 1000 个 duty 步进

先设：

```text
TOP = 999
```

因为一个周期需要 `TOP + 1 = 1000` 个计数步。

然后：

```text
DIV = 150_000_000 / (1_000 * 1000) = 150
```

所以：

- `div_int = 150`
- `div_frac = 0`

如果要 50% 占空比：

```text
CC = 500
```

### 5.4 例 2：50 Hz 舵机控制

目标：

- 周期 20 ms
- 占空比不是核心，核心是脉宽 1 ms 到 2 ms

假设：

```text
TOP = 65217
DIV = 46
```

那么频率大约为：

```text
f_pwm = 150_000_000 / (46 * (65217 + 1)) ≈ 49.9995 Hz
```

一个周期大约 20 ms，1 ms、1.5 ms、2 ms 脉宽对应的 compare 值大约为：

```text
1.0 ms  -> 3261
1.5 ms  -> 4891
2.0 ms  -> 6522
```

### 5.5 例 3：25 kHz 4 线风扇控制

假设：

- `sys_clock = 150 MHz`
- 目标频率 `25 kHz`
- 取 `TOP = 5999`

则：

```text
DIV = 150_000_000 / (25_000 * (5999 + 1)) = 1
```

这是一个非常整齐的配置。

如果要 30% 占空比，近似可取：

```text
CC = 6000 * 0.3 = 1800
```

---

## 6. 当前 HAL 的输出 API 应该怎么理解

当前 pico-rs PWM HAL 里，builder 方法负责的是：

- 绑定引脚
- 初始化 slice
- 设置一个安全的默认状态

它们不负责根据系统时钟帮你自动决定频率。

也就是说，类似：

- `output_a(pin)`
- `output_ab(pin_a, pin_b)`

这些方法的职责是“建 slice 并绑定引脚”，不是“完成频率设计”。

频率和占空比相关的配置，应使用两层 API：

### 6.1 底层寄存器风格 API

- `set_wrap`
- `set_div_int`
- `set_div_frac`
- `set_div`
- `set_duty_a`
- `set_duty_b`
- `set_duty`

适合你已经完全知道目标参数时直接写寄存器。

### 6.2 高频率配置 helper

- `configure_frequency`
- `configure_frequency_with_wrap`
- `get_frequency`

适合这种场景：

- 你已经知道 `TOP`
- 想要一个目标频率
- 希望 HAL 帮你算最接近的 divider

推荐用法通常是：

1. 先决定 `TOP`
2. 用 `configure_frequency_with_wrap(sys_clock, target_freq, top)` 配置
3. 再设置 `CC`
4. 最后启用输出

---

## 7. PWM 输入模式到底是什么

输入模式最容易被误解。

它不是传统意义上的“输入捕获”。

它**不会在每个边沿帮你记录时间戳**，也**不会帮你直接测出单个脉冲的宽度**。它做的事情更简单、更底层：

**让 B 引脚参与控制 slice 计数器如何前进。**

当前 HAL 的输入模式有三种：

- `InputMode::RisingEdge`
- `InputMode::FallingEdge`
- `InputMode::GatedByB`

### 7.1 RisingEdge

每出现一个 B 引脚上升沿，计数器加 1。

因此在一段已知门控时间之后：

- 计数值表示边沿数量
- 可以用来估算频率、脉冲数、转速

### 7.2 FallingEdge

和 `RisingEdge` 相同，只是改成对下降沿计数。

适合输入信号下降沿更干净、抖动更小的场景。

### 7.3 GatedByB

只有当 B 引脚为高电平时，计数器才会按 slice 的计数时钟推进。

因此一段已知门控时间之后：

- 计数值表示高电平累计持续了多少计数 tick
- 可以用来估算 duty 或高电平持续时间占比

这就是为什么：

- 边沿模式下，计数结果通常要用 `calc_input_frequency`
- 门控模式下，计数结果通常要用 `calc_duty_cycle` 或手工换算

---

## 8. 输入模式下计数器到底在数什么

这是理解输入模式最关键的一点。

### 8.1 边沿模式

在 `RisingEdge` 或 `FallingEdge` 模式里：

- 计数器的值就是“门控时间内发生了多少个边沿”

这时：

- `sys_clock` 不直接决定你数到了几个边沿
- `DIV` 也不是频率换算的核心
- 核心参数是门控时间

### 8.2 门控模式

在 `GatedByB` 模式里：

- 计数器只会在 B 为高时继续按计数时钟推进

因此：

- 计数值是高电平累计时间对应的 tick 数
- `sys_clock` 和 `DIV` 直接影响测量结果

这时必须知道计数时钟：

```text
f_ctr = f_sys / DIV
```

---

## 9. 输入模式的计算

### 9.1 边沿计数模式下的频率计算

如果门控时间是：

```text
T_gate
```

秒，计数得到的边沿数是：

```text
N_edges
```

则频率为：

```text
f_in = N_edges / T_gate
```

如果门控时间单位是微秒：

```text
f_in = N_edges * 1_000_000 / gate_time_us
```

这正是 `calc_input_frequency(edge_count, gate_time_us)` 的含义。

#### 例子：

- 门控时间 `100 ms`
- 数到 `240` 个上升沿

则：

```text
f_in = 240 / 0.1 = 2400 Hz
```

### 9.2 用边沿计数测风扇转速

如果风扇 tach 每转输出 `P` 个脉冲，则：

```text
rpm = f_tach * 60 / P
```

把上面的频率公式代入：

```text
rpm = edge_count * 60 * 1_000_000 / (gate_time_us * pulses_per_rev)
```

例如：

- tach 每转 2 脉冲
- 门控时间 250 ms
- 数到 10 个上升沿

则：

```text
f_tach = 10 / 0.25 = 40 Hz
rpm = 40 * 60 / 2 = 1200 rpm
```

### 9.3 门控模式下的 duty 计算

门控模式下必须先知道计数时钟：

```text
f_ctr = f_sys / DIV
```

假设门控时间为：

```text
T_gate
```

秒，则理论上这段时间内总共可以走的 tick 数约为：

```text
N_total = f_ctr * T_gate
```

如果实际测得高电平累计 tick 为：

```text
N_high
```

则 duty 为：

```text
duty = N_high / N_total
```

百分比形式：

```text
duty_percent = N_high * 100 / N_total
```

如果门控时间用微秒表示，则可写成：

```text
N_total = (f_sys / DIV) * gate_time_us / 1_000_000
```

### 9.4 `calc_duty_cycle` 的含义和限制

当前 HAL 中的 `calc_duty_cycle(high_count, sys_clk, divider, gate_time_us)` 用的是整数 divider。

也就是说，它更适合：

- `div_frac = 0`
- 或者你只需要一个近似结果

如果你在门控模式下使用了非零 `div_frac`，更准确的做法是手工按完整 divider：

```text
DIV = div_int + div_frac / 16
```

来换算。

#### 例子：

假设：

- `sys_clock = 12 MHz`
- `DIV = 12`
- 则计数时钟 `f_ctr = 1 MHz`
- 门控时间 `1 ms`
- 测得 `high_count = 400`

则：

```text
N_total = 1_000_000 * 0.001 = 1000
duty = 400 / 1000 = 40%
```

---

## 10. 输入模式下的分辨率与误差

### 10.1 边沿计数模式的分辨率

边沿计数模式的分辨率主要由门控时间决定。

如果门控时间是：

```text
T_gate
```

那么最基础的频率分辨率大约是：

```text
Δf ≈ 1 / T_gate
```

例如：

- `10 ms` 门控，分辨率约 `100 Hz`
- `100 ms` 门控，分辨率约 `10 Hz`
- `1 s` 门控，分辨率约 `1 Hz`

因此：

- 窗口越短，更新越快，但结果更粗、更抖
- 窗口越长，结果更稳，但更新更慢

### 10.2 门控模式的分辨率

门控模式的时间分辨率由计数时钟决定：

```text
resolution_time ≈ 1 / f_ctr
```

因此：

- divider 越小，`f_ctr` 越高，时间分辨率越高
- divider 越大，分辨率变差，但更不容易溢出

门控模式本质上是在“时间分辨率”和“最长可测窗口”之间做权衡。

---

## 11. 输入模式下的溢出边界

PWM slice 计数器只有 16 位，因此最大计数值只有 `65535`。

### 11.1 边沿模式的溢出

边沿计数模式中，如果：

```text
edge_count > 65535
```

就会发生回卷。

因此在门控时间为 `T_gate` 秒时，频率上限粗略估算为：

```text
f_max ≈ 65535 / T_gate
```

例如 `100 ms` 门控：

```text
f_max ≈ 65535 / 0.1 = 655350 Hz
```

### 11.2 门控模式的溢出

门控模式的最坏情况是输入始终为高，这时计数器会以 `f_ctr` 全速推进。

因此要求：

```text
f_ctr * T_gate <= 65535
```

也就是：

```text
T_gate <= 65535 / f_ctr
```

代入 `f_ctr = f_sys / DIV` 可得：

```text
T_gate <= 65535 * DIV / f_sys
```

这条公式非常有用。

例如：

- `sys_clock = 12 MHz`
- `DIV = 1`

则最大安全门控时间约为：

```text
T_gate <= 65535 / 12_000_000 ≈ 5.46 ms
```

这意味着如果你在 `GatedByB` 模式下：

- 用 `DIV = 1`
- 测量 `10 ms`

那么高 duty 或 100% 高电平的情况下很容易溢出。

因此门控模式的设计通常要在两者之间选一个：

- 缩短 gate time
- 或者增大 divider

---

## 12. `output_a_input()` 该怎么理解

`output_a_input()` 看起来像“输出 + 输入”同时开启，但它不是两个独立外设的叠加。

它的真实含义是：

- A 通道仍然作为输出
- B 通道作为外部输入源参与控制同一个 slice 的计数器

因此它只适合这种场景：

- 你就是希望 A 输出和 B 输入共用同一套 slice 时序和计数模型

它不适合这种场景：

- A 想正常输出一个自由运行 PWM
- B 还想独立进行普通 tach 计数或 duty 测量

因为两者本质上要争用的是同一个 slice 的计数器驱动方式。

---

## 13. 4 线风扇控制与测速的推荐方案

4 线风扇通常有两个相关信号：

- 控制输入：常见是 25 kHz PWM
- tach 输出：每转若干脉冲，常见是 2 脉冲/转

推荐使用方式：

- 一个 slice 做普通 PWM 输出，用于控制风扇占空比
- 另一个 slice 做 `RisingEdge` 或 `FallingEdge` 输入计数，用于测速

原因：

- 调速 PWM 需要自由运行的计数器
- tach 测速需要计数器由 B 输入边沿推进
- 这两种需求通常不应该放在同一个 slice 上

如果还考虑电气层面，4 线风扇控制脚常常更接近开漏/集电极开路式驱动要求，这属于 GPIO 输出电气行为问题，不是 PWM 频率计算本身的问题。

---

## 14. 当前 HAL 的输入 API 使用建议

### 14.1 测频率

推荐流程：

1. `pwm.slice::<N>().input(pin_b, InputMode::RisingEdge)`
2. `start_measurement()`
3. 延时一个已知 gate interval
4. `stop_measurement()`
5. 用 `calc_input_frequency()` 换算
6. 检查 `overflow_occurred()`

### 14.2 测 duty

推荐流程：

1. 建立 `input(pin_b, InputMode::GatedByB)`
2. 选择合适的 divider，通常先从较小 divider 开始
3. `start_measurement()`
4. 延时一个已知窗口
5. `stop_measurement()`
6. 用 `calc_duty_cycle()` 或按完整 divider 手算
7. 检查是否 overflow

### 14.3 连续计脉冲

如果需求不是“固定窗口测频率”，而是“持续累计脉冲数”，则可：

1. 设置为 `RisingEdge` 或 `FallingEdge`
2. `reset_counter()`
3. `enable()`
4. 周期性读取 `get_counter()`
5. 检查 `did_wrap()` 并在软件里累计 wrap 次数

---

## 15. 常见误区

### 15.1 误区：输入模式等于输入捕获

不是。

当前 PWM 输入模式是“按外部信号推进计数器”，不是“记录边沿时间戳”。

### 15.2 误区：A/B 是两个独立定时器

不是。

它们共享一个 slice 的计数器、TOP、DIV 和模式配置。

### 15.3 误区：边沿计数模式下 sys_clock 很重要

不完全对。

在 `RisingEdge` / `FallingEdge` 下，频率计算的核心是：

- 边沿数
- 门控时间

`sys_clock` 主要不参与“计了几个边沿”。

### 15.4 误区：门控模式下 divider 可以随便取最小

不对。

divider 越小，时间分辨率越高，但越容易溢出。

### 15.5 误区：`TOP = 0xffff` 是最大分辨率，因此总是最好

不对。

如果你还需要真正的 100% duty，这样会让 `CC = TOP + 1` 无法表示。

---

## 16. 公式汇总

### 16.1 输出模式

边沿对齐频率：

```text
f_pwm = f_sys / (DIV * (TOP + 1))
```

相位修正频率（当前 HAL 模型）：

```text
f_pwm = f_sys / (DIV * TOP * 2)
```

占空比近似：

```text
duty ~= CC / (TOP + 1)
```

### 16.2 输入模式

边沿计数测频率：

```text
f_in = edge_count / T_gate
f_in = edge_count * 1_000_000 / gate_time_us
```

风扇 RPM：

```text
rpm = f_tach * 60 / pulses_per_rev
rpm = edge_count * 60 * 1_000_000 / (gate_time_us * pulses_per_rev)
```

门控模式计数时钟：

```text
f_ctr = f_sys / DIV
```

门控模式总 tick：

```text
N_total = f_ctr * T_gate
```

门控模式 duty：

```text
duty = N_high / N_total
duty_percent = N_high * 100 / N_total
```

边沿模式溢出上限：

```text
f_max ≈ 65535 / T_gate
```

门控模式最大安全门控时间：

```text
T_gate <= 65535 * DIV / f_sys
```

---

## 17. 总结

如果只记住最关键的几条，可以记下面这些：

1. 一个 PWM slice 只有一个计数器，A/B 共享它。
2. 输出模式的核心公式是 `f_pwm = f_sys / (DIV * (TOP + 1))`。
3. 真正的 100% duty 需要 `CC = TOP + 1`，因此不要随手把 `TOP` 设成 `0xffff`。
4. 输入模式不是时间戳捕获，而是“让 B 引脚控制计数器如何前进”。
5. `RisingEdge` / `FallingEdge` 适合测速和计脉冲。
6. `GatedByB` 适合测高电平持续时间和 duty。
7. 门控时间越长，边沿计数越稳；divider 越小，门控测量分辨率越高，但越容易溢出。
8. 4 线风扇的调速输出和 tach 测速，通常应该拆到两个 slice。

如果后续还要扩展，可以在这份文档基础上继续补两类专题：

- 舵机、风扇、电机三类典型输出配置的完整设计流程
- tach 测速和门控测 duty 的完整示例与误差分析
