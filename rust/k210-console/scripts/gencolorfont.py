#!/usr/bin/env python3
'''
Slice an image into 8x8 chunks (tiles) to create a color font.
'''
import sys
from PIL import Image
import struct

BW=8       # tile width
BH=8       # tile height
BG=(0,0,0) # RGB color for background

def rgb565(color):
    '''Truncate RGB888[8] color to RGB565'''
    return ((color[0] >> 3) << 11) | ((color[1] >> 2) << 5) | (color[2] >> 3)

def extract_block(img, coord):
    '''Extract a RGB block from an image.'''
    data = []
    for yi in range(0, BH):
        row = []
        for xi in range(0, BW):
            try:
                row.append(img.getpixel((coord[0] + xi, coord[1] + yi)))
            except IndexError:
                row.append(BG)
        data.append(row)
    return data
            
def encode_block(block):
    '''Encode RGB block to 32-bit column-swizzled RGB565'''
    out = []
    for yi in range(0, BH):
        for xi in range(0, BW//2):
            out.append(
                (rgb565(block[yi][xi*2 + 0]) << 16) |
                 rgb565(block[yi][xi*2 + 1]))
    return tuple(out)

infile = sys.argv[1]
outfile = sys.argv[2]

img = Image.open(infile)

blocks_x = (img.size[0] + (BW-1))//BW
blocks_y = (img.size[1] + (BH-1))//BH

print(f'{blocks_x}×{blocks_y}')

# character set, addressed by content
charset = {}

# add empty block as character 0
empty_block = encode_block([[BG]*BW]*BH)
charset[empty_block] = 0

out = []
for by in range(0, blocks_y):
    row = []
    for bx in range(0, blocks_x):
        bd = encode_block(extract_block(img, (bx * BW, by * BH)))
        # add character to character set
        try:
            ch = charset[bd]
        except KeyError:
            ch = len(charset)
            charset[bd] = ch
        row.append(ch)
    out.append(row)

m = len(empty_block)
n = len(charset)
print(f'used {n} characters')

charset_by_ch = [None] * n
for (bd, ch) in charset.items():
    charset_by_ch[ch] = bd

with open(outfile, 'w') as f:
    f.write(f'/* Auto-generated from {infile} by gencolorfont.py */\n')
    f.write(f'pub static FONT: [[u32; {m}]; {n}] = [\n')
    for bd in charset_by_ch:
        f.write('    [')
        for val in bd:
            f.write(f'0x{val:08x}, ')
        f.write('],\n')

    f.write('];\n')
    f.write('\n')

    # TODO: output sequence; RLE encoding of some kind?
    f.write(f'pub static SEQ: [[u16; {blocks_x}]; {blocks_y}] = [\n')
    for subseq in out:
        f.write('    [')
        for val in subseq:
            f.write(f'0x{val:04x}, ')
        f.write('],\n')
    f.write('];\n')
    
