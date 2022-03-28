pub mod bishop;
pub mod queen;
pub mod rook;

// See: https://www.chessprogramming.org/Kogge-Stone_Algorithm#Occluded_Fill
// NOTE: "slides" do NOT include captures and do allow the piece to NOT move.
use crate::{
    engine::utility::{NOT_A_FILE, NOT_H_FILE},
    game::board::BitBoard,
};

use super::Position;

pub enum RelDirection {
    UpLeft,
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    Other,
}

pub fn get_rel_dir(Position(observer): Position, Position(position): Position) -> RelDirection {
    assert_ne!(observer, position);
    let position_is_ahead = position > observer;

    // The only way the modulos can be equal is if you can reach it from A and B by removing or adding nines, since that's how you go about the board diagonally.
    if observer % 9 == position % 9 {
        if position_is_ahead {
            RelDirection::UpRight
        } else {
            RelDirection::DownLeft
        }
    } else if observer % 7 == position % 7 {
        // Same argument, but this time remember that we can add 7 to go on the "leftwards" diagonal.
        if position_is_ahead {
            RelDirection::UpLeft
        } else {
            RelDirection::DownRight
        }
    } else if observer / 8 == position / 8 {
        if position_is_ahead {
            RelDirection::Right
        } else {
            RelDirection::Left
        }
    } else if observer % 8 == position % 8 {
        if position_is_ahead {
            RelDirection::Up
        } else {
            RelDirection::Down
        }
    } else {
        RelDirection::Other
    }
}

pub fn get_up_slides(
    mut pieces: BitBoard,
    mut empty: BitBoard,
    friendly_pieces: BitBoard,
) -> BitBoard {
    pieces |= empty & (pieces << 8);
    empty &= empty << 8;
    pieces |= empty & (pieces << 16);
    empty &= empty << 16;
    pieces |= empty & (pieces << 32);

    pieces.move_up(1) & !friendly_pieces
}

pub fn get_down_slides(
    mut pieces: BitBoard,
    mut empty: BitBoard,
    friendly_pieces: BitBoard,
) -> BitBoard {
    pieces |= empty & (pieces >> 8);
    empty &= empty >> 8;
    pieces |= empty & (pieces >> 16);
    empty &= empty >> 16;
    pieces |= empty & (pieces >> 32);

    pieces.move_down(1) & !friendly_pieces
}

pub fn get_left_slides(
    mut pieces: BitBoard,
    mut empty: BitBoard,
    friendly_pieces: BitBoard,
) -> BitBoard {
    empty &= NOT_H_FILE;
    pieces |= empty & (pieces << 1);
    empty &= empty << 1;
    pieces |= empty & (pieces << 2);
    empty &= empty << 2;
    pieces |= empty & (pieces << 4);

    pieces.move_left(1) & !friendly_pieces
}

pub fn get_right_slides(
    mut pieces: BitBoard,
    mut empty: BitBoard,
    friendly_pieces: BitBoard,
) -> BitBoard {
    empty &= NOT_A_FILE;
    pieces |= empty & (pieces >> 1);
    empty &= empty >> 1;
    pieces |= empty & (pieces >> 2);
    empty &= empty >> 2;
    pieces |= empty & (pieces >> 4);

    pieces.move_right(1) & !friendly_pieces
}

pub fn get_up_right_slides(
    mut pieces: BitBoard,
    mut empty: BitBoard,
    friendly_pieces: BitBoard,
) -> BitBoard {
    empty &= NOT_A_FILE;
    pieces |= empty & (pieces << 9);
    empty &= empty << 9;
    pieces |= empty & (pieces << 18);
    empty &= empty << 18;
    pieces |= empty & (pieces << 36);

    pieces.move_up_right(1) & !friendly_pieces
}

pub fn get_down_right_slides(
    mut pieces: BitBoard,
    mut empty: BitBoard,
    friendly_pieces: BitBoard,
) -> BitBoard {
    empty &= NOT_A_FILE;
    pieces |= empty & (pieces >> 7);
    empty &= empty >> 7;
    pieces |= empty & (pieces >> 14);
    empty &= empty >> 14;
    pieces |= empty & (pieces >> 28);

    pieces.move_down_right(1) & !friendly_pieces
}

pub fn get_up_left_slides(
    mut pieces: BitBoard,
    mut empty: BitBoard,
    friendly_pieces: BitBoard,
) -> BitBoard {
    empty &= NOT_H_FILE;
    pieces |= empty & (pieces << 7);
    empty &= empty << 7;
    pieces |= empty & (pieces << 14);
    empty &= empty << 14;
    pieces |= empty & (pieces << 28);

    pieces.move_up_left(1) & !friendly_pieces
}

pub fn get_down_left_slides(
    mut pieces: BitBoard,
    mut empty: BitBoard,
    friendly_pieces: BitBoard,
) -> BitBoard {
    empty &= NOT_H_FILE;
    pieces |= empty & (pieces >> 9);
    empty &= empty >> 9;
    pieces |= empty & (pieces >> 18);
    empty &= empty >> 18;
    pieces |= empty & (pieces >> 36);

    pieces.move_down_left(1) & !friendly_pieces
}

pub fn get_cross_slides(
    mut pieces: BitBoard,
    mut empty: BitBoard,
    friendly_pieces: BitBoard,
) -> BitBoard {
    get_up_slides(pieces, empty, friendly_pieces)
        | get_right_slides(pieces, empty, friendly_pieces)
        | get_down_slides(pieces, empty, friendly_pieces)
        | get_left_slides(pieces, empty, friendly_pieces)
}

pub fn get_diagonal_slides(
    mut pieces: BitBoard,
    mut empty: BitBoard,
    friendly_pieces: BitBoard,
) -> BitBoard {
    get_up_left_slides(pieces, empty, friendly_pieces)
        | get_up_right_slides(pieces, empty, friendly_pieces)
        | get_down_right_slides(pieces, empty, friendly_pieces)
        | get_down_left_slides(pieces, empty, friendly_pieces)
}
