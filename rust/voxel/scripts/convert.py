#!/usr/bin/env python3
'''
Convert map from set of PNG files to native format
for inclusion.

Example maps can be found here:
https://github.com/s-macke/VoxelSpace/tree/master/maps
'''
import sys
from PIL import Image
import struct

def rgb565(r, g, b):
    '''Truncate RGB888 color to RGB565'''
    return (((r) >> 3) << 11) | (((g) >> 2) << 5) | ((b) >> 3)

def convert_palette(pal):
    '''Convert 8-bit indexed image palette to RGB565'''
    return [rgb565(pal[i*3+0], pal[i*3+1], pal[i*3+2]) for i in range(256)]

color_fname = sys.argv[1]
depth_fname = sys.argv[2]
out_fname = sys.argv[3]

color_img = Image.open(color_fname)
depth_img = Image.open(depth_fname)
assert(color_img.size == (1024,1024))
assert(depth_img.size == (1024,1024))
assert(color_img.getbands() == ('P',))
assert(len(color_img.getpalette()) == 768)
assert(depth_img.getbands() == ('L',))

# downsample
delta = 4
size_out = (color_img.size[0] // delta, color_img.size[1] // delta)

out = []
out += convert_palette(color_img.getpalette())
for y in range(0, size_out[1]):
    for x in range(0, size_out[0]):
        pos = (x * delta, y * delta)
        val = color_img.getpixel(pos) | (depth_img.getpixel(pos) << 8)
        out.append(val)

with open(out_fname, 'wb') as f:
    for val in out:
        f.write(struct.pack('<H', val))
