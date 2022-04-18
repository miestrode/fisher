use std::mem;

use crate::{
    generators::{AttackGen, Move, Position},
    piece_boards::{
        BLACK_BISHOPS, BLACK_KING, BLACK_KNIGHTS, BLACK_PAWNS, BLACK_QUEENS, BLACK_ROOKS,
        WHITE_BISHOPS, WHITE_KING, WHITE_KNIGHTS, WHITE_PAWNS, WHITE_QUEENS, WHITE_ROOKS,
    },
    BitBoard, Piece, PieceKind, Pins, Player,
};

#[derive(Clone, Copy)]
pub struct PlayerState {
    pub king: BitBoard,
    pub queens: BitBoard,
    pub rooks: BitBoard,
    pub bishops: BitBoard,
    pub knights: BitBoard,
    pub pawns: BitBoard,
    pub occupied: BitBoard,
    pub attacks: BitBoard,
    pub king_must_move: bool,
    pub pins: Pins,
    pub check_mask: BitBoard,
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
                attacks: BitBoard::empty(), // White plays first, and so this will be updated.
                check_mask: BitBoard::full(),
                pins: Pins::new(),
                king_must_move: false,
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
                attacks: BitBoard::empty(), // At the first turn, black's attack data is useless, as any first move cannot cause check.
                pins: Pins::new(),
                king_must_move: false,
                check_mask: BitBoard::full(),
            },
        }
    }

    pub fn get_bitboard(&mut self, piece_kind: PieceKind) -> &mut BitBoard {
        match piece_kind {
            PieceKind::King => &mut self.king,
            PieceKind::Queen => &mut self.queens,
            PieceKind::Rook => &mut self.rooks,
            PieceKind::Bishop => &mut self.bishops,
            PieceKind::Knight => &mut self.knights,
            PieceKind::Pawn => &mut self.pawns,
        }
    }

    pub fn make_move(&mut self, chess_move: Move) {
        self.get_bitboard(chess_move.piece_kind)
            .make_move(chess_move.target, chess_move.origin);

        // All the bitboards must be synchronized with this one.
        self.occupied
            .make_move(chess_move.target, chess_move.origin);
    }

    pub fn kill_piece(&mut self, position: Position, piece_kind: PieceKind) {
        self.get_bitboard(piece_kind).toggle_bit(position);

        // All the bitboards must be synchronized with this one.
        self.occupied.toggle_bit(position);
    }
}

#[derive(Clone, Copy)]
pub struct Board {
    pub active: PlayerState,
    pub inactive: PlayerState,
    pub player_to_play: Player,
    pub board_state: [Option<Piece>; 64],
}

impl Board {
    pub fn new() -> Self {
        let mut board = Self {
            active: PlayerState::new(Player::White),
            inactive: PlayerState::new(Player::Black),
            player_to_play: Player::White,
            board_state: [None; 64],
        };

        for square in 0..64 {
            board.board_state[square as usize] = board.get_piece_slow(Position(square));
        }

        board
    }

    pub fn switch_players(&mut self) {
        mem::swap(&mut self.active, &mut self.inactive)
    }

    pub fn make_move(&mut self, chess_move: Move) {
        self.active.make_move(chess_move);

        // If the move was a capture, we should also update the enemy player's state.
        if let Some(Piece { piece_kind, .. }) = self.get_piece(chess_move.target) {
            self.inactive.kill_piece(chess_move.target, piece_kind);
        }

        // When we make a move, we must update the board state. It is not generated from scratch each time! It is updated, incrementally.
        self.board_state
            .swap(chess_move.origin.0 as usize, chess_move.target.0 as usize);
        self.board_state[chess_move.origin.0 as usize] = None; // The swap could have been with another piece. In case the move is a capture.

        // Before we switch, we need to generate the active player's attacks and the pins + check mask of the inactive player.
        // They are all needed so the inactive player won't make an illegal move next turn.
        AttackGen::run(self);
        self.update_pins_and_cm();

        self.switch_players();
    }

    fn update_pins_and_cm(&mut self) {}

    fn get_piece_slow(&self, position: Position) -> Option<Piece> {
        if self.active.king.get_bit(position) {
            Some(Piece {
                piece_kind: PieceKind::King,
                player: Player::White,
            })
        } else if self.active.queens.get_bit(position) {
            Some(Piece {
                piece_kind: PieceKind::Queen,
                player: Player::White,
            })
        } else if self.active.rooks.get_bit(position) {
            Some(Piece {
                piece_kind: PieceKind::Rook,
                player: Player::White,
            })
        } else if self.active.bishops.get_bit(position) {
            Some(Piece {
                piece_kind: PieceKind::Bishop,
                player: Player::White,
            })
        } else if self.active.knights.get_bit(position) {
            Some(Piece {
                piece_kind: PieceKind::Knight,
                player: Player::White,
            })
        } else if self.active.pawns.get_bit(position) {
            Some(Piece {
                piece_kind: PieceKind::Pawn,
                player: Player::White,
            })
        } else if self.inactive.king.get_bit(position) {
            Some(Piece {
                piece_kind: PieceKind::King,
                player: Player::Black,
            })
        } else if self.inactive.queens.get_bit(position) {
            Some(Piece {
                piece_kind: PieceKind::Queen,
                player: Player::Black,
            })
        } else if self.inactive.rooks.get_bit(position) {
            Some(Piece {
                piece_kind: PieceKind::Rook,
                player: Player::Black,
            })
        } else if self.inactive.bishops.get_bit(position) {
            Some(Piece {
                piece_kind: PieceKind::Bishop,
                player: Player::Black,
            })
        } else if self.inactive.knights.get_bit(position) {
            Some(Piece {
                piece_kind: PieceKind::Knight,
                player: Player::Black,
            })
        } else if self.inactive.pawns.get_bit(position) {
            Some(Piece {
                piece_kind: PieceKind::Pawn,
                player: Player::Black,
            })
        } else {
            None
        }
    }

    pub fn get_piece(&self, position: Position) -> Option<Piece> {
        self.board_state[position.0 as usize]
    }
}
