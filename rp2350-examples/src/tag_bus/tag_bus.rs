//! # TagBus — 嵌入式标签总线协议
//!
//! 适用于 RP2350 (Raspberry Pi Pico 2) 的轻量级工业标签总线协议实现。
//! 基于 UART 物理层，支持标签发现、批量读写。
//!
//! ## 协议概要
//!
//! ```text
//! ┌────────┬────────┬────────┬──────────┬──────────┬────────┬────────┐
//! │  帧头   │ 设备ID │ 功能码  │ 载荷长度  │   载荷    │ CRC16  │  帧尾   │
//! │  2B    │  2B    │  1B    │   2B     │   nB     │  2B    │  2B    │
//! └────────┴────────┴────────┴──────────┴──────────┴────────┴────────┘
//! ```
//!
//! ## 功能码
//!
//! | 码值 | 名称 | 说明 |
//! |------|------|------|
//! | 0x01 | 读标签列表 | 获取标签元数据（支持分页） |
//! | 0x02 | 读输入标签 | 批量读取只读标签当前值 |
//! | 0x03 | 读保持标签 | 批量读取可读写标签当前值 |
//! | 0x05 | 写保持标签 | 批量写入标签值（支持广播 0xFFFF） |
//! | 0x06 | 获取设备信息 | 获取设备厂商/型号/固件版本等 |
//! | 0x07 | 心跳/状态 | 查询设备健康状态、运行统计 |
//!
//! ## 关键特性
//!
//! - 全链路小端字节序，RP2350 原生零开销
//!
//! ## 硬件接线
//!
//! | Pico GPIO | UART 功能 | 方向 |
//! |-----------|----------|------|
//! | GP0 | UART0_TX | Pico → 主机 |
//! | GP1 | UART0_RX | 主机 → Pico |
//!
//! ## 编译运行
//!
//! ```bash
//! cargo run --bin tag_bus --target thumbv8m.main-none-eabihf
//! ```

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;

use rp2350::clocks::ClockBuilder;
use rp2350::gpio::Pins;
use rp2350::main;
use rp2350::pac::Peripherals;
use rp2350::timer::Timer0;
use rp2350::uart::{DataBits, Parity, StopBits, Uart0};

// ============================================================================
// 配置参数
// ============================================================================

/// UART 波特率
const UART_BAUD: u32 = 115_200;

/// 接收缓冲区大小
const RX_BUF_SIZE: usize = 512;

/// 发送缓冲区大小（= 最大载荷 + 帧开销 11 字节）
const TX_BUF_SIZE: usize = MAX_PAYLOAD + 11;

/// 最大标签数量
const MAX_TAGS: usize = 32;

/// 载荷最大长度（帧级，2 字节长度字段）
const MAX_PAYLOAD: usize = 1024;

// ============================================================================
// 帧常量
// ============================================================================

/// 帧头标记
const FRAME_HEADER: u16 = 0xA55A;

/// 帧尾标记
const FRAME_FOOTER: u16 = 0x7E7E;

/// 最小帧长度: 帧头(2) + 设备ID(2) + 功能码(1) + 载荷长度(2) + CRC16(2) + 帧尾(2)
const FRAME_MIN_LEN: usize = 11;

/// 帧头偏移
const OFF_HEADER: usize = 0;

/// 设备 ID 偏移
const OFF_DEVICE_ID: usize = 2;

/// 功能码偏移
const OFF_FUNC: usize = 4;

/// 载荷长度偏移
const OFF_PAYLOAD_LEN: usize = 5;

/// 载荷偏移
const OFF_PAYLOAD: usize = 7;

// ============================================================================
// 功能码
// ============================================================================

/// 读标签列表 — 获取所有标签元数据
pub const FUNC_READ_TAG_LIST: u8 = 0x01;

/// 读输入标签 — 批量读取只读标签当前值
pub const FUNC_READ_INPUT: u8 = 0x02;

/// 读保持标签 — 批量读取可读写标签当前值
pub const FUNC_READ_HOLDING: u8 = 0x03;

/// 写保持标签 — 批量写入可读写标签值
pub const FUNC_WRITE_HOLDING: u8 = 0x05;

/// 获取设备信息
pub const FUNC_DEVICE_INFO: u8 = 0x06;

/// 心跳/设备状态 — 查询设备健康状态与运行统计
pub const FUNC_HEARTBEAT: u8 = 0x07;

/// 广播地址 — 所有设备执行写操作但不回复
pub const BROADCAST_ID: u16 = 0xFFFF;

// ============================================================================
// 设备状态码（心跳响应）
// ============================================================================

/// 设备正常
pub const DEV_STATUS_OK: u8 = 0x00;

/// 设备告警
pub const DEV_STATUS_WARNING: u8 = 0x01;

/// 设备故障
pub const DEV_STATUS_ERROR: u8 = 0x02;

// ============================================================================
// 状态码
// ============================================================================

/// 成功
pub const STATUS_OK: u8 = 0x00;

/// 标签不存在
pub const STATUS_NOT_FOUND: u8 = 0x01;

/// 标签只读
pub const STATUS_READ_ONLY: u8 = 0x02;

/// 类型不匹配
pub const STATUS_TYPE_MISMATCH: u8 = 0x03;

/// 值越界
pub const STATUS_OUT_OF_RANGE: u8 = 0x04;

/// 写入失败
pub const STATUS_WRITE_FAILED: u8 = 0x05;

/// 未知功能码
pub const STATUS_UNKNOWN_FUNC: u8 = 0x06;

// ============================================================================
// 标签属性标志
// ============================================================================

/// 访问模式：只读
pub const TAG_ACCESS_RO: u8 = 0x01;

/// 访问模式：可读写
pub const TAG_ACCESS_RW: u8 = 0x02;

// ============================================================================
// 数据类型编码（与 NSON spec 保持一致）
// ============================================================================

