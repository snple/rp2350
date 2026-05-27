# USB HID Report Descriptor 详解

USB HID Report Descriptor（报告描述符）是一段紧凑的二进制数据，用于告诉主机（PC）：
- 设备会发送/接收什么格式的数据
- 每个字段的含义、大小和取值范围

它看起来就是一堆十六进制字节，但实际上是一种 **结构化的标签-值编码**。

---

## 1. 基本编码格式

每条"指令"由 **前缀字节（prefix byte）** + **数据字节** 组成：

```
前缀字节: [bTag (4bit)] [bType (2bit)] [bSize (2bit)]
          高位 ←――――――――――――――――――――――――→ 低位
```

| 字段    | 位数 | 说明                                        |
|---------|------|---------------------------------------------|
| bSize   | 2bit | 后跟数据字节数：0=0字节, 1=1字节, 2=2字节, 3=4字节 |
| bType   | 2bit | 类型：0=Main, 1=Global, 2=Local, 3=保留      |
| bTag    | 4bit | 具体标签（Usage Page、Usage、Input 等）       |

### 示例拆解

```
0x05, 0x01
```
- 前缀 `0x05` = 二进制 `0000 0101`
  - bSize = `01` → 后跟 **1** 字节数据
  - bType = `01` → **Global** 类型项
  - bTag  = `0000` → **Usage Page**
- 数据 `0x01` = Generic Desktop

所以 `0x05, 0x01` = **Usage Page (Generic Desktop)**

---

## 2. 常用标签速查表

### Main 类型 (bType = 0)

| 前缀字节 | 标签名          | 含义                       |
|----------|----------------|----------------------------|
| `0x81`   | Input          | 声明一个输入报告字段        |
| `0x91`   | Output         | 声明一个输出报告字段        |
| `0xB1`   | Feature        | 声明一个特征报告字段        |
| `0xA1`   | Collection     | 开始一个集合               |
| `0xC0`   | End Collection | 结束一个集合               |

### Global 类型 (bType = 1)

| 前缀字节    | 标签名           | 含义                         |
|------------|-----------------|-------------------------------|
| `0x05`     | Usage Page      | 用途页面（功能大类）           |
| `0x15`     | Logical Minimum | 逻辑值下界                    |
| `0x25`     | Logical Maximum | 逻辑值上界（1字节）           |
| `0x26`     | Logical Maximum | 逻辑值上界（2字节）           |
| `0x75`     | Report Size     | 每个字段占多少 **bit**        |
| `0x95`     | Report Count    | 有多少个这样的字段            |

### Local 类型 (bType = 2)

| 前缀字节 | 标签名         | 含义                         |
|----------|---------------|-------------------------------|
| `0x09`   | Usage         | 具体用途（功能小类）           |
| `0x19`   | Usage Minimum | 用途范围下界                  |
| `0x29`   | Usage Maximum | 用途范围上界                  |

---

## 3. Input / Output 标志位

`Input`、`Output`、`Feature` 后面的数据字节是一个 **位掩码**，描述字段属性：

| 位   | 值=0          | 值=1            |
|------|---------------|-----------------|
| bit0 | Data（数据）   | Constant（常量） |
| bit1 | Array（数组）  | Variable（变量） |
| bit2 | Absolute（绝对）| Relative（相对） |

常见组合：

| 值     | 含义                              | 典型用途              |
|--------|-----------------------------------|-----------------------|
| `0x00` | Data, Array, Absolute             | 键盘的按键数组         |
| `0x01` | Constant                          | 填充/保留字段          |
| `0x02` | Data, Variable, Absolute          | 修饰键位、LED 状态     |
| `0x06` | Data, Variable, Relative          | 鼠标 X/Y/滚轮         |

---

## 4. Usage Page 常用值

