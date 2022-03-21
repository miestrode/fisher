use std::{
    fmt::{Display, Write},
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, Not, Shl, Shr, Sub},
};

use bitvec::{order::Lsb0, view::BitView};

use crate::engine::{
    move_gen::{Move, Position},
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
        .h_flip()
    }

    pub const fn empty() -> Self {
        Self(0)
    }

    pub fn get_bit(&self, position: Position) -> bool {
        self.0.view_bits::<Lsb0>()[position.0 as usize]
    }

    pub fn toggle_bit(&mut self, position: Position) {
        self.0 = self.0 ^ !(1 << position.0);
    }

    pub fn make_move(&mut self, to: Position, from: Position) {
        self.toggle_bit(from); // Assumed to be on.
        self.toggle_bit(to); // Assumed to be off.
    }

    pub fn v_flip(self) -> Self {
        Self(self.0.swap_bytes())
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
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
            DE_BRUIJN_INDICES[(((self.0 ^ (self.0 - 1)) * de_bruijn_number) >> 58) as usize],
        );

        // This is done to set the first one to a 0, since we are popping it.
        // See: https://www.chessprogramming.org/General_Setwise_Operations#Reset
        self.0 &= self.0 - 1;

        position
    }

    pub fn isolate_first_one(mut self) -> Self {
        assert_ne!(self.0, 0);

        // See: https://www.chessprogramming.org/General_Setwise_Operations#Isolation
        let isolated_one = self.0 & self.0.wrapping_neg(); // Compute the two's complement.

        // This is done to set the first one to a 0, since we are popping it.
        // See: https://www.chessprogramming.org/General_Setwise_Operations#Reset
        self.0 &= self.0 - 1;

        BitBoard(isolated_one)
    }

    pub const fn move_right(self, amount: u32) -> Self {
        self << amount & NOT_A_FILE
    }

    pub const fn move_left(self, amount: u32) -> Self {
        (self >> amount) & NOT_H_FILE
    }

    pub const fn move_up(self, amount: u32) -> Self {
        self << (8 * amount)
    }

    pub const fn move_down(self, amount: u32) -> Self {
        self >> (8 * amount)
    }

    pub const fn move_up_right(self, amount: u32) -> Self {
        self << (9 * amount)
    }

    pub const fn move_up_left(self, amount: u32) -> Self {
        self << (7 * amount)
    }

    pub const fn move_down_right(self, amount: u32) -> Self {
        self >> (7 * amount)
    }

    pub const fn move_down_left(self, amount: u32) -> Self {
        self >> (9 * amount)
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

#[derive(Clone, Copy)]
pub struct PiecePins {
    pub horizontal: BitBoard,
    pub vertical: BitBoard,
    pub diagonal: BitBoard,
    pub anti_diagonal: BitBoard,
}

impl PiecePins {
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
        if !(piece & self.horizontal).is_empty() {
            self.horizontal
        } else if !(piece & self.vertical).is_empty() {
            self.vertical
        } else if !(piece & self.diagonal).is_empty() {
            self.diagonal
        } else if !(piece & self.anti_diagonal).is_empty() {
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
    pub occupied: BitBoard,
    pub pins: PiecePins,
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
                occupied: WHITE_KING
                    | WHITE_QUEENS
                    | WHITE_ROOKS
                    | WHITE_BISHOPS
                    | WHITE_KNIGHTS
                    | WHITE_PAWNS,
                pins: PiecePins::new(),
            },
            Player::Black => Self {
                king: BLACK_KING,
                queens: BLACK_QUEENS,
                rooks: BLACK_ROOKS,
                bishops: BLACK_BISHOPS,
                knights: BLACK_KNIGHTS,
                pawns: BLACK_PAWNS,
                occupied: BLACK_KING
                    | BLACK_QUEENS
                    | BLACK_ROOKS
                    | BLACK_BISHOPS
                    | BLACK_KNIGHTS
                    | BLACK_PAWNS,
                pins: PiecePins::new(),
            },
        }
    }
}

#[derive(Clone, Copy)]
pub struct Board {
    pub white_state: PlayerState,
    pub black_state: PlayerState,
    pub check_mask: BitBoard,
    pub player_to_play: Player,
}

impl Board {
    pub fn new() -> Self {
        Self {
            white_state: PlayerState::new(Player::White),
            black_state: PlayerState::new(Player::Black),
            check_mask: !BitBoard::empty(),
            player_to_play: Player::White,
        }
    }

    pub fn make_move(&mut self, chess_move: Move) {
        match self.player_to_play {}
    }

    // All moves used are assumed to be legal, as the move generator the engine has doesn't generate any psuedo-legal moves.
    fn make_white_move(&mut self, chess_move: Move) {
        match chess_move.piece_kind.piece_kind {
            PieceKind::Pawn => self
                .white_state
                .pawns
                .make_move(chess_move.to, chess_move.from),
            PieceKind::Queen => self
                .white_state
                .queens
                .make_move(chess_move.to, chess_move.from),
            PieceKind::Rook => self
                .white_state
                .rooks
                .make_move(chess_move.to, chess_move.from),
            PieceKind::Bishop => self
                .white_state
                .bishops
                .make_move(chess_move.to, chess_move.from),
            PieceKind::King => self
                .white_state
                .king
                .make_move(chess_move.to, chess_move.from),
            PieceKind::Knight => self
                .white_state
                .knights
                .make_move(chess_move.to, chess_move.from),
        }

        self.update_white_pins();
    }

    // Same thing here.
    fn make_black_move(&mut self, chess_move: Move) {
        match chess_move.piece_kind.piece_kind {
            PieceKind::Pawn => self
                .black_state
                .pawns
                .make_move(chess_move.to, chess_move.from),
            PieceKind::Queen => self
                .black_state
                .queens
                .make_move(chess_move.to, chess_move.from),
            PieceKind::Rook => self
                .black_state
                .rooks
                .make_move(chess_move.to, chess_move.from),
            PieceKind::Bishop => self
                .black_state
                .bishops
                .make_move(chess_move.to, chess_move.from),
            PieceKind::King => self
                .black_state
                .king
                .make_move(chess_move.to, chess_move.from),
            PieceKind::Knight => self
                .black_state
                .knights
                .make_move(chess_move.to, chess_move.from),
        }

        self.update_black_pins();
    }

    fn update_white_pins(&mut self) {
        todo!()
    }

    fn update_black_pins(&mut self) {
        todo!()
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
