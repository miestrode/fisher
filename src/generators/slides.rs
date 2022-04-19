// See: https://www.chessprogramming.org/Kogge-Stone_Algorithm#Occluded_Fill
use crate::{BitBoard, NOT_A_FILE, NOT_H_FILE};

pub fn get_up_attacks(mut pieces: BitBoard, mut empty: BitBoard) -> BitBoard {
    pieces |= empty & (pieces << 8);
    empty &= empty << 8;
    pieces |= empty & (pieces << 16);
    empty &= empty << 16;
    pieces |= empty & (pieces << 32);

    pieces.move_up(1)
}

pub fn get_down_attacks(mut pieces: BitBoard, mut empty: BitBoard) -> BitBoard {
    pieces |= empty & (pieces >> 8);
    empty &= empty >> 8;
    pieces |= empty & (pieces >> 16);
    empty &= empty >> 16;
    pieces |= empty & (pieces >> 32);

    pieces.move_down(1)
}

pub fn get_right_attacks(mut pieces: BitBoard, mut empty: BitBoard) -> BitBoard {
    empty &= NOT_A_FILE;
    pieces |= empty & (pieces << 1);
    empty &= empty << 1;
    pieces |= empty & (pieces << 2);
    empty &= empty << 2;
    pieces |= empty & (pieces << 4);

    pieces.move_right()
}

pub fn get_left_attacks(mut pieces: BitBoard, mut empty: BitBoard) -> BitBoard {
    empty &= NOT_H_FILE;
    pieces |= empty & (pieces >> 1);
    empty &= empty >> 1;
    pieces |= empty & (pieces >> 2);
    empty &= empty >> 2;
    pieces |= empty & (pieces >> 4);

    pieces.move_left()
}

pub fn get_up_right_attacks(mut pieces: BitBoard, mut empty: BitBoard) -> BitBoard {
    empty &= NOT_A_FILE;
    pieces |= empty & (pieces << 9);
    empty &= empty << 9;
    pieces |= empty & (pieces << 18);
    empty &= empty << 18;
    pieces |= empty & (pieces << 36);

    pieces.move_up_right()
}

pub fn get_down_right_attacks(mut pieces: BitBoard, mut empty: BitBoard) -> BitBoard {
    empty &= NOT_A_FILE;
    pieces |= empty & (pieces >> 7);
    empty &= empty >> 7;
    pieces |= empty & (pieces >> 14);
    empty &= empty >> 14;
    pieces |= empty & (pieces >> 28);

    pieces.move_down_right()
}

pub fn get_up_left_attacks(mut pieces: BitBoard, mut empty: BitBoard) -> BitBoard {
    empty &= NOT_H_FILE;
    pieces |= empty & (pieces << 7);
    empty &= empty << 7;
    pieces |= empty & (pieces << 14);
    empty &= empty << 14;
    pieces |= empty & (pieces << 28);

    pieces.move_up_left()
}

pub fn get_down_left_attacks(mut pieces: BitBoard, mut empty: BitBoard) -> BitBoard {
    empty &= NOT_H_FILE;
    pieces |= empty & (pieces >> 9);
    empty &= empty >> 9;
    pieces |= empty & (pieces >> 18);
    empty &= empty >> 18;
    pieces |= empty & (pieces >> 36);

    pieces.move_down_left()
}
