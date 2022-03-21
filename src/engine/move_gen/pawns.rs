use crate::game::board::{
    piece_boards::{BLACK_PAWNS, WHITE_PAWNS},
    BitBoard, PieceKind, PiecePins,
};

use super::{GenMoves, Move, Position};

pub struct WhitePawnMoveGen<'a> {
    pub enemy_pieces: BitBoard,
    pub empty_squares: BitBoard,
    pub pawns: BitBoard,
    pub check_mask: BitBoard,
    pub pins: PiecePins,
    pub moves: &'a mut Vec<Move>,
}

impl<'a> WhitePawnMoveGen<'a> {
    fn gen_left_attack_board(&self) -> BitBoard {
        self.check_mask
            & self.enemy_pieces
            & (self.pawns & !(self.pins.diagonal | self.pins.get_hv_pins())).move_up_left(1)
    }

    fn gen_right_attack_board(&self) -> BitBoard {
        self.check_mask
            & self.enemy_pieces
            & (self.pawns & !(self.pins.anti_diagonal | self.pins.get_hv_pins())).move_up_right(1)
    }

    fn gen_push_board(&self) -> BitBoard {
        self.check_mask
            & self.empty_squares
            & (self.pawns & !(self.pins.get_diag_pins() | self.pins.horizontal)).move_up(1)
    }

    fn gen_double_push_board(&self) -> BitBoard {
        self.check_mask
            & self.empty_squares
            & (WHITE_PAWNS & !(self.pins.get_diag_pins() | self.pins.horizontal)).move_up(2)
    }

    fn gen_left_attacks(&mut self) {
        let mut attacks = self.gen_left_attack_board();

        // While there are still some attacks on the bit board.
        while !attacks.is_empty() {
            let to = attacks.pop_first_one();
            let from = Position(to.0 - 7);

            self.moves.push(Move {
                from,
                to,
                piece_kind: PieceKind::Pawn,
            });
        }
    }

    fn gen_right_attacks(&mut self) {
        let mut attacks = self.gen_right_attack_board();

        // While there are still some attacks on the bit board.
        while !attacks.is_empty() {
            let to = attacks.pop_first_one();
            let from = Position(to.0 - 9);

            self.moves.push(Move {
                from,
                to,
                piece_kind: PieceKind::Pawn,
            });
        }
    }

    fn gen_pushes(&mut self) {
        let mut pushes = self.gen_push_board();

        // While there are still some attacks on the bit board.
        while !pushes.is_empty() {
            let to = pushes.pop_first_one();

            let from = Position(to.0 - 8);
            self.moves.push(Move {
                from,
                to,
                piece_kind: PieceKind::Pawn,
            })
        }
    }

    fn gen_double_pushes(&mut self) {
        let mut pushes = self.gen_double_push_board();

        // While there are still some attacks on the bit board.
        while !pushes.is_empty() {
            let to = pushes.pop_first_one();

            let from = Position(to.0 - 16);

            self.moves.push(Move {
                from,
                to,
                piece_kind: PieceKind::Pawn,
            })
        }
    }
}

impl<'a> GenMoves for WhitePawnMoveGen<'a> {
    fn gen_moves(mut self) {
        self.gen_left_attacks();
        self.gen_right_attacks();
        self.gen_pushes();
        self.gen_double_pushes();
    }
}

pub struct BlackPawnMoveGen<'a> {
    pub enemy_pieces: BitBoard,
    pub empty_squares: BitBoard,
    pub pawns: BitBoard,
    pub check_mask: BitBoard,
    pub pins: PiecePins,
    pub moves: &'a mut Vec<Move>,
}

impl<'a> BlackPawnMoveGen<'a> {
    fn gen_left_attack_board(&self) -> BitBoard {
        self.check_mask
            & self.enemy_pieces
            & (self.pawns & !(self.pins.anti_diagonal | self.pins.get_hv_pins())).move_down_left(1)
    }

    fn gen_right_attack_board(&self) -> BitBoard {
        self.check_mask
            & self.enemy_pieces
            & (self.pawns & !(self.pins.diagonal | self.pins.get_hv_pins())).move_down_right(1)
    }

    fn gen_push_board(&self) -> BitBoard {
        self.check_mask
            & self.empty_squares
            & (self.pawns & !(self.pins.get_diag_pins() | self.pins.horizontal)).move_down(1)
    }

    fn gen_double_push_board(&self) -> BitBoard {
        self.check_mask
            & self.empty_squares
            & (BLACK_PAWNS & !(self.pins.get_diag_pins() | self.pins.horizontal)).move_down(2)
    }

    fn gen_left_attacks(&mut self) {
        let mut attacks = self.gen_left_attack_board();

        // While there are still some attacks on the bit board.
        while !attacks.is_empty() {
            let to = attacks.pop_first_one();
            let from = Position(to.0 + 9);

            self.moves.push(Move {
                from,
                to,
                piece_kind: PieceKind::Pawn,
            });
        }
    }

    fn gen_right_attacks(&mut self) {
        let mut attacks = self.gen_right_attack_board();

        // While there are still some attacks on the bit board.
        while !attacks.is_empty() {
            let to = attacks.pop_first_one();
            let from = Position(to.0 + 7);

            self.moves.push(Move {
                from,
                to,
                piece_kind: PieceKind::Pawn,
            });
        }
    }

    fn gen_pushes(&mut self) {
        let mut pushes = self.gen_push_board();

        // While there are still some attacks on the bit board.
        while !pushes.is_empty() {
            let to = pushes.pop_first_one();

            let from = Position(to.0 + 8);
            self.moves.push(Move {
                from,
                to,
                piece_kind: PieceKind::Pawn,
            })
        }
    }

    fn gen_double_pushes(&mut self) {
        let mut pushes = self.gen_double_push_board();

        // While there are still some attacks on the bit board.
        while pushes.is_empty() {
            let to = pushes.pop_first_one();

            let from = Position(to.0 + 16);

            self.moves.push(Move {
                from,
                to,
                piece_kind: PieceKind::Pawn,
            })
        }
    }
}

impl<'a> GenMoves for BlackPawnMoveGen<'a> {
    fn gen_moves(mut self) {
        self.gen_left_attacks();
        self.gen_right_attacks();
        self.gen_pushes();
        self.gen_double_pushes();
    }
}
