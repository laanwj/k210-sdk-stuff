#!/usr/bin/env python3
'''
Slice an image (or multiple images) into 8x8 chunks (tiles) and deduplicate
them to create a color font and a sequence of characters that represents the
original image(s).
'''
import sys
from PIL import Image
import struct

BW=8       # tile width
BH=8       # tile height
BG=(0,0,0) # RGB color for background (hardcoded, could be configurable)

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

assert(len(sys.argv) >= 3)
infiles = sys.argv[1:-1]
outfile = sys.argv[-1]

images = [(infile, Image.open(infile)) for infile in infiles]

# character set, addressed by content
charset = {}

# add empty block as character 0
empty_block = encode_block([[BG]*BW]*BH)
charset[empty_block] = 0

# sequence of characters to represent every image
seq = []
for (infile, img) in images:
    blocks_x = (img.size[0] + (BW-1))//BW
    blocks_y = (img.size[1] + (BH-1))//BH

    print(f'{infile}: {blocks_x}×{blocks_y}')

    out = []
    for by in range(0, blocks_y):
        row = []
        for bx in range(0, blocks_x):
            bd = encode_block(extract_block(img, (bx * BW, by * BH)))
            try:
                # re-use existing matching character
                ch = charset[bd]
            except KeyError:
                # add character to character set
                ch = len(charset)
                charset[bd] = ch
            row.append(ch)
        out.append(row)
    seq.append((out, blocks_x, blocks_y))

m = len(empty_block)
n = len(charset)
print(f'used {n} characters')

charset_by_ch = [None] * n
for (bd, ch) in charset.items():
    charset_by_ch[ch] = bd

with open(outfile, 'w') as f:
    f.write(f'/* Auto-generated from {infile} by gencolorfont.py */\n')
    f.write('#[rustfmt::skip]\n')
    f.write(f'pub static FONT: [[u32; {m}]; {n}] = [\n')
    for bd in charset_by_ch:
        f.write('    [')
        for val in bd:
            f.write(f'0x{val:08x}, ')
        f.write('],\n')

    f.write('];\n')
    f.write('\n')

    # TODO: output sequence; RLE encoding of some kind?
    for (i,(out,blocks_x,blocks_y)) in enumerate(seq):
        f.write('#[rustfmt::skip]\n')
        f.write(f'pub static SEQ{i}: [[u16; {blocks_x}]; {blocks_y}] = [\n')
        for subseq in out:
            f.write('    [')
            for val in subseq:
                f.write(f'0x{val:04x}, ')
            f.write('],\n')
        f.write('];\n')
