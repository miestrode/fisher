use crate::game::board::BitBoard;

pub const DE_BRUIJN_INDICES: [u8; 64] = [
    0, 47, 1, 56, 48, 27, 2, 60, 57, 49, 41, 37, 28, 16, 3, 61, 54, 58, 35, 52, 50, 42, 21, 44, 38,
    32, 29, 23, 17, 11, 4, 62, 46, 55, 26, 59, 40, 36, 15, 53, 34, 51, 20, 43, 31, 22, 10, 45, 25,
    39, 14, 33, 19, 30, 9, 24, 13, 18, 8, 12, 7, 6, 5, 63,
];

pub const NOT_A_FILE: BitBoard =
    BitBoard(0b0111111101111111011111110111111101111111011111110111111101111111);
pub const NOT_H_FILE: BitBoard =
    BitBoard(0b1111111011111110111111101111111011111110111111101111111011111111);

pub const AVOID_WRAPS: [BitBoard; 8] = [
    BitBoard(0xfefefefefefefe00),
    BitBoard(0xfefefefefefefefe),
    BitBoard(0x00fefefefefefefe),
    BitBoard(0x00ffffffffffffff),
    BitBoard(0x007f7f7f7f7f7f7f),
    BitBoard(0x7f7f7f7f7f7f7f7f),
    BitBoard(0x7f7f7f7f7f7f7f00),
    BitBoard(0xffffffffffffff00),
];