pub const TYPE_BOOL: u8 = 0x01;
pub const TYPE_NULL: u8 = 0x02;
pub const TYPE_FLOAT32: u8 = 0x11;
pub const TYPE_FLOAT64: u8 = 0x12;
pub const TYPE_INT32: u8 = 0x13;
pub const TYPE_INT64: u8 = 0x14;
pub const TYPE_UINT32: u8 = 0x15;
pub const TYPE_UINT64: u8 = 0x16;
pub const TYPE_INT8: u8 = 0x17;
pub const TYPE_UINT8: u8 = 0x18;
pub const TYPE_INT16: u8 = 0x19;
pub const TYPE_UINT16: u8 = 0x1A;
pub const TYPE_STRING: u8 = 0x21;
pub const TYPE_BYTES: u8 = 0x22;
pub const TYPE_TIMESTAMP: u8 = 0x41;
pub const TYPE_ID: u8 = 0x42;

// NSON 定义了但 TagBus 暂不使用的类型码:
//   ARRAY=0x31, MAP=0x32

// ============================================================================
// CRC16 (Modbus 兼容: 多项式 0x8005, 初始值 0xFFFF)
// ============================================================================

/// CRC16 查找表（多项式 0x8005，反射输入/输出）
static CRC16_TABLE: [u16; 256] = {
    let mut table = [0u16; 256];
    let mut i = 0;
    while i < 256 {
        let mut crc = i as u16;
        let mut j = 0;
        while j < 8 {
            if (crc & 0x0001) != 0 {
                crc = (crc >> 1) ^ 0xA001;
            } else {
                crc >>= 1;
            }
            j += 1;
        }
        table[i] = crc;
        i += 1;
    }
    table
};

/// 计算 CRC16
pub fn crc16(data: &[u8]) -> u16 {
    let mut crc: u16 = 0xFFFF;
    for &byte in data {
        let idx = ((crc as u8) ^ byte) as usize;
        crc = (crc >> 8) ^ CRC16_TABLE[idx];
    }
    crc
}

/// 验证帧 CRC: 对 [设备ID..CRC之前] 计算
pub fn verify_crc(frame: &[u8], payload_len: usize) -> bool {
    if frame.len() < OFF_PAYLOAD + payload_len + 2 {
        return false;
    }
    let crc_start = OFF_DEVICE_ID;
    let crc_end = OFF_PAYLOAD + payload_len;
    let computed = crc16(&frame[crc_start..crc_end]);
    let received = u16::from_le_bytes([frame[crc_end], frame[crc_end + 1]]);
    computed == received
}

// ============================================================================
// 标签元数据
// ============================================================================

/// 标签元数据（存储在设备端，通过 0x01 功能码对外发布）
pub struct TagMeta {
    /// 标签 ID
    pub id: u16,
    /// 数据类型编码
    pub data_type: u8,
    /// 访问模式 (TAG_ACCESS_RO / TAG_ACCESS_RW)
    pub access: u8,
    /// 名称长度
    pub name_len: u8,
    /// 标签名称 (UTF-8)
    pub name: [u8; 32],
    /// 扩展属性长度（1 字节）
    pub ext_len: u8,
    /// 扩展属性 (键值对字符串，如 "unit:℃,min:10,max:35")
    pub ext: [u8; 64],
}

impl TagMeta {
    /// 创建新的标签元数据
    pub const fn new(id: u16, data_type: u8, access: u8, name: &str, ext: &str) -> Self {
        let name_bytes = name.as_bytes();
        let ext_bytes = ext.as_bytes();
        let mut name_buf = [0u8; 32];
        let mut ext_buf = [0u8; 64];
        let mut i = 0;
        while i < name_bytes.len() && i < 32 {
            name_buf[i] = name_bytes[i];
            i += 1;
        }
        let mut i = 0;
        while i < ext_bytes.len() && i < 64 {
            ext_buf[i] = ext_bytes[i];
            i += 1;
        }
        Self {
            id,
            data_type,
            access,
            name_len: name_bytes.len() as u8,
            name: name_buf,
            ext_len: ext_bytes.len() as u8,
            ext: ext_buf,
        }
    }

    /// 获取类型对应的固定长度（字符串/bytes 返回 0）
    pub fn type_len(&self) -> u8 {
        type_fixed_len(self.data_type)
    }
}

/// 获取类型对应的固定值长度，变长类型返回 0
pub const fn type_fixed_len(data_type: u8) -> u8 {
    match data_type {
        TYPE_BOOL | TYPE_INT8 | TYPE_UINT8 => 1,
        TYPE_NULL => 0,
        TYPE_INT16 | TYPE_UINT16 => 2,
        TYPE_INT32 | TYPE_UINT32 | TYPE_FLOAT32 => 4,
        TYPE_FLOAT64 | TYPE_INT64 | TYPE_UINT64 | TYPE_TIMESTAMP => 8,
        TYPE_ID => 12,
        _ => 0, // string, bytes
    }
}

// ============================================================================
// 标签值
// ============================================================================

/// 标签运行时值
#[derive(Clone)]
pub enum TagValue {
    Bool(bool),
    Null,
    Int8(i8),
    Uint8(u8),
    Int16(i16),
    Uint16(u16),
    Int32(i32),
    Uint32(u32),
    Float32(f32),
    Float64(f64),
    String([u8; 255], u8), // buffer + length
    Bytes([u8; 255], u8),
    Int64(i64),
    Uint64(u64),
    /// Unix 时间戳，秒（u64, 小端）
    Timestamp(u64),
    /// 12 字节唯一 ID（与 NSON Id 一致: ts[6] + count[2] + random[4]）
    Id([u8; 12]),
}

