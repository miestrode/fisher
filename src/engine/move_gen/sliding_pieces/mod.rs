// See: https://www.chessprogramming.org/Kogge-Stone_Algorithm#Occluded_Fill
// NOTE: "slides" do NOT include captures and do allow the piece to NOT move.
use crate::{
    engine::utility::{NOT_A_FILE, NOT_H_FILE},
    game::board::BitBoard,
};

fn get_up_slides(mut pieces: BitBoard, mut empty: BitBoard) -> BitBoard {
    pieces |= empty & (pieces << 8);
    empty &= (empty << 8);
    pieces |= empty & (pieces << 16);
    empty &= (empty << 16);

    pieces | empty & (pieces << 32)
}

fn get_down_slides(mut pieces: BitBoard, mut empty: BitBoard) -> BitBoard {
    pieces |= empty & (pieces >> 8);
    empty &= (empty >> 8);
    pieces |= empty & (pieces >> 16);
    empty &= (empty >> 16);

    pieces | empty & (pieces >> 32)
}

fn get_right_slides(mut pieces: BitBoard, mut empty: BitBoard) -> BitBoard {
    empty &= NOT_A_FILE;
    pieces |= empty & (pieces << 1);
    empty &= (empty << 1);
    pieces |= empty & (pieces << 2);
    empty &= (empty << 2);

    pieces | empty & (pieces << 4)
}

fn get_left_slides(mut pieces: BitBoard, mut empty: BitBoard) -> BitBoard {
    empty &= NOT_H_FILE;
    pieces |= empty & (pieces >> 1);
    empty &= (empty >> 1);
    pieces |= empty & (pieces >> 2);
    empty &= (empty >> 2);

    pieces | empty & (pieces >> 4)
}

fn get_up_right_slides(mut pieces: BitBoard, mut empty: BitBoard) -> BitBoard {
    empty &= NOT_A_FILE;
    pieces |= empty & (pieces << 9);
    empty &= (empty << 9);
    pieces |= empty & (pieces << 18);
    empty &= (empty << 18);

    pieces | empty & (pieces << 36)
}

fn get_down_right_slides(mut pieces: BitBoard, mut empty: BitBoard) -> BitBoard {
    empty &= NOT_H_FILE;
    pieces |= empty & (pieces >> 7);
    empty &= (empty >> 7);
    pieces |= empty & (pieces >> 14);
    empty &= (empty >> 14);

    pieces | empty & (pieces >> 28)
}

fn get_up_left_slides(mut pieces: BitBoard, mut empty: BitBoard) -> BitBoard {
    empty &= NOT_H_FILE;
    pieces |= empty & (pieces >> 9);
    empty &= (empty >> 9);
    pieces |= empty & (pieces >> 18);
    empty &= (empty >> 18);

    pieces | empty & (pieces >> 36)
}

fn get_down_left_slides(mut pieces: BitBoard, mut empty: BitBoard) -> BitBoard {
    empty &= NOT_H_FILE;
    pieces |= empty & (pieces << 7);
    empty &= (empty << 7);
    pieces |= empty & (pieces << 14);
    empty &= (empty << 14);

    pieces | empty & (pieces << 28)
}

pub fn get_cross_attacks(pieces: BitBoard, empty: BitBoard, friendly_pieces: BitBoard) -> BitBoard {
    get_up_slides(pieces, empty).move_up()
        | get_down_slides(pieces, empty).move_down()
        | get_left_slides(pieces, empty).move_left()
        | get_right_slides(pieces, empty).move_right() & !friendly_pieces
}

pub fn get_diagonal_attacks(
    pieces: BitBoard,
    empty: BitBoard,
    friendly_pieces: BitBoard,
) -> BitBoard {
    get_up_left_slides(pieces, empty).move_up().move_left()
        | get_up_right_slides(pieces, empty).move_up().move_right()
        | get_down_left_slides(pieces, empty).move_down().move_left()
        | get_down_right_slides(pieces, empty)
            .move_down()
            .move_right()
            & !friendly_pieces
}
