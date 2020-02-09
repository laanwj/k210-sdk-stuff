/** Table from cp437 to unicode char; generated with create_tables.py */
static FROM: [char; 256] = [
    '\u{0000}', '\u{263a}', '\u{263b}', '\u{2665}', '\u{2666}', '\u{2663}', '\u{2660}', '\u{2022}',
    '\u{25d8}', '\u{25cb}', '\u{25d9}', '\u{2642}', '\u{2640}', '\u{266a}', '\u{266b}', '\u{263c}',
    '\u{25ba}', '\u{25c4}', '\u{2195}', '\u{203c}', '\u{00b6}', '\u{00a7}', '\u{25ac}', '\u{21a8}',
    '\u{2191}', '\u{2193}', '\u{2192}', '\u{2190}', '\u{221f}', '\u{2194}', '\u{25b2}', '\u{25bc}',
    '\u{0020}', '\u{0021}', '\u{0022}', '\u{0023}', '\u{0024}', '\u{0025}', '\u{0026}', '\u{0027}',
    '\u{0028}', '\u{0029}', '\u{002a}', '\u{002b}', '\u{002c}', '\u{002d}', '\u{002e}', '\u{002f}',
    '\u{0030}', '\u{0031}', '\u{0032}', '\u{0033}', '\u{0034}', '\u{0035}', '\u{0036}', '\u{0037}',
    '\u{0038}', '\u{0039}', '\u{003a}', '\u{003b}', '\u{003c}', '\u{003d}', '\u{003e}', '\u{003f}',
    '\u{0040}', '\u{0041}', '\u{0042}', '\u{0043}', '\u{0044}', '\u{0045}', '\u{0046}', '\u{0047}',
    '\u{0048}', '\u{0049}', '\u{004a}', '\u{004b}', '\u{004c}', '\u{004d}', '\u{004e}', '\u{004f}',
    '\u{0050}', '\u{0051}', '\u{0052}', '\u{0053}', '\u{0054}', '\u{0055}', '\u{0056}', '\u{0057}',
    '\u{0058}', '\u{0059}', '\u{005a}', '\u{005b}', '\u{005c}', '\u{005d}', '\u{005e}', '\u{005f}',
    '\u{0060}', '\u{0061}', '\u{0062}', '\u{0063}', '\u{0064}', '\u{0065}', '\u{0066}', '\u{0067}',
    '\u{0068}', '\u{0069}', '\u{006a}', '\u{006b}', '\u{006c}', '\u{006d}', '\u{006e}', '\u{006f}',
    '\u{0070}', '\u{0071}', '\u{0072}', '\u{0073}', '\u{0074}', '\u{0075}', '\u{0076}', '\u{0077}',
    '\u{0078}', '\u{0079}', '\u{007a}', '\u{007b}', '\u{007c}', '\u{007d}', '\u{007e}', '\u{2302}',
    '\u{00c7}', '\u{00fc}', '\u{00e9}', '\u{00e2}', '\u{00e4}', '\u{00e0}', '\u{00e5}', '\u{00e7}',
    '\u{00ea}', '\u{00eb}', '\u{00e8}', '\u{00ef}', '\u{00ee}', '\u{00ec}', '\u{00c4}', '\u{00c5}',
    '\u{00c9}', '\u{00e6}', '\u{00c6}', '\u{00f4}', '\u{00f6}', '\u{00f2}', '\u{00fb}', '\u{00f9}',
    '\u{00ff}', '\u{00d6}', '\u{00dc}', '\u{00a2}', '\u{00a3}', '\u{00a5}', '\u{20a7}', '\u{0192}',
    '\u{00e1}', '\u{00ed}', '\u{00f3}', '\u{00fa}', '\u{00f1}', '\u{00d1}', '\u{00aa}', '\u{00ba}',
    '\u{00bf}', '\u{2310}', '\u{00ac}', '\u{00bd}', '\u{00bc}', '\u{00a1}', '\u{00ab}', '\u{00bb}',
    '\u{2591}', '\u{2592}', '\u{2593}', '\u{2502}', '\u{2524}', '\u{2561}', '\u{2562}', '\u{2556}',
    '\u{2555}', '\u{2563}', '\u{2551}', '\u{2557}', '\u{255d}', '\u{255c}', '\u{255b}', '\u{2510}',
    '\u{2514}', '\u{2534}', '\u{252c}', '\u{251c}', '\u{2500}', '\u{253c}', '\u{255e}', '\u{255f}',
    '\u{255a}', '\u{2554}', '\u{2569}', '\u{2566}', '\u{2560}', '\u{2550}', '\u{256c}', '\u{2567}',
    '\u{2568}', '\u{2564}', '\u{2565}', '\u{2559}', '\u{2558}', '\u{2552}', '\u{2553}', '\u{256b}',
    '\u{256a}', '\u{2518}', '\u{250c}', '\u{2588}', '\u{2584}', '\u{258c}', '\u{2590}', '\u{2580}',
    '\u{03b1}', '\u{00df}', '\u{0393}', '\u{03c0}', '\u{03a3}', '\u{03c3}', '\u{00b5}', '\u{03c4}',
    '\u{03a6}', '\u{0398}', '\u{03a9}', '\u{03b4}', '\u{221e}', '\u{03c6}', '\u{03b5}', '\u{2229}',
    '\u{2261}', '\u{00b1}', '\u{2265}', '\u{2264}', '\u{2320}', '\u{2321}', '\u{00f7}', '\u{2248}',
    '\u{00b0}', '\u{2219}', '\u{00b7}', '\u{221a}', '\u{207f}', '\u{00b2}', '\u{25a0}', '\u{25a1}',
];