impl TagValue {
    /// 获取值的字节长度
    pub fn len(&self) -> u8 {
        match self {
            TagValue::Bool(_) | TagValue::Int8(_) | TagValue::Uint8(_) => 1,
            TagValue::Null => 0,
            TagValue::Int16(_) | TagValue::Uint16(_) => 2,
            TagValue::Int32(_) | TagValue::Uint32(_) | TagValue::Float32(_) => 4,
            TagValue::Float64(_)
            | TagValue::Int64(_)
            | TagValue::Uint64(_)
            | TagValue::Timestamp(_) => 8,
            TagValue::Id(_) => 12,
            TagValue::String(_, l) | TagValue::Bytes(_, l) => *l,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 将值序列化为小端字节数组
    pub fn to_bytes(&self, buf: &mut [u8]) -> usize {
        let len = self.len() as usize;
        match self {
            TagValue::Bool(v) => buf[0] = if *v { 0x01 } else { 0x00 },
            TagValue::Int8(v) => buf[0] = *v as u8,
            TagValue::Uint8(v) => buf[0] = *v,
            TagValue::Int16(v) => buf[..2].copy_from_slice(&v.to_le_bytes()),
            TagValue::Uint16(v) => buf[..2].copy_from_slice(&v.to_le_bytes()),
            TagValue::Int32(v) => buf[..4].copy_from_slice(&v.to_le_bytes()),
            TagValue::Uint32(v) => buf[..4].copy_from_slice(&v.to_le_bytes()),
            TagValue::Float32(v) => buf[..4].copy_from_slice(&v.to_le_bytes()),
            TagValue::Float64(v) => buf[..8].copy_from_slice(&v.to_le_bytes()),
            TagValue::Int64(v) => buf[..8].copy_from_slice(&v.to_le_bytes()),
            TagValue::Uint64(v) => buf[..8].copy_from_slice(&v.to_le_bytes()),
            TagValue::Null => { /* 0 bytes */ }
            TagValue::Timestamp(v) => buf[..8].copy_from_slice(&v.to_le_bytes()),
            TagValue::Id(v) => buf[..12].copy_from_slice(v),
            TagValue::String(data, _) | TagValue::Bytes(data, _) => {
                buf[..len].copy_from_slice(&data[..len])
            }
        }
        len
    }

    /// 从小端字节数组解析值
    pub fn from_bytes(data_type: u8, bytes: &[u8], len: u8) -> Option<Self> {
        let n = len as usize;
        if bytes.len() < n {
            return None;
        }
        match data_type {
            TYPE_BOOL => Some(TagValue::Bool(bytes[0] != 0)),
            TYPE_INT8 => Some(TagValue::Int8(bytes[0] as i8)),
            TYPE_UINT8 => Some(TagValue::Uint8(bytes[0])),
            TYPE_INT16 if n >= 2 => Some(TagValue::Int16(i16::from_le_bytes([bytes[0], bytes[1]]))),
            TYPE_UINT16 if n >= 2 => {
                Some(TagValue::Uint16(u16::from_le_bytes([bytes[0], bytes[1]])))
            }
            TYPE_INT32 if n >= 4 => Some(TagValue::Int32(i32::from_le_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3],
            ]))),
            TYPE_UINT32 if n >= 4 => Some(TagValue::Uint32(u32::from_le_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3],
            ]))),
            TYPE_FLOAT32 if n >= 4 => Some(TagValue::Float32(f32::from_le_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3],
            ]))),
            TYPE_FLOAT64 if n >= 8 => Some(TagValue::Float64(f64::from_le_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ]))),
            TYPE_INT64 if n >= 8 => Some(TagValue::Int64(i64::from_le_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ]))),
            TYPE_UINT64 if n >= 8 => Some(TagValue::Uint64(u64::from_le_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ]))),
            TYPE_NULL => Some(TagValue::Null),
            TYPE_TIMESTAMP if n >= 8 => Some(TagValue::Timestamp(u64::from_le_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ]))),
            TYPE_ID if n >= 12 => {
                let mut buf = [0u8; 12];
                buf.copy_from_slice(&bytes[..12]);
                Some(TagValue::Id(buf))
            }
            TYPE_STRING => {
                let mut buf = [0u8; 255];
                let copy_len = if n > 255 { 255 } else { n };
                buf[..copy_len].copy_from_slice(&bytes[..copy_len]);
                Some(TagValue::String(buf, copy_len as u8))
            }
            TYPE_BYTES => {
                let mut buf = [0u8; 255];
                let copy_len = if n > 255 { 255 } else { n };
                buf[..copy_len].copy_from_slice(&bytes[..copy_len]);
                Some(TagValue::Bytes(buf, copy_len as u8))
            }
            _ => None,
        }
    }
}

// ============================================================================
// 设备信息
// ============================================================================

/// 设备信息结构（通过 0x06 功能码获取）
pub struct DeviceInfo {
    /// 协议版本 (主版本, 次版本)，如 (2, 0)
    pub protocol_version: (u8, u8),
    /// 厂商 ID
    pub vendor_id: u16,
    /// 设备型号 ID
    pub product_id: u16,
    /// 固件版本 (year, month, day)，如 (2024, 1, 1)
    pub firmware_version: (u16, u8, u8),
    /// 只读标签数量
    pub ro_tag_count: u16,
    /// 可读写标签数量
    pub rw_tag_count: u16,
    /// 设备名称
    pub name: [u8; 32],
    /// 设备名称长度
    pub name_len: u8,
}

// ============================================================================
// 帧解析结果
// ============================================================================

/// 解析后的帧结构
pub struct ParsedFrame {
    pub device_id: u16,
    pub func_code: u8,
    pub payload: [u8; MAX_PAYLOAD],
    pub payload_len: u16,
}

// ============================================================================
// TagBus 协议引擎
// ============================================================================

