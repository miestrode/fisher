use crate::{
    tables::{KING_MOVES, KNIGHT_MOVES},
    PieceKind, CASTLE_KS_SPACE, CASTLE_QS_SPACE, EIGHTH_RANK, FIRST_RANK, SECOND_RANK,
    SEVENTH_RANK,
};

use super::{slides, Move, MoveGen};

impl MoveGen {
    // NOTICE: Make sure these functions and the white pawn functions are synced!
    pub fn gen_black_pawn_en_passants(&mut self) {
        let unpinned_left_pawns = !self.active.pins.get_ape_diagonal();
        let unpinned_right_pawns = !self.active.pins.get_ape_anti_diagonal();

        if (self.en_passant
            & (self.active.pawns & unpinned_left_pawns).move_down_left()
            & self.active.check_mask)
            .is_not_empty()
        {
            let target = self.en_passant.first_one_position();

            self.moves.push(Move::Regular {
                target,
                origin: target.move_up_right(1),
                piece_kind: PieceKind::Pawn,
                is_en_passant: true,
            })
        }

        if (self.en_passant
            & (self.active.pawns & unpinned_right_pawns).move_down_right()
            & self.active.check_mask)
            .is_not_empty()
        {
            let target = self.en_passant.first_one_position();

            self.moves.push(Move::Regular {
                target,
                origin: target.move_up_left(1),
                piece_kind: PieceKind::Pawn,
                is_en_passant: true,
            })
        }
    }