| 值     | Usage Page 名称   | 设备类型                      |
|--------|-------------------|-------------------------------|
| `0x01` | Generic Desktop   | 键盘、鼠标、游戏手柄           |
| `0x07` | Keyboard/Keypad   | 键盘按键码                    |
| `0x08` | LEDs              | 键盘 LED（Num Lock 等）       |
| `0x09` | Buttons           | 鼠标/游戏手柄按钮             |
| `0x0C` | Consumer          | 多媒体按键（音量、播放等）     |

---

## 5. Collection 类型

`Collection(x)` 中 x 的含义：

| 值     | 类型         | 用途                           |
|--------|-------------|--------------------------------|
| `0x00` | Physical    | 物理组合（鼠标的指针）          |
| `0x01` | Application | 应用级（一个完整设备功能）       |
| `0x02` | Logical     | 数据的逻辑分组                  |

---

## 6. 完整示例：键盘描述符逐行解读

```rust
pub const KEYBOARD_REPORT_DESCRIPTOR: &[u8] = &[
    // ── 声明这是一个"键盘"设备 ──
    0x05, 0x01,       // Usage Page (Generic Desktop)      → 通用桌面设备
    0x09, 0x06,       // Usage (Keyboard)                  → 具体是键盘
    0xA1, 0x01,       // Collection (Application)          → 开始一个应用集合

    // ── 第1部分：修饰键（Ctrl/Shift/Alt/GUI × 左右）= 8个1-bit字段 ──
    0x05, 0x07,       // Usage Page (Keyboard/Keypad)      → 按键用途页
    0x19, 0xE0,       // Usage Minimum (0xE0 = Left Ctrl)  → 从左Ctrl开始
    0x29, 0xE7,       // Usage Maximum (0xE7 = Right GUI)  → 到右GUI结束
    0x15, 0x00,       // Logical Minimum (0)               → 值范围 0
    0x25, 0x01,       // Logical Maximum (1)               → 值范围 1（按下/释放）
    0x75, 0x01,       // Report Size (1)                   → 每个字段 1 bit
    0x95, 0x08,       // Report Count (8)                  → 共 8 个字段
    0x81, 0x02,       // Input (Data, Variable, Absolute)  → 输入：可变数据
    // → 报告第 1 字节：[LCtrl|LShift|LAlt|LGUI|RCtrl|RShift|RAlt|RGUI]

    // ── 第2部分：保留字节（OEM 使用，通常全零）= 1个8-bit常量字段 ──
    0x75, 0x08,       // Report Size (8)                   → 8 bit
    0x95, 0x01,       // Report Count (1)                  → 1 个字段
    0x81, 0x01,       // Input (Constant)                  → 常量，主机忽略
    // → 报告第 2 字节：0x00（保留）

    // ── 第3部分：LED 输出（Num Lock/Caps Lock 等）= 5个1-bit字段 + 3-bit填充 ──
    0x05, 0x08,       // Usage Page (LEDs)                 → LED 用途页
    0x19, 0x01,       // Usage Minimum (Num Lock)          → 从 Num Lock 开始
    0x29, 0x05,       // Usage Maximum (Kana)              → 到 Kana 结束
    0x75, 0x01,       // Report Size (1)                   → 每个 1 bit
    0x95, 0x05,       // Report Count (5)                  → 共 5 个 LED
    0x91, 0x02,       // Output (Data, Variable, Absolute) → 输出报告
    0x75, 0x03,       // Report Size (3)                   → 3 bit 填充
    0x95, 0x01,       // Report Count (1)
    0x91, 0x01,       // Output (Constant)                 → 填充到整字节
    // → 输出报告 1 字节：[NumLock|CapsLock|ScrollLock|Compose|Kana|pad|pad|pad]

    // ── 第4部分：按键数组 = 6个8-bit字段，每个是一个 keycode ──
    0x05, 0x07,       // Usage Page (Keyboard/Keypad)
    0x19, 0x00,       // Usage Minimum (0)                 → keycode 范围起始
    0x29, 0xFF,       // Usage Maximum (255)               → keycode 范围结束
    0x15, 0x00,       // Logical Minimum (0)
    0x26, 0xFF, 0x00, // Logical Maximum (255)             → 2字节形式
    0x75, 0x08,       // Report Size (8)                   → 每个 8 bit
    0x95, 0x06,       // Report Count (6)                  → 共 6 个键位
    0x81, 0x00,       // Input (Data, Array)               → 数组方式
    // → 报告第 3~8 字节：最多同时 6 个按下的 keycode

    0xC0,             // End Collection
];
```