/// TagBus v4.1 协议引擎
///
/// 管理标签注册表、处理请求并生成响应。
pub struct TagBus {
    /// 设备 ID
    device_id: u16,
    /// 标签元数据表
    tags: [Option<TagMeta>; MAX_TAGS],
    /// 标签运行时值表（仅 RW 标签需要）
    tag_values: [Option<TagValue>; MAX_TAGS],
    /// 实际标签数量
    tag_count: usize,
    /// 发送缓冲区
    tx_buf: [u8; TX_BUF_SIZE],
    /// 接收缓冲区
    rx_buf: [u8; RX_BUF_SIZE],
    /// 接收缓冲写入位置
    rx_pos: usize,
    // -- 运行统计（心跳） --
    /// 设备状态 (DEV_STATUS_OK / WARNING / ERROR)
    pub status: u8,
    /// 运行秒数
    pub uptime_seconds: u32,
    /// 接收帧计数
    pub frame_rx_count: u32,
    /// 发送帧计数
    pub frame_tx_count: u32,
    /// 错误帧计数（CRC 错误 + 格式错误）
    pub error_count: u32,
}

impl TagBus {
    /// 创建新的 TagBus 实例
    pub fn new(device_id: u16) -> Self {
        const NO_TAG: Option<TagMeta> = None;
        const NO_VAL: Option<TagValue> = None;
        Self {
            device_id,
            tags: [NO_TAG; MAX_TAGS],
            tag_values: [NO_VAL; MAX_TAGS],
            tag_count: 0,
            tx_buf: [0u8; TX_BUF_SIZE],
            rx_buf: [0u8; RX_BUF_SIZE],
            rx_pos: 0,
            status: DEV_STATUS_OK,
            uptime_seconds: 0,
            frame_rx_count: 0,
            frame_tx_count: 0,
            error_count: 0,
        }
    }

    // -- 标签注册 -----------------------------------------------------------

    /// 注册只读标签
    pub fn register_ro(&mut self, meta: TagMeta, initial_value: TagValue) {
        if self.tag_count < MAX_TAGS {
            self.tags[self.tag_count] = Some(meta);
            self.tag_values[self.tag_count] = Some(initial_value);
            self.tag_count += 1;
        }
    }

    /// 注册可读写标签
    pub fn register_rw(&mut self, meta: TagMeta, initial_value: TagValue) {
        self.register_ro(meta, initial_value);
    }

    /// 按 ID 查找标签索引
    fn find_tag(&self, id: u16) -> Option<usize> {
        for i in 0..self.tag_count {
            if let Some(ref tag) = self.tags[i]
                && tag.id == id
            {
                return Some(i);
            }
        }
        None
    }

    // -- 帧构建辅助 ---------------------------------------------------------

    /// 写入帧头
    fn write_frame_header(&mut self, func: u8, payload_len: u16) -> usize {
        let mut pos = 0;
        // 帧头
        self.tx_buf[pos..pos + 2].copy_from_slice(&FRAME_HEADER.to_le_bytes());
        pos += 2;
        // 设备 ID
        self.tx_buf[pos..pos + 2].copy_from_slice(&self.device_id.to_le_bytes());
        pos += 2;
        // 功能码
        self.tx_buf[pos] = func;
        pos += 1;
        // 载荷长度
        self.tx_buf[pos..pos + 2].copy_from_slice(&payload_len.to_le_bytes());
        pos += 2;
        pos
    }

    /// 写入 CRC 和帧尾，返回帧总长度
    fn finalize_frame(&mut self, payload_start: usize, payload_len: u16) -> usize {
        let crc_start = OFF_DEVICE_ID;
        let crc_end = payload_start + payload_len as usize;
        let crc = crc16(&self.tx_buf[crc_start..crc_end]);
        let pos = crc_end;
        self.tx_buf[pos..pos + 2].copy_from_slice(&crc.to_le_bytes());
        self.tx_buf[pos + 2..pos + 4].copy_from_slice(&FRAME_FOOTER.to_le_bytes());
        pos + 4
    }

    // -- 请求处理 -----------------------------------------------------------

    /// 处理接收到的帧，返回响应帧长度（0 表示不发送响应）
    pub fn process_frame(&mut self, frame: &[u8]) -> usize {
        if frame.len() < FRAME_MIN_LEN {
            self.error_count = self.error_count.wrapping_add(1);
            return 0;
        }

        // 验证帧头/帧尾
        let header = u16::from_le_bytes([frame[OFF_HEADER], frame[OFF_HEADER + 1]]);
        if header != FRAME_HEADER {
            self.error_count = self.error_count.wrapping_add(1);
            return 0;
        }

        let payload_len =
            u16::from_le_bytes([frame[OFF_PAYLOAD_LEN], frame[OFF_PAYLOAD_LEN + 1]]) as usize;
        let footer_off = OFF_PAYLOAD + payload_len + 2; // +2 for CRC
        if frame.len() < footer_off + 2 {
            self.error_count = self.error_count.wrapping_add(1);
            return 0;
        }
        let footer = u16::from_le_bytes([frame[footer_off], frame[footer_off + 1]]);
        if footer != FRAME_FOOTER {
            self.error_count = self.error_count.wrapping_add(1);
            return 0;
        }

        // 验证 CRC
        if !verify_crc(frame, payload_len) {
            warn!("CRC mismatch");
            self.error_count = self.error_count.wrapping_add(1);
            return 0;
        }

        // 验证设备 ID：匹配自身 或 广播地址
        let dev_id = u16::from_le_bytes([frame[OFF_DEVICE_ID], frame[OFF_DEVICE_ID + 1]]);
        let is_broadcast = dev_id == BROADCAST_ID;
        let is_self = dev_id == self.device_id;

        if !is_self && !is_broadcast {
            // 不是发给我的，静默丢弃
            return 0;
        }

        self.frame_rx_count = self.frame_rx_count.wrapping_add(1);

        let func = frame[OFF_FUNC];
        let payload = &frame[OFF_PAYLOAD..OFF_PAYLOAD + payload_len];

        // 广播只对写命令有效，读命令广播无意义
        if is_broadcast && func != FUNC_WRITE_HOLDING {
            return 0;
        }

        let response_len = match func {
            FUNC_READ_TAG_LIST => self.handle_read_tag_list(payload),
            FUNC_READ_INPUT => self.handle_read_tags(payload, TAG_ACCESS_RO),
            FUNC_READ_HOLDING => self.handle_read_tags(payload, TAG_ACCESS_RW),
            FUNC_WRITE_HOLDING => self.handle_write_holding(payload),
            FUNC_DEVICE_INFO => self.handle_device_info(),
            FUNC_HEARTBEAT => self.handle_heartbeat(),
            _ => {
                warn!("Unknown function code: 0x{:02X}", func);
                self.build_error_response(func, STATUS_UNKNOWN_FUNC)
            }
        };

        // 广播写：执行但不回复
        if is_broadcast {
            return 0;
        }

        if response_len > 0 {
            self.frame_tx_count = self.frame_tx_count.wrapping_add(1);
        }

        response_len
    }

