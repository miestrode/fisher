use crate::{
    tables::{KING_MOVES, KNIGHT_MOVES},
    PieceKind, CASTLE_KS_SPACE, CASTLE_QS_SPACE, EIGHTH_RANK, FIRST_RANK, SECOND_RANK,
    SEVENTH_RANK,
};

use super::{slides, Move, MoveGen};

impl MoveGen {
    // NOTICE: Make sure these functions and the white pawn functions are synced!
    pub fn gen_black_pawn_en_passants(&mut self) {
        if (self.ep_capture_point & self.active.check_mask).isnt_empty() {
            let unpinned_left_pawns = self.active.pawns & !self.active.pins.get_ape_diagonal();
            let unpinned_right_pawns =
                self.active.pawns & !self.active.pins.get_ape_anti_diagonal();

            let left_possible_pawn = self.ep_capture_point.move_up_right() & unpinned_left_pawns;
            let right_possible_pawn = self.ep_capture_point.move_up_left() & unpinned_right_pawns;

            if left_possible_pawn.isnt_empty() {
                self.moves.push(Move::EnPassant {
                    origin: left_possible_pawn.first_one_square(),
                })
            }

            if right_possible_pawn.isnt_empty() {
                self.moves.push(Move::EnPassant {
                    origin: right_possible_pawn.first_one_square(),
                })
            }
        }
    }