### 键盘报告的数据格式（8 字节）

```
字节 0: 修饰键位图   [LCtrl|LShift|LAlt|LGUI|RCtrl|RShift|RAlt|RGUI]
字节 1: 保留 (0x00)
字节 2: 按键 1 的 keycode
字节 3: 按键 2 的 keycode
字节 4: 按键 3 的 keycode
字节 5: 按键 4 的 keycode
字节 6: 按键 5 的 keycode
字节 7: 按键 6 的 keycode
```

---

## 7. 完整示例：鼠标描述符逐行解读

```rust
pub const MOUSE_REPORT_DESCRIPTOR: &[u8] = &[
    // ── 声明这是一个"鼠标"设备 ──
    0x05, 0x01,       // Usage Page (Generic Desktop)
    0x09, 0x02,       // Usage (Mouse)
    0xA1, 0x01,       // Collection (Application)
    0x09, 0x01,       //   Usage (Pointer)                 → 鼠标下的"指针"
    0xA1, 0x00,       //   Collection (Physical)           → 物理集合

    // ── 按钮（3个1-bit + 5-bit填充）──
    0x05, 0x09,       //   Usage Page (Buttons)
    0x19, 0x01,       //   Usage Minimum (Button 1 = 左键)
    0x29, 0x03,       //   Usage Maximum (Button 3 = 中键)
    0x15, 0x00,       //   Logical Minimum (0)
    0x25, 0x01,       //   Logical Maximum (1)
    0x75, 0x01,       //   Report Size (1)
    0x95, 0x03,       //   Report Count (3)                → 3 个按钮
    0x81, 0x02,       //   Input (Data, Variable, Absolute)
    0x75, 0x05,       //   Report Size (5)
    0x95, 0x01,       //   Report Count (1)
    0x81, 0x01,       //   Input (Constant)                → 5-bit 填充
    // → 字节 0: [Btn1|Btn2|Btn3|0|0|0|0|0]

    // ── X / Y 相对移动 ──
    0x05, 0x01,       //   Usage Page (Generic Desktop)
    0x09, 0x30,       //   Usage (X)
    0x09, 0x31,       //   Usage (Y)
    0x15, 0x81,       //   Logical Minimum (-127)
    0x25, 0x7F,       //   Logical Maximum (127)
    0x75, 0x08,       //   Report Size (8)                 → 每个 8 bit
    0x95, 0x02,       //   Report Count (2)                → X 和 Y 两个值
    0x81, 0x06,       //   Input (Data, Variable, Relative) → 相对值
    // → 字节 1: X 偏移 (-127 ~ 127)
    // → 字节 2: Y 偏移 (-127 ~ 127)

    // ── 滚轮 ──
    0x09, 0x38,       //   Usage (Wheel)
    0x15, 0x81,       //   Logical Minimum (-127)
    0x25, 0x7F,       //   Logical Maximum (127)
    0x75, 0x08,       //   Report Size (8)
    0x95, 0x01,       //   Report Count (1)
    0x81, 0x06,       //   Input (Data, Variable, Relative)
    // → 字节 3: 滚轮偏移 (-127 ~ 127)

    0xC0,             //   End Collection (Physical)
    0xC0,             // End Collection (Application)
];
```

### 鼠标报告的数据格式（4 字节）

