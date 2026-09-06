#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
从真实的 HZK16(GB2312 16×16 点阵) 二进制字库中，按区位索引抽取所需汉字，
生成与现有 font.rs 相同格式的子集（只含用到的字）。

用法:
    python3 gen_hzk16.py                 # 默认 "屏幕测试"
    python3 gen_hzk16.py "新的文字..."
    python3 gen_hzk16.py "文字" /path/to/HZK16

HZK16 字形: 16 行 × 2 字节，行内高位在前(MSB)，与 st7789 draw_text 位序一致。
索引: offset = (94*(b0-0xA1) + (b1-0xA1)) * 32
"""
import sys

# m5-hzk16-master 工程自带的 HZK16 真点阵字库
HZK16_PATH = "/home/danc/code/rp2350/m5-hzk16-master/HZK16/Fonts/HZK16"
OUT_PATH = "font.rs"
TEXT = "屏幕测试"


def load_glyph(data, ch):
    try:
        b = ch.encode("gb2312")
    except UnicodeEncodeError:
        return None  # 不在 GB2312 简体集内
    if len(b) != 2:
        return None
    b0, b1 = b[0], b[1]
    offset = (94 * (b0 - 0xA1) + (b1 - 0xA1)) * 32
    if offset < 0 or offset + 32 > len(data):
        return None
    return data[offset:offset + 32]


def fmt_rows(data, rowbytes):
    rows = []
    for y in range(0, len(data), rowbytes):
        rows.append(", ".join("0x%02X" % x for x in data[y:y + rowbytes]))
    return ",\n    ".join("        " + r for r in rows)


def main():
    text = sys.argv[1] if len(sys.argv) > 1 else TEXT
    font_path = sys.argv[2] if len(sys.argv) > 2 else HZK16_PATH
    with open(font_path, "rb") as f:
        data = f.read()

    chars = []
    for ch in text:
        if ch not in chars:
            chars.append(ch)

    glyphs = {ch: load_glyph(data, ch) for ch in chars}
    missing = [ch for ch, g in glyphs.items() if g is None]

    lines = []
    lines.append("//! 中文字形位图（由 gen_hzk16.py 从 HZK16 抽取生成，请勿手改）。")
    lines.append("//! 1bit/像素，行优先，每行高位在前；每字为 GLYPH_SIZE×GLYPH_SIZE 点阵。")
    lines.append("")
    lines.append("/// 字形边长（像素）")
    lines.append("pub const GLYPH_SIZE: u8 = 16;")
    lines.append("/// 每行字节数")
    lines.append("pub const GLYPH_ROW_BYTES: usize = (GLYPH_SIZE as usize).div_ceil(8);")
    lines.append("")
    idx = 0
    for ch in chars:
        g = glyphs[ch]
        if g is None:
            continue
        lines.append("static DATA_%d: [u8; %d] = [" % (idx, len(g)))
        lines.append(fmt_rows(g, 2))
        lines.append("];")
        lines.append("")
        idx += 1
    lines.append("/// 查字符字形；命中返回点阵数据（长 GLYPH_ROW_BYTES×GLYPH_SIZE），否则 None。")
    lines.append("pub fn find(c: char) -> Option<&'static [u8]> {")
    lines.append("    match c {")
    idx = 0
    for ch in chars:
        if glyphs[ch] is None:
            continue
        lines.append("        '%s' => Some(&DATA_%d)," % (ch, idx))
        idx += 1
    lines.append("        _ => None,")
    lines.append("    }")
    lines.append("}")
    lines.append("")

    with open(OUT_PATH, "w", encoding="utf-8") as f:
        f.write("\n".join(lines))

    print("OK: 抽取 %d 字, 每字 32B, 共 %dB -> %s" % (idx, idx * 32, OUT_PATH))
    for ch in chars:
        g = glyphs[ch]
        if g is None:
            print("  缺: %r 不在 GB2312" % ch)
        else:
            n = sum(bin(b).count("1") for b in g)
            print("  %r U+%04X : 置位像素 %d" % (ch, ord(ch), n))
    if missing:
        print("注意缺字:", " ".join(missing))


if __name__ == "__main__":
    main()
