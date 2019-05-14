#!/usr/bin/env python3
'''Script to generate BLOCK_SPRITE constant'''
BLK_SIZE = 8

def rgb565(r, g, b):
    return ((r >> 3) << 11) | ((g >> 2) << 5) | (b >> 3);

image = [[(0, 0, 0)] * BLK_SIZE for _ in range(BLK_SIZE)]

mm = BLK_SIZE//2-1
for y in range(BLK_SIZE//2):
    for x in range(BLK_SIZE//2):
        l = min(min(x,y),mm)
        col = (0xa8*(l+1)//mm,0x48*(l+1)//mm,0xa8*(l+1)//mm)
        image[y][x] = col
        image[BLK_SIZE-y-1][x] = col
        image[y][BLK_SIZE-x-1] = col
        image[BLK_SIZE-y-1][BLK_SIZE-x-1] = col

outb = []
for y in range(BLK_SIZE):
    outb.append([((rgb565(*image[y][x*2+0])<<16) | rgb565(*image[y][x*2+1])) for x in range(BLK_SIZE//2)])

print('pub static BLOCK_SPRITE: [[u32; 4];8] = [')
for y in outb:
    print('    [%s],' % (', '.join(('0x%08x' % i) for i in y)))
print('];')
