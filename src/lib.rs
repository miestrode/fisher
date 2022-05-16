#![feature(const_trait_impl, const_for, const_mut_refs)]

use std::{
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, Not, Shl, Shr, Sub},
    str::FromStr,
};

use game::board::Board;
use generators::Square;

use crate::generators::MoveGen;

pub mod engine;
pub mod game;
pub mod generators;
pub mod tables;
pub mod uci;

pub const PROMOTION_PIECES: [PieceKind; 4] = [
    PieceKind::Queen,
    PieceKind::Rook,
    PieceKind::Bishop,
    PieceKind::Knight,
];

pub const LEFT_ROOK_ORIGINS: [Square; 2] = [Square::A1, Square::A8];
pub const RIGHT_ROOK_ORIGINS: [Square; 2] = [Square::H1, Square::H8];

pub const NOT_H_FILE: BitBoard =
    BitBoard(0b0111111101111111011111110111111101111111011111110111111101111111);
pub const NOT_A_FILE: BitBoard =
    BitBoard(0b1111111011111110111111101111111011111110111111101111111011111110);

pub const SECOND_RANK: BitBoard =
    BitBoard(0b0000000000000000000000000000000000000000000000001111111100000000);
pub const SEVENTH_RANK: BitBoard =
    BitBoard(0b0000000011111111000000000000000000000000000000000000000000000000);
pub const FIRST_RANK: BitBoard =
    BitBoard(0b0000000000000000000000000000000000000000000000000000000011111111);
pub const EIGHTH_RANK: BitBoard =
    BitBoard(0b1111111100000000000000000000000000000000000000000000000000000000);

pub const CASTLE_KS_SPACE: BitBoard =
    BitBoard(0b0000000000000000000000000000000000000000000000000000000001100000);
pub const CASTLE_QS_SPACE: BitBoard =
    BitBoard(0b0000000000000000000000000000000000000000000000000000000000001110);

#[derive(Clone, Copy, PartialEq)]
pub struct BitBoard(pub u64);

impl From<Square> for BitBoard {
    fn from(square: Square) -> Self {
        BitBoard(1 << square.0)
    }
}

impl BitBoard {
    pub const fn empty() -> Self {
        Self(0)
    }

    pub fn full() -> Self {
        Self(u64::MAX)
    }

    pub fn count_ones(&self) -> u32 {
        self.0.count_ones()
    }

    pub fn get_bit(&self, square: Square) -> bool {
        ((self.0 >> square.0) & 1) == 1
    }

    pub fn toggle_bit(&mut self, square: Square) {
        self.0 ^= 1 << square.0;
    }

    /*
    In this scenario, our function should behave like the following for it's inputs:
    1) When a one meets a moved one, it stays one.
    2) Otherwise, the cell becomes a zero.

    In other words, we need the "and" function.

    TODO: Verify that this code works.
    */
    pub fn smear_zeroes_up(self) -> Self {
        self.move_up(1) & self
    }

    // TODO: Same idea as above! Make sure this works.
    pub fn smear_zeroes_down(self) -> Self {
        self & self.move_down(1)
    }

    pub fn first_one_square(&self) -> Square {
        assert!(self.isnt_empty());

        Square(self.0.trailing_zeros())
    }

    pub fn move_bit(&mut self, origin: Square, target: Square) {
        self.toggle_bit(target); // Assumed to be on.
        self.toggle_bit(origin); // Assumed to be off.
    }

    pub fn turn_off(&mut self, square: Square) {
        self.0 &= !(1 << square.0)
    }

