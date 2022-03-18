pub mod bishop;
pub mod queen;
pub mod rook;

// See: https://www.chessprogramming.org/Kogge-Stone_Algorithm#Occluded_Fill
// NOTE: "slides" do NOT include captures and do allow the piece to NOT move.
use crate::{
    engine::utility::{AVOID_WRAPS, NOT_A_FILE, NOT_H_FILE},
    game::board::BitBoard,
};

use super::Position;

#[derive(Debug, Clone, Copy)]
pub enum RelDirection {
    Flat,
    LeftDiagonal,
    Straight,
    RightDiagonal,
    Other,
}

pub fn gen_relative_direction(a: Position, b: Position) -> RelDirection {
    let (a, b) = (a.0, b.0);

    // The only way the modulos can be equal is if you can reach it from A and B by removing or adding nines, since that's how you go about the board diagonally.
    if a % 9 == b % 9 {
        RelDirection::RightDiagonal
    } else if a % 7 == b % 7 {
        // Same argument, but this time remember that we can add 7 to go on the "leftwards" diagonal.
        RelDirection::LeftDiagonal
    } else if a / 8 == b / 8 {
        // Remember: In Rust, division of integers is floored.
        RelDirection::Flat
    } else if a % 8 == b % 8 {
        RelDirection::Straight
    } else {
        RelDirection::Other
    }
}

fn get_up_slides(mut pieces: BitBoard, mut empty: BitBoard) -> BitBoard {
    pieces |= empty & (pieces << 8);
    empty &= empty << 8;
    pieces |= empty & (pieces << 16);
    empty &= empty << 16;
    pieces |= empty & (pieces << 32);

    pieces
}

fn get_down_slides(mut pieces: BitBoard, mut empty: BitBoard) -> BitBoard {
    pieces |= empty & (pieces >> 8);
    empty &= empty >> 8;
    pieces |= empty & (pieces >> 16);
    empty &= empty >> 16;
    pieces |= empty & (pieces >> 32);

    pieces
}

fn get_left_slides(mut pieces: BitBoard, mut empty: BitBoard) -> BitBoard {
    empty &= AVOID_WRAPS[1];
    pieces |= empty & (pieces << 1);
    empty &= empty << 1;
    pieces |= empty & (pieces << 2);
    empty &= empty << 2;
    pieces |= empty & (pieces << 4);

    pieces
}

fn get_right_slides(mut pieces: BitBoard, mut empty: BitBoard) -> BitBoard {
    empty &= AVOID_WRAPS[5];
    pieces |= empty & (pieces >> 1);
    empty &= empty >> 1;
    pieces |= empty & (pieces >> 2);
    empty &= empty >> 2;
    pieces |= empty & (pieces >> 4);

    pieces
}

fn get_up_right_slides(mut pieces: BitBoard, mut empty: BitBoard) -> BitBoard {
    empty &= AVOID_WRAPS[0];
    pieces |= empty & (pieces << 9);
    empty &= empty << 9;
    pieces |= empty & (pieces << 18);
    empty &= empty << 18;
    pieces |= empty & (pieces << 36);

    pieces
}

fn get_down_right_slides(mut pieces: BitBoard, mut empty: BitBoard) -> BitBoard {
    empty &= AVOID_WRAPS[2];
    pieces |= empty & (pieces >> 7);
    empty &= empty >> 7;
    pieces |= empty & (pieces >> 14);
    empty &= empty >> 14;
    pieces |= empty & (pieces >> 28);

    pieces
}

fn get_up_left_slides(mut pieces: BitBoard, mut empty: BitBoard) -> BitBoard {
    empty &= AVOID_WRAPS[6];
    pieces |= empty & (pieces << 7);
    empty &= empty << 7;
    pieces |= empty & (pieces << 14);
    empty &= empty << 14;
    pieces |= empty & (pieces << 28);

    pieces
}

fn get_down_left_slides(mut pieces: BitBoard, mut empty: BitBoard) -> BitBoard {
    empty &= AVOID_WRAPS[4];
    pieces |= empty & (pieces >> 9);
    empty &= empty >> 9;
    pieces |= empty & (pieces >> 18);
    empty &= empty >> 18;
    pieces |= empty & (pieces >> 36);

    pieces
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

// This function considers not moving the piece as a slide.
pub fn get_slides_on_empty_board(pieces: BitBoard) -> BitBoard {
    let empty = !BitBoard::empty();

    get_up_slides(pieces, empty)
        | get_left_slides(pieces, empty)
        | get_right_slides(pieces, empty)
        | get_down_slides(pieces, empty)
        | get_up_left_slides(pieces, empty)
        | get_down_left_slides(pieces, empty)
        | get_up_right_slides(pieces, empty)
        | get_down_right_slides(pieces, empty)
}
