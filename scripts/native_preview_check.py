#!/usr/bin/env python3
from __future__ import annotations

import argparse
import struct
import sys
import zlib
from pathlib import Path


def fail(message: str) -> None:
    print(f"native preview check failure: {message}", file=sys.stderr)
    raise SystemExit(1)


def unfilter_scanlines(width: int, height: int, data: bytes) -> list[tuple[int, int, int, int]]:
    channels = 4
    stride = width * channels
    pos = 0
    previous = bytearray(stride)
    pixels: list[tuple[int, int, int, int]] = []

    for _ in range(height):
        if pos >= len(data):
            fail("truncated PNG scanline data")
        filter_type = data[pos]
        pos += 1
        row = bytearray(data[pos:pos + stride])
        pos += stride
        if len(row) != stride:
            fail("truncated PNG row")

        for i, value in enumerate(row):
            left = row[i - channels] if i >= channels else 0
            up = previous[i]
            up_left = previous[i - channels] if i >= channels else 0

            if filter_type == 0:
                restored = value
            elif filter_type == 1:
                restored = value + left
            elif filter_type == 2:
                restored = value + up
            elif filter_type == 3:
                restored = value + ((left + up) // 2)
            elif filter_type == 4:
                predictor = paeth(left, up, up_left)
                restored = value + predictor
            else:
                fail(f"unsupported PNG filter type {filter_type}")

            row[i] = restored & 0xFF

        for offset in range(0, stride, channels):
            pixels.append(tuple(row[offset:offset + channels]))  # type: ignore[arg-type]
        previous = row

    return pixels


def paeth(left: int, up: int, up_left: int) -> int:
    estimate = left + up - up_left
    left_distance = abs(estimate - left)
    up_distance = abs(estimate - up)
    up_left_distance = abs(estimate - up_left)
    if left_distance <= up_distance and left_distance <= up_left_distance:
        return left
    if up_distance <= up_left_distance:
        return up
    return up_left


def load_rgba_png(path: Path) -> tuple[int, int, list[tuple[int, int, int, int]]]:
    if not path.exists():
        fail(f"{path} does not exist")
    raw = path.read_bytes()
    if not raw.startswith(b"\x89PNG\r\n\x1a\n"):
        fail(f"{path} is not a PNG")

    offset = 8
    width = height = bit_depth = color_type = None
    compressed = bytearray()

    while offset < len(raw):
        if offset + 8 > len(raw):
            fail("truncated PNG chunk header")
        length = struct.unpack(">I", raw[offset:offset + 4])[0]
        chunk_type = raw[offset + 4:offset + 8]
        start = offset + 8
        end = start + length
        if end + 4 > len(raw):
            fail("truncated PNG chunk data")
        chunk_data = raw[start:end]
        offset = end + 4

        if chunk_type == b"IHDR":
            width, height, bit_depth, color_type, compression, filter_method, interlace = struct.unpack(
                ">IIBBBBB", chunk_data
            )
            if bit_depth != 8 or color_type != 6 or compression != 0 or filter_method != 0 or interlace != 0:
                fail("preview PNG must be 8-bit non-interlaced RGBA")
        elif chunk_type == b"IDAT":
            compressed.extend(chunk_data)
        elif chunk_type == b"IEND":
            break

    if width is None or height is None:
        fail("PNG missing IHDR")
    if not compressed:
        fail("PNG missing image data")

    image_data = zlib.decompress(bytes(compressed))
    return width, height, unfilter_scanlines(width, height, image_data)


def region(pixels: list[tuple[int, int, int, int]], width: int, x0: int, y0: int, x1: int, y1: int) -> list[tuple[int, int, int, int]]:
    out: list[tuple[int, int, int, int]] = []
    for y in range(y0, y1):
        start = y * width + x0
        out.extend(pixels[start:start + (x1 - x0)])
    return out


def dark_ratio(sample: list[tuple[int, int, int, int]]) -> float:
    dark = sum(1 for r, g, b, _ in sample if r < 80 and g < 80 and b < 80)
    return dark / max(len(sample), 1)


def accent_ratio(sample: list[tuple[int, int, int, int]]) -> float:
    accent = sum(1 for r, g, b, _ in sample if b > 150 and g > 80 and r < 80)
    return accent / max(len(sample), 1)


def black_ratio(sample: list[tuple[int, int, int, int]]) -> float:
    black = sum(1 for r, g, b, _ in sample if r < 8 and g < 8 and b < 8)
    return black / max(len(sample), 1)


def white_ratio(sample: list[tuple[int, int, int, int]]) -> float:
    white = sum(1 for r, g, b, _ in sample if r > 220 and g > 220 and b > 220)
    return white / max(len(sample), 1)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("image", type=Path)
    parser.add_argument("--mode", choices=["empty", "open"], default="empty")
    args = parser.parse_args()

    width, height, pixels = load_rgba_png(args.image)
    if width < 1000 or height < 650:
        fail(f"preview dimensions are too small: {width}x{height}")

    full_black = black_ratio(pixels)
    if full_black > 0.02:
        fail(f"preview has too many near-black pixels: {full_black:.2%}")

    sidebar = region(pixels, width, 0, int(height * 0.08), int(width * 0.25), int(height * 0.5))
    if dark_ratio(sidebar) < 0.01:
        fail("sidebar lacks visible dark text or icons")
    if accent_ratio(sidebar) < 0.01:
        fail("sidebar lacks selected-row accent color")

    canvas = region(pixels, width, int(width * 0.35), int(height * 0.25), int(width * 0.7), int(height * 0.75))
    if args.mode == "empty":
        if dark_ratio(canvas) < 0.002:
            fail("canvas lacks visible empty-state content")
    else:
        if white_ratio(canvas) < 0.45:
            fail("open-document canvas lacks a visible PDF page surface")
        inspector = region(pixels, width, int(width * 0.73), int(height * 0.18), int(width * 0.96), int(height * 0.65))
        if dark_ratio(inspector) < 0.01:
            fail("open-document inspector lacks visible document evidence")

    print(f"native preview check passed: {args.image} ({width}x{height})")


if __name__ == "__main__":
    main()
