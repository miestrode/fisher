use crate::{
    tables::{KING_MOVES, KNIGHT_MOVES},
    PieceKind, NOT_A_FILE, NOT_H_FILE, SECOND_RANK, SEVENTH_RANK,
};

use super::{slides, Move, MoveGen};

impl MoveGen {
    pub fn gen_black_pawn_attacks(&mut self) {
        let mut left_attacks = self.active.check_mask
            & self.inactive.occupied
            & !(self.active.pins.get_hv_pins() | self.active.pins.anti_diagonal)
            & NOT_A_FILE
            & self.active.pawns.move_down_left();

        let mut right_attacks = self.active.check_mask
            & self.inactive.occupied
            & !(self.active.pins.get_hv_pins() | self.active.pins.diagonal)
            & NOT_H_FILE
            & self.active.pawns.move_down_right();

        while left_attacks.is_not_empty() {
            let target = left_attacks.pop_first_one();

            self.moves.push(Move {
                origin: target.move_up_right(1),
                target,
                piece_kind: PieceKind::Pawn,
            });
        }

        while right_attacks.is_not_empty() {
            let target = right_attacks.pop_first_one();

            self.moves.push(Move {
                origin: target.move_up_left(1),
                target,
                piece_kind: PieceKind::Pawn,
            });
        }
    }

    pub fn gen_black_pawn_pushes(&mut self) {
        let unpinned_locations = !(self.active.pins.get_diag_pins() | self.active.pins.horizontal);

        let mut pushes = self.active.check_mask
            & self.empty_squares
            & (self.active.pawns & unpinned_locations).move_down(1);

        let mut double_pushes = self.active.check_mask
            & self.empty_squares.smear_zeroes_down() // Both squares of movement must be vacant for a double push.
            & (self.active.pawns
                & SEVENTH_RANK
                & unpinned_locations)
                .move_down(2);

        while pushes.is_not_empty() {
            let target = pushes.pop_first_one();

            self.moves.push(Move {
                origin: target.move_up(1),
                target,
                piece_kind: PieceKind::Pawn,
            })
        }

        while double_pushes.is_not_empty() {
            let target = double_pushes.pop_first_one();

            self.moves.push(Move {
                origin: target.move_up(2),
                target,
                piece_kind: PieceKind::Pawn,
            })
        }
    }

    pub fn gen_white_pawn_attacks(&mut self) {
        let mut left_attacks = self.active.check_mask
            & self.inactive.occupied
            & !(self.active.pins.get_hv_pins() | self.active.pins.diagonal)
            & NOT_A_FILE
            & self.active.pawns.move_up_left();

        let mut right_attacks = self.active.check_mask
            & self.inactive.occupied
            & !(self.active.pins.get_hv_pins() | self.active.pins.anti_diagonal)
            & NOT_H_FILE
            & self.active.pawns.move_up_right();

        while left_attacks.is_not_empty() {
            let target = left_attacks.pop_first_one();

            self.moves.push(Move {
                origin: target.move_down_right(1),
                target,
                piece_kind: PieceKind::Pawn,
            });
        }

        while right_attacks.is_not_empty() {
            let target = right_attacks.pop_first_one();

            self.moves.push(Move {
                origin: target.move_down_left(1),
                target,
                piece_kind: PieceKind::Pawn,
            });
        }
    }

    pub fn gen_white_pawn_pushes(&mut self) {
        let unpinned_locations = !(self.active.pins.get_diag_pins() | self.active.pins.horizontal);

        let mut pushes = self.active.check_mask
            & self.empty_squares
            & (self.active.pawns & unpinned_locations).move_up(1);

        let mut double_pushes = self.active.check_mask
            & self.empty_squares.smear_zeroes_up()
            & !(self.active.pins.get_diag_pins() | self.active.pins.horizontal)
            & (self.active.pawns & SECOND_RANK & unpinned_locations).move_up(2);

        while pushes.is_not_empty() {
            let target = pushes.pop_first_one();

            self.moves.push(Move {
                origin: target.move_down(1),
                target,
                piece_kind: PieceKind::Pawn,
            })
        }

        while double_pushes.is_not_empty() {
            let target = double_pushes.pop_first_one();

            self.moves.push(Move {
                origin: target.move_down(2),
                target,
                piece_kind: PieceKind::Pawn,
            })
        }
    }

