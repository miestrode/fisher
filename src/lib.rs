#![feature(const_trait_impl, const_for, const_mut_refs)]

use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, Not, Shl, Shr, Sub};

use game::board::Board;
use generators::{MoveGen, Position};
use serde::{Deserialize, Serialize};
pub mod engine;
pub mod game;
pub mod generators;
pub mod tables;

pub const DE_BRUIJN_INDICES: [u32; 64] = [
    0, 47, 1, 56, 48, 27, 2, 60, 57, 49, 41, 37, 28, 16, 3, 61, 54, 58, 35, 52, 50, 42, 21, 44, 38,
    32, 29, 23, 17, 11, 4, 62, 46, 55, 26, 59, 40, 36, 15, 53, 34, 51, 20, 43, 31, 22, 10, 45, 25,
    39, 14, 33, 19, 30, 9, 24, 13, 18, 8, 12, 7, 6, 5, 63,
];

pub const NOT_H_FILE: BitBoard =
    BitBoard(0b0111111101111111011111110111111101111111011111110111111101111111);
pub const NOT_A_FILE: BitBoard =
    BitBoard(0b1111111011111110111111101111111011111110111111101111111011111110);

pub const SECOND_RANK: BitBoard =
    BitBoard(0b0000000000000000000000000000000000000000000000001111111100000000);
pub const SEVENTH_RANK: BitBoard =
    BitBoard(0b0000000011111111000000000000000000000000000000000000000000000000);

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct BitBoard(pub u64);

impl From<&str> for BitBoard {
    fn from(bitstring: &str) -> Self {
        Self(
            u64::from_str_radix(
                bitstring
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join("")
                    .as_str(),
                2,
            )
            .unwrap(),
        )
        .h_flip()
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

    pub fn get_bit(&self, position: Position) -> bool {
        ((self.0 >> position.0) & 1) == 1
    }

    pub fn toggle_bit(&mut self, position: Position) {
        self.0 = self.0 ^ (1 << position.0);
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

    pub fn make_move(&mut self, to: Position, from: Position) {
        self.toggle_bit(from); // Assumed to be on.
        self.toggle_bit(to); // Assumed to be off.
    }

    pub fn is_not_empty(&self) -> bool {
        self.0 != 0
    }

    pub fn is_single_1(&self) -> bool {
        self.0.is_power_of_two()
    }

    pub fn is_full(&self) -> bool {
        self.0 == u64::MAX
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn v_flip(self) -> Self {
        Self(self.0.swap_bytes())
    }

    // See: https://www.chessprogramming.org/Flipping_Mirroring_and_Rotating#Horizontal
    pub fn h_flip(self) -> Self {
        let mut bits = self.0;

        let k_1 = 0x5555555555555555;
        let k_2 = 0x3333333333333333;
        let k_4 = 0x0f0f0f0f0f0f0f0f;

        bits = ((bits >> 1) & k_1) + 2 * (bits & k_1);
        bits = ((bits >> 2) & k_2) + 4 * (bits & k_2);
        bits = ((bits >> 4) & k_4) + 16 * (bits & k_4);

        Self(bits)
    }

    // See: https://www.chessprogramming.org/BitScan#De_Bruijn_Multiplication
    pub fn pop_first_one(&mut self) -> Position {
        assert_ne!(self.0, 0);

        let de_bruijn_number = 0x03f79d71b4cb0a89;

        let position = Position(
            DE_BRUIJN_INDICES
                [(((self.0 ^ (self.0 - 1)).wrapping_mul(de_bruijn_number)) >> 58) as usize],
        );

        // This is done to set the first one to a 0, since we are popping it.
        // See: https://www.chessprogramming.org/General_Setwise_Operations#Reset
        self.0 &= self.0 - 1;

        position
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

    pub fn pfo_with_bitboard(&mut self) -> (Position, Self) {
        assert_ne!(self.0, 0);

        // See: https://www.chessprogramming.org/General_Setwise_Operations#Isolation
        let isolated_one = self.0 & self.0.wrapping_neg(); // Compute the two's complement.

        // This is done to set the first one to a 0, since we are popping it.
        // See: https://www.chessprogramming.org/General_Setwise_Operations#Reset
        self.0 &= self.0 - 1;

        (
            Position(isolated_one.trailing_zeros()),
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

pub mod piece_boards {
    use super::BitBoard;

    pub const WHITE_KING: BitBoard =
        BitBoard(0b0000000000000000000000000000000000000000000000000000000000010000);
    pub const WHITE_QUEENS: BitBoard =
        BitBoard(0b0000000000000000000000000000000000000000000000000000000000001000);
    pub const WHITE_ROOKS: BitBoard =
        BitBoard(0b0000000000000000000000000000000000000000000000000000000010000001);
    pub const WHITE_BISHOPS: BitBoard =
        BitBoard(0b0000000000000000000000000000000000000000000000000000000000100100);
    pub const WHITE_KNIGHTS: BitBoard =
        BitBoard(0b0000000000000000000000000000000000000000000000000000000001000010);
    pub const WHITE_PAWNS: BitBoard =
        BitBoard(0b0000000000000000000000000000000000000000000000001111111100000000);

    // The black king and queen's position bit-board isn't "symmetric" to the white ones.
    pub const BLACK_KING: BitBoard =
        BitBoard(0b0001000000000000000000000000000000000000000000000000000000000000);
    pub const BLACK_QUEENS: BitBoard =
        BitBoard(0b0000100000000000000000000000000000000000000000000000000000000000);
    pub const BLACK_ROOKS: BitBoard = BitBoard(WHITE_ROOKS.0.reverse_bits());
    pub const BLACK_BISHOPS: BitBoard = BitBoard(WHITE_BISHOPS.0.reverse_bits());
    pub const BLACK_KNIGHTS: BitBoard = BitBoard(WHITE_KNIGHTS.0.reverse_bits());
    pub const BLACK_PAWNS: BitBoard = BitBoard(WHITE_PAWNS.0.reverse_bits());
}

#[derive(Serialize, Deserialize, Clone, Copy)]
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
        if (piece & self.horizontal).is_not_empty() {
            self.horizontal
        } else if (piece & self.vertical).is_not_empty() {
            self.vertical
        } else if (piece & self.diagonal).is_not_empty() {
            self.diagonal
        } else if (piece & self.anti_diagonal).is_not_empty() {
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

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum PieceKind {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum Player {
    White,
    Black,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Piece {
    pub piece_kind: PieceKind,
    pub player: Player,
}

fn search(depth: u32) -> u32 {
    search_inner(Board::new(), depth)
}

fn search_inner(board: Board, depth: u32) -> u32 {
    if depth == 0 {
        1
    } else {
        let moves = MoveGen::run(board);

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