pub fn from(ch: u8) -> char {
    FROM[usize::from(ch)]
}

pub fn to(ch: char) -> (u16, u16) {
    (match ch {
        '\u{0000}' => 0x00, // NUL
        '\u{263a}' => 0x01, // WHITE SMILING FACE
        '\u{263b}' => 0x02, // BLACK SMILING FACE
        '\u{2665}' => 0x03, // BLACK HEART SUIT
        '\u{2666}' => 0x04, // BLACK DIAMOND SUIT
        '\u{2663}' => 0x05, // BLACK CLUB SUIT
        '\u{2660}' => 0x06, // BLACK SPADE SUIT
        '\u{2022}' => 0x07, // BULLET
        '\u{25d8}' => 0x08, // INVERSE BULLET
        '\u{25cb}' => 0x09, // WHITE CIRCLE
        '\u{25d9}' => 0x0a, // INVERSE WHITE CIRCLE
        '\u{2642}' => 0x0b, // MALE SIGN
        '\u{2640}' => 0x0c, // FEMALE SIGN
        '\u{266a}' => 0x0d, // EIGHTH NOTE
        '\u{266b}' => 0x0e, // BEAMED EIGHTH NOTES
        '\u{263c}' => 0x0f, // WHITE SUN WITH RAYS
        '\u{25ba}' => 0x10, // BLACK RIGHT-POINTING POINTER
        '\u{25c4}' => 0x11, // BLACK LEFT-POINTING POINTER
        '\u{2195}' => 0x12, // UP DOWN ARROW
        '\u{203c}' => 0x13, // DOUBLE EXCLAMATION MARK
        '\u{00b6}' => 0x14, // PILCROW SIGN
        '\u{00a7}' => 0x15, // SECTION SIGN
        '\u{25ac}' => 0x16, // BLACK RECTANGLE
        '\u{21a8}' => 0x17, // UP DOWN ARROW WITH BASE
        '\u{2191}' => 0x18, // UPWARDS ARROW
        '\u{2193}' => 0x19, // DOWNWARDS ARROW
        '\u{2192}' => 0x1a, // RIGHTWARDS ARROW
        '\u{2190}' => 0x1b, // LEFTWARDS ARROW
        '\u{221f}' => 0x1c, // RIGHT ANGLE
        '\u{2194}' => 0x1d, // LEFT RIGHT ARROW
        '\u{25b2}' => 0x1e, // BLACK UP-POINTING TRIANGLE
        '\u{25bc}' => 0x1f, // BLACK DOWN-POINTING TRIANGLE
        '\u{0020}' => 0x20, // SPACE
        '\u{0021}' => 0x21, // EXCLAMATION MARK
        '\u{0022}' => 0x22, // QUOTATION MARK
        '\u{0023}' => 0x23, // NUMBER SIGN
        '\u{0024}' => 0x24, // DOLLAR SIGN
        '\u{0025}' => 0x25, // PERCENT SIGN
        '\u{0026}' => 0x26, // AMPERSAND
        '\u{0027}' => 0x27, // APOSTROPHE
        '\u{0028}' => 0x28, // LEFT PARENTHESIS
        '\u{0029}' => 0x29, // RIGHT PARENTHESIS
        '\u{002a}' => 0x2a, // ASTERISK
        '\u{002b}' => 0x2b, // PLUS SIGN
        '\u{002c}' => 0x2c, // COMMA
        '\u{002d}' => 0x2d, // HYPHEN-MINUS
        '\u{002e}' => 0x2e, // FULL STOP
        '\u{002f}' => 0x2f, // SOLIDUS
        '\u{0030}' => 0x30, // DIGIT ZERO
        '\u{0031}' => 0x31, // DIGIT ONE
        '\u{0032}' => 0x32, // DIGIT TWO
        '\u{0033}' => 0x33, // DIGIT THREE
        '\u{0034}' => 0x34, // DIGIT FOUR
        '\u{0035}' => 0x35, // DIGIT FIVE
        '\u{0036}' => 0x36, // DIGIT SIX
        '\u{0037}' => 0x37, // DIGIT SEVEN
        '\u{0038}' => 0x38, // DIGIT EIGHT
        '\u{0039}' => 0x39, // DIGIT NINE
        '\u{003a}' => 0x3a, // COLON
        '\u{003b}' => 0x3b, // SEMICOLON
        '\u{003c}' => 0x3c, // LESS-THAN SIGN
        '\u{003d}' => 0x3d, // EQUALS SIGN
        '\u{003e}' => 0x3e, // GREATER-THAN SIGN
        '\u{003f}' => 0x3f, // QUESTION MARK
        '\u{0040}' => 0x40, // COMMERCIAL AT
        '\u{0041}' => 0x41, // LATIN CAPITAL LETTER A
        '\u{0042}' => 0x42, // LATIN CAPITAL LETTER B
        '\u{0043}' => 0x43, // LATIN CAPITAL LETTER C
        '\u{0044}' => 0x44, // LATIN CAPITAL LETTER D
        '\u{0045}' => 0x45, // LATIN CAPITAL LETTER E
        '\u{0046}' => 0x46, // LATIN CAPITAL LETTER F
        '\u{0047}' => 0x47, // LATIN CAPITAL LETTER G
        '\u{0048}' => 0x48, // LATIN CAPITAL LETTER H
        '\u{0049}' => 0x49, // LATIN CAPITAL LETTER I
        '\u{004a}' => 0x4a, // LATIN CAPITAL LETTER J
        '\u{004b}' => 0x4b, // LATIN CAPITAL LETTER K
        '\u{004c}' => 0x4c, // LATIN CAPITAL LETTER L
        '\u{004d}' => 0x4d, // LATIN CAPITAL LETTER M
        '\u{004e}' => 0x4e, // LATIN CAPITAL LETTER N
        '\u{004f}' => 0x4f, // LATIN CAPITAL LETTER O
        '\u{0050}' => 0x50, // LATIN CAPITAL LETTER P
        '\u{0051}' => 0x51, // LATIN CAPITAL LETTER Q
        '\u{0052}' => 0x52, // LATIN CAPITAL LETTER R
        '\u{0053}' => 0x53, // LATIN CAPITAL LETTER S
        '\u{0054}' => 0x54, // LATIN CAPITAL LETTER T
        '\u{0055}' => 0x55, // LATIN CAPITAL LETTER U
        '\u{0056}' => 0x56, // LATIN CAPITAL LETTER V
        '\u{0057}' => 0x57, // LATIN CAPITAL LETTER W
        '\u{0058}' => 0x58, // LATIN CAPITAL LETTER X
        '\u{0059}' => 0x59, // LATIN CAPITAL LETTER Y
        '\u{005a}' => 0x5a, // LATIN CAPITAL LETTER Z
        '\u{005b}' => 0x5b, // LEFT SQUARE BRACKET
        '\u{005c}' => 0x5c, // REVERSE SOLIDUS
        '\u{005d}' => 0x5d, // RIGHT SQUARE BRACKET
        '\u{005e}' => 0x5e, // CIRCUMFLEX ACCENT
        '\u{005f}' => 0x5f, // LOW LINE
        '\u{0060}' => 0x60, // GRAVE ACCENT
        '\u{0061}' => 0x61, // LATIN SMALL LETTER A
        '\u{0062}' => 0x62, // LATIN SMALL LETTER B
        '\u{0063}' => 0x63, // LATIN SMALL LETTER C
        '\u{0064}' => 0x64, // LATIN SMALL LETTER D
        '\u{0065}' => 0x65, // LATIN SMALL LETTER E
        '\u{0066}' => 0x66, // LATIN SMALL LETTER F
        '\u{0067}' => 0x67, // LATIN SMALL LETTER G
        '\u{0068}' => 0x68, // LATIN SMALL LETTER H
        '\u{0069}' => 0x69, // LATIN SMALL LETTER I
        '\u{006a}' => 0x6a, // LATIN SMALL LETTER J
        '\u{006b}' => 0x6b, // LATIN SMALL LETTER K
        '\u{006c}' => 0x6c, // LATIN SMALL LETTER L
        '\u{006d}' => 0x6d, // LATIN SMALL LETTER M
        '\u{006e}' => 0x6e, // LATIN SMALL LETTER N
        '\u{006f}' => 0x6f, // LATIN SMALL LETTER O
        '\u{0070}' => 0x70, // LATIN SMALL LETTER P
        '\u{0071}' => 0x71, // LATIN SMALL LETTER Q
        '\u{0072}' => 0x72, // LATIN SMALL LETTER R
        '\u{0073}' => 0x73, // LATIN SMALL LETTER S
        '\u{0074}' => 0x74, // LATIN SMALL LETTER T
        '\u{0075}' => 0x75, // LATIN SMALL LETTER U
        '\u{0076}' => 0x76, // LATIN SMALL LETTER V
        '\u{0077}' => 0x77, // LATIN SMALL LETTER W
        '\u{0078}' => 0x78, // LATIN SMALL LETTER X
        '\u{0079}' => 0x79, // LATIN SMALL LETTER Y
        '\u{007a}' => 0x7a, // LATIN SMALL LETTER Z
        '\u{007b}' => 0x7b, // LEFT CURLY BRACKET
        '\u{007c}' => 0x7c, // VERTICAL LINE
        '\u{007d}' => 0x7d, // RIGHT CURLY BRACKET
        '\u{007e}' => 0x7e, // TILDE
        '\u{2302}' => 0x7f, // HOUSE
        '\u{00c7}' => 0x80, // LATIN CAPITAL LETTER C WITH CEDILLA
        '\u{00fc}' => 0x81, // LATIN SMALL LETTER U WITH DIAERESIS
        '\u{00e9}' => 0x82, // LATIN SMALL LETTER E WITH ACUTE
        '\u{00e2}' => 0x83, // LATIN SMALL LETTER A WITH CIRCUMFLEX
        '\u{00e4}' => 0x84, // LATIN SMALL LETTER A WITH DIAERESIS
        '\u{00e0}' => 0x85, // LATIN SMALL LETTER A WITH GRAVE
        '\u{00e5}' => 0x86, // LATIN SMALL LETTER A WITH RING ABOVE
        '\u{00e7}' => 0x87, // LATIN SMALL LETTER C WITH CEDILLA
        '\u{00ea}' => 0x88, // LATIN SMALL LETTER E WITH CIRCUMFLEX
        '\u{00eb}' => 0x89, // LATIN SMALL LETTER E WITH DIAERESIS
        '\u{00e8}' => 0x8a, // LATIN SMALL LETTER E WITH GRAVE
        '\u{00ef}' => 0x8b, // LATIN SMALL LETTER I WITH DIAERESIS
        '\u{00ee}' => 0x8c, // LATIN SMALL LETTER I WITH CIRCUMFLEX
        '\u{00ec}' => 0x8d, // LATIN SMALL LETTER I WITH GRAVE
        '\u{00c4}' => 0x8e, // LATIN CAPITAL LETTER A WITH DIAERESIS
        '\u{00c5}' => 0x8f, // LATIN CAPITAL LETTER A WITH RING ABOVE
        '\u{00c9}' => 0x90, // LATIN CAPITAL LETTER E WITH ACUTE
        '\u{00e6}' => 0x91, // LATIN SMALL LETTER AE
        '\u{00c6}' => 0x92, // LATIN CAPITAL LETTER AE
        '\u{00f4}' => 0x93, // LATIN SMALL LETTER O WITH CIRCUMFLEX
        '\u{00f6}' => 0x94, // LATIN SMALL LETTER O WITH DIAERESIS
        '\u{00f2}' => 0x95, // LATIN SMALL LETTER O WITH GRAVE
        '\u{00fb}' => 0x96, // LATIN SMALL LETTER U WITH CIRCUMFLEX
        '\u{00f9}' => 0x97, // LATIN SMALL LETTER U WITH GRAVE
        '\u{00ff}' => 0x98, // LATIN SMALL LETTER Y WITH DIAERESIS
        '\u{00d6}' => 0x99, // LATIN CAPITAL LETTER O WITH DIAERESIS
        '\u{00dc}' => 0x9a, // LATIN CAPITAL LETTER U WITH DIAERESIS
        '\u{00a2}' => 0x9b, // CENT SIGN
        '\u{00a3}' => 0x9c, // POUND SIGN
        '\u{00a5}' => 0x9d, // YEN SIGN
        '\u{20a7}' => 0x9e, // PESETA SIGN
        '\u{0192}' => 0x9f, // LATIN SMALL LETTER F WITH HOOK
        '\u{00e1}' => 0xa0, // LATIN SMALL LETTER A WITH ACUTE
        '\u{00ed}' => 0xa1, // LATIN SMALL LETTER I WITH ACUTE
        '\u{00f3}' => 0xa2, // LATIN SMALL LETTER O WITH ACUTE
        '\u{00fa}' => 0xa3, // LATIN SMALL LETTER U WITH ACUTE
        '\u{00f1}' => 0xa4, // LATIN SMALL LETTER N WITH TILDE
        '\u{00d1}' => 0xa5, // LATIN CAPITAL LETTER N WITH TILDE
        '\u{00aa}' => 0xa6, // FEMININE ORDINAL INDICATOR
        '\u{00ba}' => 0xa7, // MASCULINE ORDINAL INDICATOR
        '\u{00bf}' => 0xa8, // INVERTED QUESTION MARK
        '\u{2310}' => 0xa9, // REVERSED NOT SIGN
        '\u{00ac}' => 0xaa, // NOT SIGN
        '\u{00bd}' => 0xab, // VULGAR FRACTION ONE HALF
        '\u{00bc}' => 0xac, // VULGAR FRACTION ONE QUARTER
        '\u{00a1}' => 0xad, // INVERTED EXCLAMATION MARK
        '\u{00ab}' => 0xae, // LEFT-POINTING DOUBLE ANGLE QUOTATION MARK
        '\u{00bb}' => 0xaf, // RIGHT-POINTING DOUBLE ANGLE QUOTATION MARK
        '\u{2591}' => 0xb0, // LIGHT SHADE
        '\u{2592}' => 0xb1, // MEDIUM SHADE
        '\u{2593}' => 0xb2, // DARK SHADE
        '\u{2502}' => 0xb3, // BOX DRAWINGS LIGHT VERTICAL
        '\u{2524}' => 0xb4, // BOX DRAWINGS LIGHT VERTICAL AND LEFT
        '\u{2561}' => 0xb5, // BOX DRAWINGS VERTICAL SINGLE AND LEFT DOUBLE
        '\u{2562}' => 0xb6, // BOX DRAWINGS VERTICAL DOUBLE AND LEFT SINGLE
        '\u{2556}' => 0xb7, // BOX DRAWINGS DOWN DOUBLE AND LEFT SINGLE
        '\u{2555}' => 0xb8, // BOX DRAWINGS DOWN SINGLE AND LEFT DOUBLE
        '\u{2563}' => 0xb9, // BOX DRAWINGS DOUBLE VERTICAL AND LEFT
        '\u{2551}' => 0xba, // BOX DRAWINGS DOUBLE VERTICAL
        '\u{2557}' => 0xbb, // BOX DRAWINGS DOUBLE DOWN AND LEFT
        '\u{255d}' => 0xbc, // BOX DRAWINGS DOUBLE UP AND LEFT
        '\u{255c}' => 0xbd, // BOX DRAWINGS UP DOUBLE AND LEFT SINGLE
        '\u{255b}' => 0xbe, // BOX DRAWINGS UP SINGLE AND LEFT DOUBLE
        '\u{2510}' => 0xbf, // BOX DRAWINGS LIGHT DOWN AND LEFT
        '\u{2514}' => 0xc0, // BOX DRAWINGS LIGHT UP AND RIGHT
        '\u{2534}' => 0xc1, // BOX DRAWINGS LIGHT UP AND HORIZONTAL
        '\u{252c}' => 0xc2, // BOX DRAWINGS LIGHT DOWN AND HORIZONTAL
        '\u{251c}' => 0xc3, // BOX DRAWINGS LIGHT VERTICAL AND RIGHT
        '\u{2500}' => 0xc4, // BOX DRAWINGS LIGHT HORIZONTAL
        '\u{253c}' => 0xc5, // BOX DRAWINGS LIGHT VERTICAL AND HORIZONTAL
        '\u{255e}' => 0xc6, // BOX DRAWINGS VERTICAL SINGLE AND RIGHT DOUBLE
        '\u{255f}' => 0xc7, // BOX DRAWINGS VERTICAL DOUBLE AND RIGHT SINGLE
        '\u{255a}' => 0xc8, // BOX DRAWINGS DOUBLE UP AND RIGHT
        '\u{2554}' => 0xc9, // BOX DRAWINGS DOUBLE DOWN AND RIGHT
        '\u{2569}' => 0xca, // BOX DRAWINGS DOUBLE UP AND HORIZONTAL
        '\u{2566}' => 0xcb, // BOX DRAWINGS DOUBLE DOWN AND HORIZONTAL
        '\u{2560}' => 0xcc, // BOX DRAWINGS DOUBLE VERTICAL AND RIGHT
        '\u{2550}' => 0xcd, // BOX DRAWINGS DOUBLE HORIZONTAL
        '\u{256c}' => 0xce, // BOX DRAWINGS DOUBLE VERTICAL AND HORIZONTAL
        '\u{2567}' => 0xcf, // BOX DRAWINGS UP SINGLE AND HORIZONTAL DOUBLE
        '\u{2568}' => 0xd0, // BOX DRAWINGS UP DOUBLE AND HORIZONTAL SINGLE
        '\u{2564}' => 0xd1, // BOX DRAWINGS DOWN SINGLE AND HORIZONTAL DOUBLE
        '\u{2565}' => 0xd2, // BOX DRAWINGS DOWN DOUBLE AND HORIZONTAL SINGLE
        '\u{2559}' => 0xd3, // BOX DRAWINGS UP DOUBLE AND RIGHT SINGLE
        '\u{2558}' => 0xd4, // BOX DRAWINGS UP SINGLE AND RIGHT DOUBLE
        '\u{2552}' => 0xd5, // BOX DRAWINGS DOWN SINGLE AND RIGHT DOUBLE
        '\u{2553}' => 0xd6, // BOX DRAWINGS DOWN DOUBLE AND RIGHT SINGLE
        '\u{256b}' => 0xd7, // BOX DRAWINGS VERTICAL DOUBLE AND HORIZONTAL SINGLE
        '\u{256a}' => 0xd8, // BOX DRAWINGS VERTICAL SINGLE AND HORIZONTAL DOUBLE
        '\u{2518}' => 0xd9, // BOX DRAWINGS LIGHT UP AND LEFT
        '\u{250c}' => 0xda, // BOX DRAWINGS LIGHT DOWN AND RIGHT
        '\u{2588}' => 0xdb, // FULL BLOCK
        '\u{2584}' => 0xdc, // LOWER HALF BLOCK
        '\u{258c}' => 0xdd, // LEFT HALF BLOCK
        '\u{2590}' => 0xde, // RIGHT HALF BLOCK
        '\u{2580}' => 0xdf, // UPPER HALF BLOCK
        '\u{03b1}' => 0xe0, // GREEK SMALL LETTER ALPHA
        '\u{00df}' => 0xe1, // LATIN SMALL LETTER SHARP S
        '\u{0393}' => 0xe2, // GREEK CAPITAL LETTER GAMMA
        '\u{03c0}' => 0xe3, // GREEK SMALL LETTER PI
        '\u{03a3}' => 0xe4, // GREEK CAPITAL LETTER SIGMA
        '\u{03c3}' => 0xe5, // GREEK SMALL LETTER SIGMA
        '\u{00b5}' => 0xe6, // MICRO SIGN
        '\u{03c4}' => 0xe7, // GREEK SMALL LETTER TAU
        '\u{03a6}' => 0xe8, // GREEK CAPITAL LETTER PHI
        '\u{0398}' => 0xe9, // GREEK CAPITAL LETTER THETA
        '\u{03a9}' => 0xea, // GREEK CAPITAL LETTER OMEGA
        '\u{03b4}' => 0xeb, // GREEK SMALL LETTER DELTA
        '\u{221e}' => 0xec, // INFINITY
        '\u{03c6}' => 0xed, // GREEK SMALL LETTER PHI
        '\u{03b5}' => 0xee, // GREEK SMALL LETTER EPSILON
        '\u{2229}' => 0xef, // INTERSECTION
        '\u{2261}' => 0xf0, // IDENTICAL TO
        '\u{00b1}' => 0xf1, // PLUS-MINUS SIGN
        '\u{2265}' => 0xf2, // GREATER-THAN OR EQUAL TO
        '\u{2264}' => 0xf3, // LESS-THAN OR EQUAL TO
        '\u{2320}' => 0xf4, // TOP HALF INTEGRAL
        '\u{2321}' => 0xf5, // BOTTOM HALF INTEGRAL
        '\u{00f7}' => 0xf6, // DIVISION SIGN
        '\u{2248}' => 0xf7, // ALMOST EQUAL TO
        '\u{00b0}' => 0xf8, // DEGREE SIGN
        '\u{2219}' => 0xf9, // BULLET OPERATOR
        '\u{00b7}' => 0xfa, // MIDDLE DOT
        '\u{221a}' => 0xfb, // SQUARE ROOT
        '\u{207f}' => 0xfc, // SUPERSCRIPT LATIN SMALL LETTER N
        '\u{00b2}' => 0xfd, // SUPERSCRIPT TWO
        '\u{25a0}' => 0xfe, // BLACK SQUARE
        '\u{25a1}' => 0xff, // WHITE SQUARE
        _ => 254, // Unknown
    }, 0)
}