    /// 处理 0x01 读标签列表（支持分页）
    ///
    /// 请求载荷: `start(2B) + count(2B)`
    /// - `start`: 起始索引（从 0 开始）
    /// - `count`: 返回数量，0 表示全部
    ///
    /// 响应载荷: `total(2B) + start(2B) + count(2B) + 标签列表`
    pub fn handle_read_tag_list(&mut self, payload: &[u8]) -> usize {
        // 解析分页参数
        let start = if payload.len() >= 2 {
            u16::from_le_bytes([payload[0], payload[1]]) as usize
        } else {
            0
        };
        let count = if payload.len() >= 4 {
            u16::from_le_bytes([payload[2], payload[3]]) as usize
        } else {
            0 // 默认全部
        };

        let total = self.tag_count;
        let actual_start = start.min(total);
        let actual_count = if count == 0 {
            total.saturating_sub(actual_start)
        } else {
            count.min(total.saturating_sub(actual_start))
        };

        let mut pos = self.write_frame_header(FUNC_READ_TAG_LIST, 0);
        let payload_start = pos;

        // 总标签数 (2B)
        let total_u16 = total as u16;
        self.tx_buf[pos..pos + 2].copy_from_slice(&total_u16.to_le_bytes());
        pos += 2;

        // 起始索引 (2B)
        self.tx_buf[pos..pos + 2].copy_from_slice(&(actual_start as u16).to_le_bytes());
        pos += 2;

        // 返回数量 (2B)
        self.tx_buf[pos..pos + 2].copy_from_slice(&(actual_count as u16).to_le_bytes());
        pos += 2;

        // 遍历请求范围内的标签
        for i in actual_start..actual_start + actual_count {
            if let Some(ref tag) = self.tags[i] {
                // ID (2B)
                self.tx_buf[pos..pos + 2].copy_from_slice(&tag.id.to_le_bytes());
                pos += 2;
                // 类型 (1B)
                self.tx_buf[pos] = tag.data_type;
                pos += 1;
                // 访问模式 (1B)
                self.tx_buf[pos] = tag.access;
                pos += 1;
                // 名称长度 (1B)
                self.tx_buf[pos] = tag.name_len;
                pos += 1;
                // 名称 (nB)
                let name_len = tag.name_len as usize;
                self.tx_buf[pos..pos + name_len].copy_from_slice(&tag.name[..name_len]);
                pos += name_len;
                // 扩展属性长度 (1B)
                self.tx_buf[pos] = tag.ext_len;
                pos += 1;
                // 扩展属性 (nB)
                let ext_len = tag.ext_len as usize;
                self.tx_buf[pos..pos + ext_len].copy_from_slice(&tag.ext[..ext_len]);
                pos += ext_len;
            }
        }

        let payload_len = (pos - payload_start) as u16;
        // 回写载荷长度
        let len_pos = OFF_PAYLOAD_LEN;
        self.tx_buf[len_pos..len_pos + 2].copy_from_slice(&payload_len.to_le_bytes());

        self.finalize_frame(payload_start, payload_len)
    }

