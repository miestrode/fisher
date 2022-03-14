use std::{
    fmt::{Display, Write},
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not, Shl, Shr},
};

use bitvec::{order::Lsb0, view::BitView};

use crate::engine::{
    move_gen::Position,
    utility::{DE_BRUIJN_INDICES, NOT_A_FILE, NOT_H_FILE},
};

use self::piece_boards::{
    BLACK_BISHOPS, BLACK_KING, BLACK_KNIGHTS, BLACK_PAWNS, BLACK_QUEENS, BLACK_ROOKS,
    WHITE_BISHOPS, WHITE_KING, WHITE_KNIGHTS, WHITE_PAWNS, WHITE_QUEENS, WHITE_ROOKS,
};

#[derive(Clone, Copy)]
pub struct BitBoard(pub u64);

impl BitBoard {
    pub fn new_from_bit_string(string: &str) -> Self {
        Self(
            u64::from_str_radix(
                string
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join("")
                    .as_str(),
                2,
            )
            .unwrap(),
        )
    }

    pub fn empty() -> Self {
        Self(0)
    }

    pub fn get_bit(&self, position: Position) -> bool {
        self.0.view_bits::<Lsb0>()[{ position.0 } as usize]
    }

    // See: https://www.chessprogramming.org/BitScan#De_Bruijn_Multiplication
    pub fn pop_first_one(&mut self) -> Position {
        assert_ne!(self.0, 0);

        let de_bruijn_number = 0x03f79d71b4cb0a89;

        let position = Position::new(
            DE_BRUIJN_INDICES[(((self.0 ^ (self.0 - 1)) * de_bruijn_number) >> 58) as usize],
        );

        // This is done to set the first one to a 0, since we are popping it.
        // See: https://www.chessprogramming.org/General_Setwise_Operations#Reset
        self.0 &= self.0 - 1;

        position
    }

    pub fn isolate_first_one(&mut self) -> Self {
        assert_ne!(self.0, 0);

        let isolated_one = *self & self.not();

        // This is done to set the first one to a 0, since we are popping it.
        // See: https://www.chessprogramming.org/General_Setwise_Operations#Reset
        self.0 &= self.0 - 1;

        isolated_one
    }

    pub fn move_right(self) -> Self {
        BitBoard((self.0 << 1) & NOT_A_FILE.0)
    }

    pub fn move_left(self) -> Self {
        BitBoard((self.0 >> 1) & NOT_H_FILE.0)
    }

    pub fn move_up(self) -> Self {
        BitBoard(self.0 << 8)
    }

    pub fn move_down(self) -> Self {
        BitBoard(self.0 >> 8)
    }
}

// These are utility implementations for conciseness so not every used operation is implemented.
impl BitOr for BitBoard {
    type Output = BitBoard;

    fn bitor(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 | rhs.0)
    }
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for BitBoard {
    type Output = BitBoard;

    fn bitand(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 & rhs.0)
    }
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl Not for BitBoard {
    type Output = BitBoard;

    fn not(self) -> Self::Output {
        BitBoard(!self.0)
    }
}

impl Shr<u32> for BitBoard {
    type Output = BitBoard;

    fn shr(self, rhs: u32) -> Self::Output {
        BitBoard(self.0 >> rhs)
    }
}

impl Shl<u32> for BitBoard {
    type Output = BitBoard;

    fn shl(self, rhs: u32) -> Self::Output {
        BitBoard(self.0 << rhs)
    }
}

mod piece_boards {
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

#[derive(Clone, Copy)]
pub enum PieceKind {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Clone, Copy)]
pub struct Piece {
    pub piece_kind: PieceKind,
    pub player: Player,
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self.player {
            Player::White => match self.piece_kind {
                PieceKind::King => '♔',
                PieceKind::Queen => '♕',
                PieceKind::Rook => '♖',
                PieceKind::Bishop => '♗',
                PieceKind::Knight => '♘',
                PieceKind::Pawn => '♙',
            },
            Player::Black => match self.piece_kind {
                PieceKind::King => '♚',
                PieceKind::Queen => '♛',
                PieceKind::Rook => '♜',
                PieceKind::Bishop => '♝',
                PieceKind::Knight => '♞',
                PieceKind::Pawn => '♟',
            },
        })
    }
}