    pub fn gen_black_pawn_attacks(&mut self) {
        let unpinned_left_pawns = !self.active.pins.get_ape_diagonal();
        let unpinned_right_pawns = !self.active.pins.get_ape_anti_diagonal();

        let mut left_attacks = self.active.check_mask
            & self.inactive.occupied
            & (self.active.pawns & unpinned_left_pawns).move_down_left();

        let mut right_attacks = self.active.check_mask
            & self.inactive.occupied
            & (self.active.pawns & unpinned_right_pawns).move_down_right();

        while left_attacks.is_not_empty() {
            let target = left_attacks.pop_first_one();
            let origin = target.move_up_right(1);

            if target.0 < 8 {
                self.add_promotions(origin, target);
            } else {
                self.moves.push(Move::Regular {
                    origin,
                    target,
                    piece_kind: PieceKind::Pawn,
                    is_en_passant: false,
                });
            }
        }

        while right_attacks.is_not_empty() {
            let target = right_attacks.pop_first_one();
            let origin = target.move_up_left(1);

            if target.0 < 8 {
                self.add_promotions(origin, target);
            } else {
                self.moves.push(Move::Regular {
                    origin,
                    target,
                    piece_kind: PieceKind::Pawn,
                    is_en_passant: false,
                });
            }
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

        let mut promotions = pushes & FIRST_RANK;
        pushes &= !FIRST_RANK;

        while promotions.is_not_empty() {
            let target = promotions.pop_first_one();
            self.add_promotions(target.move_up(1), target)
        }

        while pushes.is_not_empty() {
            let target = pushes.pop_first_one();

            self.moves.push(Move::Regular {
                origin: target.move_up(1),
                target,
                piece_kind: PieceKind::Pawn,
                is_en_passant: false,
            });
        }

        while double_pushes.is_not_empty() {
            let target = double_pushes.pop_first_one();

            self.moves.push(Move::Regular {
                origin: target.move_up(2),
                target,
                piece_kind: PieceKind::Pawn,
                is_en_passant: false,
            });
        }
    }

    pub fn gen_white_pawn_en_passants(&mut self) {
        let unpinned_left_pawns = !self.active.pins.get_ape_anti_diagonal();
        let unpinned_right_pawns = !self.active.pins.get_ape_diagonal();

        if (self.en_passant
            & (self.active.pawns & unpinned_left_pawns).move_up_left()
            & self.active.check_mask)
            .is_not_empty()
        {
            let target = self.en_passant.first_one_position();

            self.moves.push(Move::Regular {
                target,
                origin: target.move_down_right(1),
                piece_kind: PieceKind::Pawn,
                is_en_passant: true,
            })
        }

        if (self.en_passant
            & (self.active.pawns & unpinned_right_pawns).move_up_right()
            & self.active.check_mask)
            .is_not_empty()
        {
            let target = self.en_passant.first_one_position();

            self.moves.push(Move::Regular {
                target,
                origin: target.move_down_left(1),
                piece_kind: PieceKind::Pawn,
                is_en_passant: true,
            })
        }
    }

    pub fn gen_white_pawn_attacks(&mut self) {
        let legal_left_pawns = !self.active.pins.get_ape_anti_diagonal();
        let legal_right_pawns = !self.active.pins.get_ape_diagonal();

        let mut left_attacks = self.active.check_mask
            & self.inactive.occupied
            & (self.active.pawns & legal_left_pawns).move_up_left();

        let mut right_attacks = self.active.check_mask
            & self.inactive.occupied
            & (self.active.pawns & legal_right_pawns).move_up_right();

        while left_attacks.is_not_empty() {
            let target = left_attacks.pop_first_one();
            let origin = target.move_down_right(1);

            if 55 < target.0 {
                self.add_promotions(origin, target);
            } else {
                self.moves.push(Move::Regular {
                    origin,
                    target,
                    piece_kind: PieceKind::Pawn,
                    is_en_passant: false,
                });
            }
        }

        while right_attacks.is_not_empty() {
            let target = right_attacks.pop_first_one();
            let origin = target.move_down_left(1);

            if 55 < target.0 {
                self.add_promotions(origin, target);
            } else {
                self.moves.push(Move::Regular {
                    origin,
                    target,
                    piece_kind: PieceKind::Pawn,
                    is_en_passant: false,
                });
            }
        }
    }

    pub fn gen_white_pawn_pushes(&mut self) {
        let unpinned_locations = !(self.active.pins.get_diag_pins() | self.active.pins.horizontal);

        let mut pushes = self.active.check_mask
            & self.empty_squares
            & (self.active.pawns & unpinned_locations).move_up(1);

        let mut double_pushes = self.active.check_mask
            & self.empty_squares.smear_zeroes_up()
            & (self.active.pawns & SECOND_RANK & unpinned_locations).move_up(2);

        let mut promotions = pushes & EIGHTH_RANK;
        pushes &= !EIGHTH_RANK;

        while promotions.is_not_empty() {
            let target = promotions.pop_first_one();

            self.add_promotions(target.move_down(1), target);
        }

        while pushes.is_not_empty() {
            let target = pushes.pop_first_one();

            self.moves.push(Move::Regular {
                origin: target.move_down(1),
                target,
                piece_kind: PieceKind::Pawn,
                is_en_passant: false,
            })
        }

        while double_pushes.is_not_empty() {
            let target = double_pushes.pop_first_one();

            self.moves.push(Move::Regular {
                origin: target.move_down(2),
                target,
                piece_kind: PieceKind::Pawn,
                is_en_passant: false,
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

                self.moves.push(Move::Regular {
                    origin,
                    target,
                    piece_kind: PieceKind::Bishop,
                    is_en_passant: false,
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

            self.moves.push(Move::Regular {
                origin,
                target,
                piece_kind: PieceKind::King,
                is_en_passant: false,
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

                self.moves.push(Move::Regular {
                    origin,
                    target,
                    piece_kind: PieceKind::Knight,
                    is_en_passant: false,
                });
            }
        }
    }

    pub fn gen_queen_moves(&mut self) {
        while self.active.queens.is_not_empty() {
            let (origin, queen) = self.active.queens.pfo_with_bitboard();

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

                self.moves.push(Move::Regular {
                    origin,
                    target,
                    piece_kind: PieceKind::Queen,
                    is_en_passant: false,
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

                self.moves.push(Move::Regular {
                    origin,
                    target,
                    piece_kind: PieceKind::Rook,
                    is_en_passant: false,
                });
            }
        }
    }

    // NOTICE: To castle, the active player mustn't be in check. This check is done in the main "gen_moves" function.
    pub fn castle_king_side(&mut self) {
        if self.active.can_castle_ks && ((self.empty_squares & CASTLE_KS_SPACE) == CASTLE_KS_SPACE)
        {
            self.moves.push(Move::CastleKS);
        }
    }

    pub fn castle_queen_side(&mut self) {
        if self.active.can_castle_qs && ((self.empty_squares & CASTLE_QS_SPACE) == CASTLE_QS_SPACE)
        {
            self.moves.push(Move::CastleQS);
        }
    }
}