    /// 处理 0x02 / 0x03 读标签值
    fn handle_read_tags(&mut self, payload: &[u8], expected_access: u8) -> usize {
        let func = if expected_access == TAG_ACCESS_RO {
            FUNC_READ_INPUT
        } else {
            FUNC_READ_HOLDING
        };

        if payload.len() < 2 {
            return self.build_error_response(func, STATUS_NOT_FOUND);
        }

        let count = u16::from_le_bytes([payload[0], payload[1]]) as usize;
        let mut ids = [0u16; 32];
        let mut actual_count: usize = 0;

        // 解析请求的 ID 列表
        #[allow(clippy::needless_range_loop)]
        for i in 0..count.min(32) {
            let off = 2 + i * 2;
            if off + 2 <= payload.len() {
                ids[i] = u16::from_le_bytes([payload[off], payload[off + 1]]);
                actual_count += 1;
            }
        }

        let mut pos = self.write_frame_header(func, 0);
        let payload_start = pos;

        // 返回数量先占位
        let count_pos = pos;
        pos += 2;
        let mut returned: u16 = 0;

        #[allow(clippy::needless_range_loop)]
        for i in 0..actual_count {
            if let Some(idx) = self.find_tag(ids[i]) {
                match &self.tags[idx] {
                    Some(tag) => {
                        // ID (2B)
                        self.tx_buf[pos..pos + 2].copy_from_slice(&tag.id.to_le_bytes());
                        pos += 2;

                        // 检查访问权限
                        if expected_access == TAG_ACCESS_RW && tag.access != TAG_ACCESS_RW {
                            // 状态 (1B) — 只读
                            self.tx_buf[pos] = STATUS_READ_ONLY;
                            pos += 1;
                            // 值长度 (1B) — 0
                            self.tx_buf[pos] = 0;
                            pos += 1;
                        } else if let Some(ref value) = self.tag_values[idx] {
                            // 状态 (1B)
                            self.tx_buf[pos] = STATUS_OK;
                            pos += 1;
                            // 值长度 (1B)
                            let vlen = value.len();
                            self.tx_buf[pos] = vlen;
                            pos += 1;
                            // 值 (nB)
                            pos += value.to_bytes(&mut self.tx_buf[pos..]);
                        } else {
                            // 状态 — 不存在
                            self.tx_buf[pos] = STATUS_NOT_FOUND;
                            pos += 1;
                            self.tx_buf[pos] = 0;
                            pos += 1;
                        }
                        returned += 1;
                    }
                    None => {
                        // tags[idx] 为 None（内部不一致，防御性处理）
                        self.tx_buf[pos..pos + 2].copy_from_slice(&ids[i].to_le_bytes());
                        pos += 2;
                        self.tx_buf[pos] = STATUS_NOT_FOUND;
                        pos += 1;
                        self.tx_buf[pos] = 0;
                        pos += 1;
                        returned += 1;
                    }
                }
            } else {
                // ID 不存在
                self.tx_buf[pos..pos + 2].copy_from_slice(&ids[i].to_le_bytes());
                pos += 2;
                self.tx_buf[pos] = STATUS_NOT_FOUND;
                pos += 1;
                self.tx_buf[pos] = 0;
                pos += 1;
                returned += 1;
            }
        }

        // 回写返回数量
        self.tx_buf[count_pos..count_pos + 2].copy_from_slice(&returned.to_le_bytes());

        let payload_len = (pos - payload_start) as u16;
        let len_pos = OFF_PAYLOAD_LEN;
        self.tx_buf[len_pos..len_pos + 2].copy_from_slice(&payload_len.to_le_bytes());

        self.finalize_frame(payload_start, payload_len)
    }

    /// 处理 0x05 写保持标签
    fn handle_write_holding(&mut self, payload: &[u8]) -> usize {
        if payload.len() < 2 {
            return self.build_error_response(FUNC_WRITE_HOLDING, STATUS_NOT_FOUND);
        }

        let count = u16::from_le_bytes([payload[0], payload[1]]) as usize;

        // 先验证并执行写入，收集结果
        #[derive(Copy, Clone)]
        struct WriteResult {
            id: u16,
            status: u8,
        }
        let mut results: [WriteResult; 32] = [WriteResult { id: 0, status: 0 }; 32];
        let mut result_count: usize = 0;

        let mut off = 2;
        for _ in 0..count.min(32) {
            if off + 3 > payload.len() {
                break;
            }

            let id = u16::from_le_bytes([payload[off], payload[off + 1]]);
            let val_len = payload[off + 2] as usize;
            off += 3;

            if off + val_len > payload.len() {
                break;
            }
            let val_bytes = &payload[off..off + val_len];
            off += val_len;

            let status = if let Some(idx) = self.find_tag(id) {
                if let Some(ref tag) = self.tags[idx] {
                    if tag.access != TAG_ACCESS_RW {
                        STATUS_READ_ONLY
                    } else {
                        let expected_len = tag.type_len();
                        let actual_len = val_len as u8;
                        if expected_len != 0 && actual_len != expected_len {
                            STATUS_TYPE_MISMATCH
                        } else if let Some(value) =
                            TagValue::from_bytes(tag.data_type, val_bytes, actual_len)
                        {
                            self.tag_values[idx] = Some(value);
                            STATUS_OK
                        } else {
                            STATUS_TYPE_MISMATCH
                        }
                    }
                } else {
                    STATUS_NOT_FOUND
                }
            } else {
                STATUS_NOT_FOUND
            };

            if result_count < 32 {
                results[result_count] = WriteResult { id, status };
                result_count += 1;
            }
        }

        // 构建响应
        let mut pos = self.write_frame_header(FUNC_WRITE_HOLDING, 0);
        let payload_start = pos;

        // 写入数量
        let ret_count = result_count as u16;
        self.tx_buf[pos..pos + 2].copy_from_slice(&ret_count.to_le_bytes());
        pos += 2;

        #[allow(clippy::needless_range_loop)]
        for i in 0..result_count {
            // ID (2B)
            self.tx_buf[pos..pos + 2].copy_from_slice(&results[i].id.to_le_bytes());
            pos += 2;
            // 状态 (1B)
            self.tx_buf[pos] = results[i].status;
            pos += 1;
        }

        let payload_len = (pos - payload_start) as u16;
        let len_pos = OFF_PAYLOAD_LEN;
        self.tx_buf[len_pos..len_pos + 2].copy_from_slice(&payload_len.to_le_bytes());

        self.finalize_frame(payload_start, payload_len)
    }

    /// 处理 0x06 获取设备信息
    fn handle_device_info(&mut self) -> usize {
        let mut pos = self.write_frame_header(FUNC_DEVICE_INFO, 0);
        let payload_start = pos;

        // 协议版本 (2B)
        self.tx_buf[pos] = 2;
        self.tx_buf[pos + 1] = 0;
        pos += 2;

        // 厂商 ID (2B)
        self.tx_buf[pos..pos + 2].copy_from_slice(&1u16.to_le_bytes());
        pos += 2;

        // 设备型号 (2B)
        self.tx_buf[pos..pos + 2].copy_from_slice(&2u16.to_le_bytes());
        pos += 2;

        // 固件版本 (4B): year(2) + month(1) + day(1)
        self.tx_buf[pos..pos + 2].copy_from_slice(&2024u16.to_le_bytes());
        pos += 2;
        self.tx_buf[pos] = 1;
        self.tx_buf[pos + 1] = 1;
        pos += 2;

        // 只读标签数 (2B)
        let mut ro_count: u16 = 0;
        for i in 0..self.tag_count {
            if let Some(ref tag) = self.tags[i]
                && tag.access == TAG_ACCESS_RO
            {
                ro_count += 1;
            }
        }
        self.tx_buf[pos..pos + 2].copy_from_slice(&ro_count.to_le_bytes());
        pos += 2;

        // 可读写标签数 (2B)
        self.tx_buf[pos..pos + 2]
            .copy_from_slice(&((self.tag_count as u16).saturating_sub(ro_count)).to_le_bytes());
        pos += 2;

        // 设备名称长度 + 名称
        let name = b"TagBus Device";
        self.tx_buf[pos] = name.len() as u8;
        pos += 1;
        self.tx_buf[pos..pos + name.len()].copy_from_slice(name);
        pos += name.len();

        let payload_len = (pos - payload_start) as u16;
        let len_pos = OFF_PAYLOAD_LEN;
        self.tx_buf[len_pos..len_pos + 2].copy_from_slice(&payload_len.to_le_bytes());

        self.finalize_frame(payload_start, payload_len)
    }