    pub fn turn_on(&mut self, square: Square) {
        self.0 |= 1 << square.0
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn is_single_1(&self) -> bool {
        self.0.is_power_of_two()
    }

    pub fn is_full(&self) -> bool {
        self.0 == u64::MAX
    }

    pub fn isnt_empty(&self) -> bool {
        self.0 != 0
    }

    pub fn pop_first_one(&mut self) -> Square {
        assert_ne!(self.0, 0);

        let square = self.first_one_square();

        // This is done to set the first one to a 0, since we are popping it.
        // See: https://www.chessprogramming.org/General_Setwise_Operations#Reset
        self.0 &= self.0 - 1;

        square
    }

    pub fn pfo_as_bitboard(&mut self) -> Self {
        assert_ne!(self.0, 0);

        // See: https://www.chessprogramming.org/General_Setwise_Operations#Isolation
        let isolated_one = self.0 & self.0.wrapping_neg(); // Compute the two's complement.

        // This is done to set the first one to a 0, since we are popping it.
        // See: https://www.chessprogramming.org/General_Setwise_Operations#Reset
        self.0 &= self.0 - 1;

        BitBoard(isolated_one)
    }

    pub fn pfo_with_bitboard(&mut self) -> (Square, Self) {
        assert_ne!(self.0, 0);

        // See: https://www.chessprogramming.org/General_Setwise_Operations#Isolation
        let isolated_one = self.0 & self.0.wrapping_neg(); // Compute the two's complement.

        // This is done to set the first one to a 0, since we are popping it.
        // See: https://www.chessprogramming.org/General_Setwise_Operations#Reset
        self.0 &= self.0 - 1;

        (
            Square(isolated_one.trailing_zeros()),
            BitBoard(isolated_one),
        )
    }

    // You are not able to move with an amount left or right due to wrapping issues. That is fine though, as the codebase doesn't need those features.
    pub const fn move_right(self) -> Self {
        (self << 1) & NOT_A_FILE
    }

    pub const fn move_left(self) -> Self {
        (self >> 1) & NOT_H_FILE
    }

    pub const fn move_up(self, amount: u32) -> Self {
        self << (8 * amount)
    }

    pub const fn move_down(self, amount: u32) -> Self {
        self >> (8 * amount)
    }

    pub const fn move_up_right(self) -> Self {
        (self << 9) & NOT_A_FILE
    }

    pub const fn move_up_left(self) -> Self {
        (self << 7) & NOT_H_FILE
    }

    pub const fn move_down_right(self) -> Self {
        (self >> 7) & NOT_A_FILE
    }

    pub const fn move_down_left(self) -> Self {
        (self >> 9) & NOT_H_FILE
    }
}

// These are utility implementations for conciseness so not every used operation is implemented.
impl const BitOr for BitBoard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl const BitAnd for BitBoard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl const Not for BitBoard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl const Shr<u32> for BitBoard {
    type Output = Self;