#[derive(Clone, Copy)]
pub enum Player {
    White,
    Black,
}

#[derive(Clone, Copy)]
pub struct PlayerState {
    pub king: BitBoard,
    pub queens: BitBoard,
    pub rooks: BitBoard,
    pub bishops: BitBoard,
    pub knights: BitBoard,
    // These two bit-boards must ALWAYS be synchronized.
    pub pawns: BitBoard,
    pub unmoved_pawns: BitBoard,
}

impl PlayerState {
    pub fn new(player: Player) -> Self {
        match player {
            Player::White => Self {
                king: WHITE_KING,
                queens: WHITE_QUEENS,
                rooks: WHITE_ROOKS,
                bishops: WHITE_BISHOPS,
                knights: WHITE_KNIGHTS,
                pawns: WHITE_PAWNS,
                unmoved_pawns: WHITE_PAWNS,
            },
            Player::Black => Self {
                king: BLACK_KING,
                queens: BLACK_QUEENS,
                rooks: BLACK_ROOKS,
                bishops: BLACK_BISHOPS,
                knights: BLACK_KNIGHTS,
                pawns: BLACK_PAWNS,
                unmoved_pawns: BLACK_PAWNS,
            },
        }
    }
}

#[derive(Clone, Copy)]
pub struct Board {
    pub white_state: PlayerState,
    pub black_state: PlayerState,
    pub player_to_play: Player,
}

impl Board {
    pub fn new() -> Self {
        Self {
            white_state: PlayerState::new(Player::White),
            black_state: PlayerState::new(Player::Black),
            player_to_play: Player::White,
        }
    }

    pub fn get_piece(&self, position: Position) -> Option<Piece> {
        if self.white_state.king.get_bit(position) {
            Some(Piece {
                piece_kind: PieceKind::King,
                player: Player::White,
            })
        } else if self.white_state.queens.get_bit(position) {
            Some(Piece {
                piece_kind: PieceKind::Queen,
                player: Player::Black,
            })
        } else if self.white_state.rooks.get_bit(position) {
            Some(Piece {
                piece_kind: PieceKind::Rook,
                player: Player::White,
            })
        } else if self.white_state.bishops.get_bit(position) {
            Some(Piece {
                piece_kind: PieceKind::Bishop,
                player: Player::White,
            })
        } else if self.white_state.knights.get_bit(position) {
            Some(Piece {
                piece_kind: PieceKind::Knight,
                player: Player::White,
            })
        } else if self.white_state.pawns.get_bit(position) {
            Some(Piece {
                piece_kind: PieceKind::Pawn,
                player: Player::White,
            })
        } else if self.black_state.king.get_bit(position) {
            Some(Piece {
                piece_kind: PieceKind::King,
                player: Player::Black,
            })
        } else if self.black_state.queens.get_bit(position) {
            Some(Piece {
                piece_kind: PieceKind::Queen,
                player: Player::Black,
            })
        } else if self.black_state.rooks.get_bit(position) {
            Some(Piece {
                piece_kind: PieceKind::Rook,
                player: Player::Black,
            })
        } else if self.black_state.bishops.get_bit(position) {
            Some(Piece {
                piece_kind: PieceKind::Bishop,
                player: Player::Black,
            })
        } else if self.black_state.knights.get_bit(position) {
            Some(Piece {
                piece_kind: PieceKind::Knight,
                player: Player::Black,
            })
        } else if self.black_state.pawns.get_bit(position) {
            Some(Piece {
                piece_kind: PieceKind::Pawn,
                player: Player::Black,
            })
        } else {
            None
        }
    }
}