    /// 处理 0x07 心跳/设备状态
    ///
    /// 请求载荷: 无
    ///
    /// 响应载荷:
    /// ```
    /// status(1B) + uptime(4B) + rx_count(4B) + tx_count(4B) + error_count(4B)
    /// ```
    fn handle_heartbeat(&mut self) -> usize {
        let mut pos = self.write_frame_header(FUNC_HEARTBEAT, 0);
        let payload_start = pos;

        // 设备状态 (1B)
        self.tx_buf[pos] = self.status;
        pos += 1;

        // 运行秒数 (4B, 小端)
        self.tx_buf[pos..pos + 4].copy_from_slice(&self.uptime_seconds.to_le_bytes());
        pos += 4;

        // 接收帧计数 (4B, 小端)
        self.tx_buf[pos..pos + 4].copy_from_slice(&self.frame_rx_count.to_le_bytes());
        pos += 4;

        // 发送帧计数 (4B, 小端)
        self.tx_buf[pos..pos + 4].copy_from_slice(&self.frame_tx_count.to_le_bytes());
        pos += 4;

        // 错误帧计数 (4B, 小端)
        self.tx_buf[pos..pos + 4].copy_from_slice(&self.error_count.to_le_bytes());
        pos += 4;

        let payload_len = (pos - payload_start) as u16;
        let len_pos = OFF_PAYLOAD_LEN;
        self.tx_buf[len_pos..len_pos + 2].copy_from_slice(&payload_len.to_le_bytes());

        self.finalize_frame(payload_start, payload_len)
    }

    /// 构建错误响应
    fn build_error_response(&mut self, func: u8, status: u8) -> usize {
        let mut pos = self.write_frame_header(func, 0);
        let payload_start = pos;

        // 返回单个状态码
        self.tx_buf[pos] = status;
        pos += 1;

        let payload_len = (pos - payload_start) as u16;
        let len_pos = OFF_PAYLOAD_LEN;
        self.tx_buf[len_pos..len_pos + 2].copy_from_slice(&payload_len.to_le_bytes());

        self.finalize_frame(payload_start, payload_len)
    }

    // -- UART 帧接收 ---------------------------------------------------------

    /// 向接收缓冲区喂入一个字节
    ///
    /// 返回 `Some(frame_end_index)` 当接收到完整帧时，
    /// 帧结束索引指向帧尾最后一个字节之后。
    pub fn feed_byte(&mut self, byte: u8) -> Option<usize> {
        if self.rx_pos >= RX_BUF_SIZE {
            // 缓冲区溢出，重置
            self.rx_pos = 0;
            return None;
        }

        self.rx_buf[self.rx_pos] = byte;
        self.rx_pos += 1;

        // 检查是否收到可能的帧尾标记
        if self.rx_pos >= FRAME_MIN_LEN {
            // 查找帧头
            if let Some(start) = self.find_frame_header() {
                let end = self.rx_pos;

                if end - start >= FRAME_MIN_LEN {
                    let payload_len = u16::from_le_bytes([
                        self.rx_buf[start + OFF_PAYLOAD_LEN],
                        self.rx_buf[start + OFF_PAYLOAD_LEN + 1],
                    ]) as usize;
                    let expected_len = OFF_PAYLOAD + payload_len + 4; // +2 CRC + 2 footer

                    if end - start >= expected_len {
                        let footer_off = start + expected_len - 2;
                        let footer = u16::from_le_bytes([
                            self.rx_buf[footer_off],
                            self.rx_buf[footer_off + 1],
                        ]);

                        if footer == FRAME_FOOTER {
                            // 完整帧接收，将数据移到缓冲区开头
                            let frame_len = expected_len;
                            // 复制有效帧到开头
                            for i in 0..frame_len {
                                self.rx_buf[i] = self.rx_buf[start + i];
                            }
                            self.rx_pos = frame_len;
                            return Some(frame_len);
                        }
                    }
                }
            }
        }

        None
    }

    /// 查找帧头位置
    fn find_frame_header(&self) -> Option<usize> {
        if self.rx_pos < 2 {
            return None;
        }
        for i in 0..self.rx_pos - 1 {
            let val = u16::from_le_bytes([self.rx_buf[i], self.rx_buf[i + 1]]);
            if val == FRAME_HEADER {
                return Some(i);
            }
        }
        None
    }

    /// 获取接收缓冲区中已接收的完整帧
    pub fn received_frame(&self) -> &[u8] {
        &self.rx_buf[..self.rx_pos]
    }

    /// 重置接收缓冲区
    pub fn reset_rx(&mut self) {
        self.rx_pos = 0;
    }

    /// 获取发送缓冲区及帧长度
    pub fn tx_frame(&self, len: usize) -> &[u8] {
        &self.tx_buf[..len]
    }
}

// ============================================================================
// 主程序 — 温控器示例
// ============================================================================