    pub fn gen_black_pawn_attacks(&mut self) {
        let unpinned_left_pawns = !self.active.pins.get_ape_diagonal();
        let unpinned_right_pawns = !self.active.pins.get_ape_anti_diagonal();

        let mut left_attacks = self.active.check_mask
            & self.inactive.pieces
            & (self.active.pawns & unpinned_left_pawns).move_down_left();

        let mut right_attacks = self.active.check_mask
            & self.inactive.pieces
            & (self.active.pawns & unpinned_right_pawns).move_down_right();

        while left_attacks.isnt_empty() {
            let target = left_attacks.pop_first_one();
            let origin = target.move_up_right(1);

            if target.0 < 8 {
                self.add_promotions(origin, target);
            } else {
                self.moves.push(Move::Regular {
                    origin,
                    target,
                    piece_kind: PieceKind::Pawn,
                    double_push: false,
                });
            }
        }

        while right_attacks.isnt_empty() {
            let target = right_attacks.pop_first_one();
            let origin = target.move_up_left(1);

            if target.0 < 8 {
                self.add_promotions(origin, target);
            } else {
                self.moves.push(Move::Regular {
                    origin,
                    target,
                    piece_kind: PieceKind::Pawn,
                    double_push: false,
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

        while promotions.isnt_empty() {
            let target = promotions.pop_first_one();
            self.add_promotions(target.move_up(1), target)
        }

        while pushes.isnt_empty() {
            let target = pushes.pop_first_one();

            self.moves.push(Move::Regular {
                origin: target.move_up(1),
                target,
                piece_kind: PieceKind::Pawn,
                double_push: false,
            });
        }

        while double_pushes.isnt_empty() {
            let target = double_pushes.pop_first_one();

            self.moves.push(Move::Regular {
                origin: target.move_up(2),
                target,
                piece_kind: PieceKind::Pawn,
                double_push: true,
            });
        }
    }

    pub fn gen_white_pawn_en_passants(&mut self) {
        if (self.ep_capture_point & self.active.check_mask).isnt_empty() {
            let unpinned_left_pawns = self.active.pawns & !self.active.pins.get_ape_anti_diagonal();
            let unpinned_right_pawns = self.active.pawns & !self.active.pins.get_ape_diagonal();

            let left_possible_pawn = self.ep_capture_point.move_down_right() & unpinned_left_pawns;
            let right_possible_pawn = self.ep_capture_point.move_down_left() & unpinned_right_pawns;

            if left_possible_pawn.isnt_empty() {
                self.moves.push(Move::EnPassant {
                    origin: left_possible_pawn.first_one_square(),
                })
            }

            if right_possible_pawn.isnt_empty() {
                self.moves.push(Move::EnPassant {
                    origin: right_possible_pawn.first_one_square(),
                })
            }
        }
    }

    pub fn gen_white_pawn_attacks(&mut self) {
        let legal_left_pawns = !self.active.pins.get_ape_anti_diagonal();
        let legal_right_pawns = !self.active.pins.get_ape_diagonal();

        let mut left_attacks = self.active.check_mask
            & self.inactive.pieces
            & (self.active.pawns & legal_left_pawns).move_up_left();

        let mut right_attacks = self.active.check_mask
            & self.inactive.pieces
            & (self.active.pawns & legal_right_pawns).move_up_right();

        while left_attacks.isnt_empty() {
            let target = left_attacks.pop_first_one();
            let origin = target.move_down_right(1);

            if 55 < target.0 {
                self.add_promotions(origin, target);
            } else {
                self.moves.push(Move::Regular {
                    origin,
                    target,
                    piece_kind: PieceKind::Pawn,
                    double_push: false,
                });
            }
        }

        while right_attacks.isnt_empty() {
            let target = right_attacks.pop_first_one();
            let origin = target.move_down_left(1);

            if 55 < target.0 {
                self.add_promotions(origin, target);
            } else {
                self.moves.push(Move::Regular {
                    origin,
                    target,
                    piece_kind: PieceKind::Pawn,
                    double_push: false,
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

        while promotions.isnt_empty() {
            let target = promotions.pop_first_one();

            self.add_promotions(target.move_down(1), target);
        }

        while pushes.isnt_empty() {
            let target = pushes.pop_first_one();

            self.moves.push(Move::Regular {
                origin: target.move_down(1),
                target,
                piece_kind: PieceKind::Pawn,
                double_push: false,
            })
        }

        while double_pushes.isnt_empty() {
            let target = double_pushes.pop_first_one();

            self.moves.push(Move::Regular {
                origin: target.move_down(2),
                target,
                piece_kind: PieceKind::Pawn,
                double_push: true,
            })
        }
    }

    pub fn gen_bishop_moves(&mut self) {
        let mut bishops = self.active.bishops & !self.active.pins.get_hv_pins();

        while bishops.isnt_empty() {
            let (origin, bishop) = bishops.pfo_with_bitboard();

            let mut moves = (slides::get_up_right_attacks(bishop, self.empty_squares)
                | slides::get_up_left_attacks(bishop, self.empty_squares)
                | slides::get_down_left_attacks(bishop, self.empty_squares)
                | slides::get_down_right_attacks(bishop, self.empty_squares))
                & self.active.check_mask
                & !self.active.pieces
                & self.active.pins.get_pin_mask(bishop);

            while moves.isnt_empty() {
                let target = moves.pop_first_one();

                self.moves.push(Move::Regular {
                    origin,
                    target,
                    piece_kind: PieceKind::Bishop,
                    double_push: false,
                });
            }
        }
    }

    pub fn gen_king_moves(&mut self) {
        let origin = self.active.king.pop_first_one(); // There's only one king.
        let mut moves = !self.active.pieces & !self.inactive.attacks & KING_MOVES[origin];

        while moves.isnt_empty() {
            let target = moves.pop_first_one();

            self.moves.push(Move::Regular {
                origin,
                target,
                piece_kind: PieceKind::King,
                double_push: false,
            });
        }
    }

    pub fn gen_knight_moves(&mut self) {
        // A knight cannot be pinned and move.
        let mut knights = self.active.knights & !self.active.pins.get_all_pins();

        while knights.isnt_empty() {
            let origin = knights.pop_first_one();

            let mut moves = self.active.check_mask & !self.active.pieces & KNIGHT_MOVES[origin];

            while moves.isnt_empty() {
                let target = moves.pop_first_one();

                self.moves.push(Move::Regular {
                    origin,
                    target,
                    piece_kind: PieceKind::Knight,
                    double_push: false,
                });
            }
        }
    }

    pub fn gen_queen_moves(&mut self) {
        while self.active.queens.isnt_empty() {
            let (origin, queen) = self.active.queens.pfo_with_bitboard();

            let mut moves = (slides::get_right_attacks(queen, self.empty_squares)
                | slides::get_up_attacks(queen, self.empty_squares)
                | slides::get_left_attacks(queen, self.empty_squares)
                | slides::get_down_attacks(queen, self.empty_squares)
                | slides::get_up_right_attacks(queen, self.empty_squares)
                | slides::get_up_left_attacks(queen, self.empty_squares)
                | slides::get_down_left_attacks(queen, self.empty_squares)
                | slides::get_down_right_attacks(queen, self.empty_squares))
                & !self.active.pieces
                & self.active.check_mask
                & self.active.pins.get_pin_mask(queen);

            while moves.isnt_empty() {
                let target = moves.pop_first_one();

                self.moves.push(Move::Regular {
                    origin,
                    target,
                    piece_kind: PieceKind::Queen,
                    double_push: false,
                });
            }
        }
    }

    pub fn gen_rook_moves(&mut self) {
        let mut rooks = self.active.rooks & !self.active.pins.get_diag_pins();

        while rooks.isnt_empty() {
            let (origin, rook) = rooks.pfo_with_bitboard();

            if (rook & self.active.pins.get_diag_pins()).isnt_empty() {
                continue;
            }

            let mut moves = (slides::get_right_attacks(rook, self.empty_squares)
                | slides::get_up_attacks(rook, self.empty_squares)
                | slides::get_left_attacks(rook, self.empty_squares)
                | slides::get_down_attacks(rook, self.empty_squares))
                & !self.active.pieces
                & self.active.check_mask
                & self.active.pins.get_pin_mask(rook);

            while moves.isnt_empty() {
                let target = moves.pop_first_one();

                self.moves.push(Move::Regular {
                    origin,
                    target,
                    piece_kind: PieceKind::Rook,
                    double_push: false,
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