    fn shr(self, rhs: u32) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl const Shl<u32> for BitBoard {
    type Output = Self;

    fn shl(self, rhs: u32) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl const BitXor<BitBoard> for BitBoard {
    type Output = BitBoard;

    fn bitxor(self, rhs: BitBoard) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl const Sub<BitBoard> for BitBoard {
    type Output = BitBoard;

    fn sub(self, rhs: BitBoard) -> Self::Output {
        self & rhs ^ self
    }
}

#[derive(Clone, Copy)]
pub struct Pins {
    pub horizontal: BitBoard,
    pub vertical: BitBoard,
    pub diagonal: BitBoard,
    pub anti_diagonal: BitBoard,
}

impl Pins {
    pub fn new() -> Self {
        Self {
            horizontal: BitBoard::empty(),
            vertical: BitBoard::empty(),
            diagonal: BitBoard::empty(),
            anti_diagonal: BitBoard::empty(),
        }
    }

    // This will return the set of all squares this piece can occupy based on the active pins.
    pub fn get_pin_mask(&self, piece: BitBoard) -> BitBoard {
        if (piece & self.horizontal).isnt_empty() {
            self.horizontal
        } else if (piece & self.vertical).isnt_empty() {
            self.vertical
        } else if (piece & self.diagonal).isnt_empty() {
            self.diagonal
        } else if (piece & self.anti_diagonal).isnt_empty() {
            self.anti_diagonal
        } else {
            !BitBoard::empty()
        }
    }

    pub fn get_hv_pins(&self) -> BitBoard {
        self.horizontal | self.vertical
    }

    pub fn get_diag_pins(&self) -> BitBoard {
        self.diagonal | self.anti_diagonal
    }

    pub fn get_all_pins(&self) -> BitBoard {
        self.get_diag_pins() | self.get_hv_pins()
    }

    pub fn get_ape_diagonal(&self) -> BitBoard {
        self.get_hv_pins() | self.anti_diagonal
    }

    pub fn get_ape_anti_diagonal(&self) -> BitBoard {
        self.get_hv_pins() | self.diagonal
    }

    pub fn get_ape_horizontal(&self) -> BitBoard {
        self.get_diag_pins() | self.vertical
    }

    pub fn get_ape_vertical(&self) -> BitBoard {
        self.get_diag_pins() | self.horizontal
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PieceKind {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

impl FromStr for PieceKind {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            Err("Input must contain a single character")
        } else {
            let mut chars = s.chars();

            match chars.next().unwrap() {
                'k' => Ok(PieceKind::King),
                'q' => Ok(PieceKind::Queen),
                'r' => Ok(PieceKind::Rook),
                'b' => Ok(PieceKind::Bishop),
                'n' => Ok(PieceKind::Knight),
                'p' => Ok(PieceKind::Pawn),
                _ => Err("Input must contain a valid piece character (k, q, r, b, n or p)"),
            }
        }
    }
}

impl PieceKind {
    fn into_piece_char(&self) -> char {
        match self {
            PieceKind::King => 'k',
            PieceKind::Queen => 'q',
            PieceKind::Rook => 'r',
            PieceKind::Bishop => 'b',
            PieceKind::Knight => 'n',
            PieceKind::Pawn => 'p',
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Player {
    White,
    Black,
}

impl Not for Player {
    type Output = Player;

    fn not(self) -> Self::Output {
        match self {
            Player::White => Player::Black,
            Player::Black => Player::White,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Piece {
    pub piece_kind: PieceKind,
    pub player: Player,
}

impl Piece {
    pub const WHITE_PAWN: Self = Self {
        piece_kind: PieceKind::Pawn,
        player: Player::White,
    };
    pub const WHITE_KNIGHT: Self = Self {
        piece_kind: PieceKind::Knight,
        player: Player::White,
    };
    pub const WHITE_BISHOP: Self = Self {
        piece_kind: PieceKind::Bishop,
        player: Player::White,
    };
    pub const WHITE_ROOK: Self = Self {
        piece_kind: PieceKind::Rook,
        player: Player::White,
    };
    pub const WHITE_QUEEN: Self = Self {
        piece_kind: PieceKind::Queen,
        player: Player::White,
    };
    pub const WHITE_KING: Self = Self {
        piece_kind: PieceKind::King,
        player: Player::White,
    };

    pub const BLACK_PAWN: Self = Self {
        piece_kind: PieceKind::Pawn,
        player: Player::Black,
    };
    pub const BLACK_KNIGHT: Self = Self {
        piece_kind: PieceKind::Knight,
        player: Player::Black,
    };
    pub const BLACK_BISHOP: Self = Self {
        piece_kind: PieceKind::Bishop,
        player: Player::Black,
    };
    pub const BLACK_ROOK: Self = Self {
        piece_kind: PieceKind::Rook,
        player: Player::Black,
    };
    pub const BLACK_QUEEN: Self = Self {
        piece_kind: PieceKind::Queen,
        player: Player::Black,
    };
    pub const BLACK_KING: Self = Self {
        piece_kind: PieceKind::King,
        player: Player::Black,
    };
}

pub fn divide(board: Board, depth: u32) -> u32 {
    let moves = MoveGen::run(board);

    moves
        .into_iter()
        .map(|chess_move| {
            let mut board_copy = board;

            board_copy.make_move(chess_move);
            let found = search_inner(board_copy, depth - 1);

            println!("{}: {}", chess_move, found);

            found
        })
        .sum()
}

fn search_inner(board: Board, depth: u32) -> u32 {
    let moves = MoveGen::run(board);

    if depth == 0 {
        1
    } else if depth == 1 {
        // At a depth of one we know all next moves will reach depth zero. Thus, we can know they are all leaves and add one each to the nodes searched.
        moves.len() as u32
    } else if moves.len() == 0 {
        0
    } else {
        moves
            .into_iter()
            .map(|chess_move| {
                let mut board_copy = board;

                board_copy.make_move(chess_move);

                search_inner(board_copy, depth - 1)
            })
            .sum()
    }
}