#[main]
fn main() -> ! {
    info!("=== TagBus v4.1 - Thermostat Example ===");

    // ------------------------------------------------------------------
    // 1. 外设初始化
    // ------------------------------------------------------------------
    let dp = Peripherals::take().unwrap();

    // 系统时钟: 150 MHz
    let clocks = ClockBuilder::new().build(dp.CLOCKS, dp.XOSC, dp.PLL_SYS, dp.PLL_USB);
    info!("System clock: {} Hz", clocks.sys_freq());

    let pins = Pins::init(dp.IO_BANK0, dp.PADS_BANK0);
    let _timer = Timer0::init(dp.TIMER0);

    // ------------------------------------------------------------------
    // 2. UART0 初始化 (GP0=TX, GP1=RX, 115200 baud)
    // ------------------------------------------------------------------
    let mut uart = Uart0::init(
        dp.UART0,
        pins.pin::<0>(), // TX
        pins.pin::<1>(), // RX
        UART_BAUD,
        clocks.peri_freq(),
    );
    uart.set_format(DataBits::Eight, StopBits::One, Parity::None);

    info!("UART0: {} baud, 8N1 (GP0=TX, GP1=RX)", UART_BAUD);

    // ------------------------------------------------------------------
    // 3. 初始化 TagBus 协议引擎
    // ------------------------------------------------------------------
    let mut tagbus = TagBus::new(0x0001);

    // 注册只读标签 (RO): 温度、湿度、状态
    tagbus.register_ro(
        TagMeta::new(100, TYPE_FLOAT32, TAG_ACCESS_RO, "温度", "unit:℃"),
        TagValue::Float32(26.0),
    );

    tagbus.register_ro(
        TagMeta::new(101, TYPE_FLOAT32, TAG_ACCESS_RO, "湿度", "unit:%"),
        TagValue::Float32(50.0),
    );

    tagbus.register_ro(
        TagMeta::new(102, TYPE_BOOL, TAG_ACCESS_RO, "状态", "0:关,1:开"),
        TagValue::Bool(true),
    );

    // 注册可读写标签 (RW): 设定温度、加热
    tagbus.register_rw(
        TagMeta::new(
            200,
            TYPE_FLOAT32,
            TAG_ACCESS_RW,
            "设定温",
            "unit:℃,min:10,max:35",
        ),
        TagValue::Float32(25.5),
    );

    tagbus.register_rw(
        TagMeta::new(201, TYPE_BOOL, TAG_ACCESS_RW, "加热", ""),
        TagValue::Bool(true),
    );

    // 注册字符串标签
    tagbus.register_ro(
        TagMeta::new(202, TYPE_STRING, TAG_ACCESS_RO, "设备名", ""),
        TagValue::String(
            {
                let mut buf = [0u8; 255];
                let s = b"AB-123";
                let mut i = 0;
                while i < s.len() {
                    buf[i] = s[i];
                    i += 1;
                }
                buf
            },
            b"AB-123".len() as u8,
        ),
    );

    info!("Tags registered: {}", tagbus.tag_count);

    // ------------------------------------------------------------------
    // 4. 发送启动就绪信息
    // ------------------------------------------------------------------
    uart.write(b"\r\n");
    uart.write(b"============================================\r\n");
    uart.write(b"  TagBus v4.1 - Thermostat Device\r\n");
    uart.write(b"  Device ID: 0x0001\r\n");
    uart.write(b"  Baud: 115200, 8N1\r\n");
    uart.write(b"  Tags: 4 RO + 2 RW = 6 total\r\n");
    uart.write(b"============================================\r\n");
    uart.write(b"\r\n");
    uart.flush();

    // ------------------------------------------------------------------
    // 5. 主循环: 接收帧 → 处理 → 发送响应
    // ------------------------------------------------------------------
    let sys_freq = clocks.sys_freq() as u64;
    let mut last_second_tick = 0u64;
    let mut tick: u64 = 0;

    loop {
        tick = tick.wrapping_add(1);

        // 每秒更新一次: uptime + 传感器值
        if tick.wrapping_sub(last_second_tick) >= sys_freq {
            last_second_tick = tick;
            tagbus.uptime_seconds = tagbus.uptime_seconds.wrapping_add(1);
            update_sensor_values(&mut tagbus, tick);
        }

        // 读取字节
        if let Some(byte) = uart.read_byte()
            && let Some(frame_len) = tagbus.feed_byte(byte)
        {
            // 收到完整帧
            let func = tagbus.received_frame()[OFF_FUNC];
            info!("RX request: func=0x{:02X}, len={}", func, frame_len);

            let response_len = {
                let frame = tagbus.received_frame();
                let len = frame.len();
                // 将帧数据复制出来以避免借用冲突
                let mut frame_copy = [0u8; RX_BUF_SIZE];
                frame_copy[..len].copy_from_slice(frame);
                tagbus.process_frame(&frame_copy[..len])
            };
            if response_len > 0 {
                let resp = tagbus.tx_frame(response_len);
                uart.write(resp);
                uart.flush();
                info!("TX response: {} bytes", response_len);
            }

            tagbus.reset_rx();
        }
    }
}

/// 模拟传感器值更新 (在实际设备中替换为真实传感器读数)
fn update_sensor_values(tagbus: &mut TagBus, _tick: u64) {
    // 更新温度 (26.0 ± 小幅波动)
    if let Some(idx) = tagbus.find_tag(100) {
        tagbus.tag_values[idx] = Some(TagValue::Float32(26.0));
    }
    // 更新湿度
    if let Some(idx) = tagbus.find_tag(101) {
        tagbus.tag_values[idx] = Some(TagValue::Float32(50.0));
    }
    // 更新状态
    if let Some(idx) = tagbus.find_tag(102) {
        tagbus.tag_values[idx] = Some(TagValue::Bool(true));
    }
}