```
字节 0: 按钮位图  [Left|Right|Middle|0|0|0|0|0]
字节 1: X 轴相对偏移 (i8, -127 ~ 127)
字节 2: Y 轴相对偏移 (i8, -127 ~ 127)
字节 3: 滚轮相对偏移 (i8, -127 ~ 127)
```

---

## 8. 如何定义自定义报告描述符

### 步骤

1. **确定设备类型** → 选择合适的 Usage Page + Usage
2. **规划报告结构** → 列出每个字段的含义、位数、取值范围
3. **逐段编码** → 使用上面的标签表写出字节序列
4. **计算报告大小** → 所有 Input 字段的 bit 总和必须是 8 的整倍数

### 自定义示例：游戏手柄（2轴 + 4按钮）

```rust
pub const GAMEPAD_REPORT_DESCRIPTOR: &[u8] = &[
    0x05, 0x01,       // Usage Page (Generic Desktop)
    0x09, 0x05,       // Usage (Gamepad)
    0xA1, 0x01,       // Collection (Application)

    // 4 个按钮
    0x05, 0x09,       //   Usage Page (Buttons)
    0x19, 0x01,       //   Usage Minimum (Button 1)
    0x29, 0x04,       //   Usage Maximum (Button 4)
    0x15, 0x00,       //   Logical Minimum (0)
    0x25, 0x01,       //   Logical Maximum (1)
    0x75, 0x01,       //   Report Size (1)
    0x95, 0x04,       //   Report Count (4)
    0x81, 0x02,       //   Input (Data, Variable, Absolute)
    // 4-bit 填充，凑满 1 字节
    0x75, 0x04,       //   Report Size (4)
    0x95, 0x01,       //   Report Count (1)
    0x81, 0x01,       //   Input (Constant)

    // X 轴 和 Y 轴 (各 8bit, 有符号)
    0x05, 0x01,       //   Usage Page (Generic Desktop)
    0x09, 0x30,       //   Usage (X)
    0x09, 0x31,       //   Usage (Y)
    0x15, 0x81,       //   Logical Minimum (-127)
    0x25, 0x7F,       //   Logical Maximum (127)
    0x75, 0x08,       //   Report Size (8)
    0x95, 0x02,       //   Report Count (2)
    0x81, 0x02,       //   Input (Data, Variable, Absolute)

    0xC0,             // End Collection
];
// 报告大小 = 4 + 4 + 8 + 8 = 24 bit = 3 字节
```

使用方式：

```rust
let gamepad = HidClass::new(&alloc, HidConfig::custom(GAMEPAD_REPORT_DESCRIPTOR, 3));
```

---

## 9. 常见注意事项

| 要点 | 说明 |
|------|------|
| **bit 对齐** | 所有 Input/Output 字段的总 bit 数必须是 8 的倍数，不够用 Constant 填充 |
| **Global 状态** | Usage Page / Logical Min/Max / Report Size/Count 是 **全局状态**，设置后一直有效，直到被下一个同类标签覆盖 |
| **Local 状态** | Usage / Usage Min/Max 是 **局部状态**，每次 Main 项（Input/Output）消费后自动清零 |
| **多字节值** | 用 `0x26`（2字节 Logical Max）表示超过 127 的值，如 `0x26, 0xFF, 0x00` = 255 |
| **有符号值** | `0x15, 0x81` = Logical Minimum(-127)，因为 `0x81` 是 `i8` 的 -127 |
| **Report ID** | 多报告设备需加 `0x85, <id>` 声明 Report ID，此时报告数据前会多 1 字节 ID |

---

## 10. 参考资料

- [USB HID 规范 1.11](https://www.usb.org/document-library/device-class-definition-hid-111)
- [HID Usage Tables](https://www.usb.org/document-library/hid-usage-tables-14)
- [USB HID Report Descriptor Tutorial (Frank Zhao)](https://eleccelerator.com/tutorial-about-usb-hid-report-descriptors/)
