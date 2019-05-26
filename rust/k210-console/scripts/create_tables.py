#!/usr/bin/env python3
import unicodedata

mapping = []
with open('../data/values.tsv', 'r') as f:
    lines = iter(f)
    next(lines)
    for line in lines:
        (byte, unich, name) = line.rstrip().split('\t')
        byte = int(byte, 0)
        unich = int(unich, 0)
        mapping.append((byte, unich, name))

# add in ASCII
mapping.append((0x00, 0x0000, 'NUL'))
for ch in range(0x20, 0x7f):
    mapping.append((ch, ch, unicodedata.name(chr(ch))))
mapping.sort()

fw_mapping = [None] * 256
for byte, unich, name in mapping:
    #print('%02x %05x %s' % (byte, unich, name))
    fw_mapping[byte] = unich

def qchar(ch):
    return "'\\u{%04x}'" % ch

print('static FROM: [char; 256] = [')
for i in range(0x100):
    if (i % 8)==0:
        print("    ", end='')
    print("%s," % qchar(fw_mapping[i]), end='')
    if ((i+1) % 8)==0:
        print()
    else:
        print(" ", end='')
print(']')

print('pub fn to(ch: char) -> u8 {')
print('    match ch {')
for byte, unich, name in mapping:
    print('        %s => 0x%02x, // %s' % (qchar(unich), byte, name))
print('    }')
print('}')