    pub fn gen_bishop_moves(&mut self) {
        while self.active.bishops.is_not_empty() {
            let (origin, bishop) = self.active.bishops.pfo_with_bitboard();

            if (bishop & self.active.pins.get_hv_pins()).is_not_empty() {
                continue;
            }

            let mut moves = (slides::get_up_right_attacks(bishop, self.empty_squares)
                | slides::get_up_left_attacks(bishop, self.empty_squares)
                | slides::get_down_left_attacks(bishop, self.empty_squares)
                | slides::get_down_right_attacks(bishop, self.empty_squares))
                & self.active.check_mask
                & !self.active.occupied
                & self.active.pins.get_pin_mask(bishop);

            while moves.is_not_empty() {
                let target = moves.pop_first_one();

                self.moves.push(Move {
                    origin,
                    target,
                    piece_kind: PieceKind::Bishop,
                });
            }
        }
    }

    pub fn gen_king_moves(&mut self) {
        let origin = self.active.king.pop_first_one(); // There's only one king.
        let mut moves =
            !self.active.occupied & !self.inactive.attacks & KING_MOVES[origin.0 as usize];

        while moves.is_not_empty() {
            let target = moves.pop_first_one();

            self.moves.push(Move {
                origin,
                target,
                piece_kind: PieceKind::King,
            });
        }
    }

    pub fn gen_knight_moves(&mut self) {
        // A knight cannot be pinned and move.
        let mut knights = self.active.knights & !self.active.pins.get_all_pins();

        while knights.is_not_empty() {
            let origin = knights.pop_first_one();

            let mut moves =
                self.active.check_mask & !self.active.occupied & KNIGHT_MOVES[origin.0 as usize];

            while moves.is_not_empty() {
                let target = moves.pop_first_one();

                self.moves.push(Move {
                    origin,
                    target,
                    piece_kind: PieceKind::Knight,
                });
            }
        }
    }

    pub fn gen_queen_moves(&mut self) {
        while self.active.bishops.is_not_empty() {
            let (origin, queen) = self.active.bishops.pfo_with_bitboard();

            let mut moves = (slides::get_right_attacks(queen, self.empty_squares)
                | slides::get_up_attacks(queen, self.empty_squares)
                | slides::get_left_attacks(queen, self.empty_squares)
                | slides::get_down_attacks(queen, self.empty_squares)
                | slides::get_up_right_attacks(queen, self.empty_squares)
                | slides::get_up_left_attacks(queen, self.empty_squares)
                | slides::get_down_left_attacks(queen, self.empty_squares)
                | slides::get_down_right_attacks(queen, self.empty_squares))
                & !self.active.occupied
                & self.active.check_mask
                & self.active.pins.get_pin_mask(queen);

            while moves.is_not_empty() {
                let target = moves.pop_first_one();

                self.moves.push(Move {
                    origin,
                    target,
                    piece_kind: PieceKind::Queen,
                });
            }
        }
    }

    pub fn gen_rook_moves(&mut self) {
        while self.active.rooks.is_not_empty() {
            let (origin, rook) = self.active.rooks.pfo_with_bitboard();

            if (rook & self.active.pins.get_diag_pins()).is_not_empty() {
                continue;
            }

            let mut moves = (slides::get_right_attacks(rook, self.empty_squares)
                | slides::get_up_attacks(rook, self.empty_squares)
                | slides::get_left_attacks(rook, self.empty_squares)
                | slides::get_down_attacks(rook, self.empty_squares))
                & !self.active.occupied
                & self.active.check_mask
                & self.active.pins.get_pin_mask(rook);

            while moves.is_not_empty() {
                let target = moves.pop_first_one();

                self.moves.push(Move {
                    origin,
                    target,
                    piece_kind: PieceKind::Rook,
                });
            }
        }
    }
}
