use crate::{
    tables::{KING_MOVES, KNIGHT_MOVES},
    PieceKind, B_CASTLE_KS_SPACE, B_CASTLE_QS_KING_PASS, B_CASTLE_QS_SPACE, EIGHTH_RANK,
    FIRST_RANK, SECOND_RANK, SEVENTH_RANK, W_CASTLE_KS_SPACE, W_CASTLE_QS_KING_PASS,
    W_CASTLE_QS_SPACE,
};

use super::{slides, Move, MoveGen};

impl MoveGen {
    // NOTICE: Make sure these functions and the white pawn functions are synced!
    pub fn gen_black_pawn_en_passants(&mut self) {
        // There are scenarios where the capture point is not in the check mask but the EP-pawn is (like when an EP-pawn threatens mate).
        if ((self.ep_info.capture_point | self.ep_info.pawn) & self.moving_player.check_mask)
            .isnt_empty()
        {
            let unpinned_left_pawns =
                self.moving_player.pawns - self.moving_player.pins.get_ape_diagonal();
            let unpinned_right_pawns =
                self.moving_player.pawns - self.moving_player.pins.get_ape_anti_diagonal();

            let left_possible_pawn =
                self.ep_info.capture_point.move_up_right() & unpinned_left_pawns;
            let right_possible_pawn =
                self.ep_info.capture_point.move_up_left() & unpinned_right_pawns;

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
        let unpinned_left_pawns = !self.moving_player.pins.get_ape_diagonal();
        let unpinned_right_pawns = !self.moving_player.pins.get_ape_anti_diagonal();

        let mut left_attacks = self.moving_player.check_mask
            & self.moved_player.pieces
            & (self.moving_player.pawns & unpinned_left_pawns).move_down_left();

        let mut right_attacks = self.moving_player.check_mask
            & self.moved_player.pieces
            & (self.moving_player.pawns & unpinned_right_pawns).move_down_right();

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
        let unpinned_locations =
            !(self.moving_player.pins.get_diag_pins() | self.moving_player.pins.horizontal);

        let mut pushes = self.moving_player.check_mask
            & self.empty_squares
            & (self.moving_player.pawns & unpinned_locations).move_down(1);

        let mut double_pushes = self.moving_player.check_mask
            & self.empty_squares.smear_zeroes_down() // Both squares of movement must be vacant for a double push.
            & (self.moving_player.pawns
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
        if ((self.ep_info.capture_point | self.ep_info.pawn) & self.moving_player.check_mask)
            .isnt_empty()
        {
            let unpinned_left_pawns =
                self.moving_player.pawns - self.moving_player.pins.get_ape_anti_diagonal();
            let unpinned_right_pawns =
                self.moving_player.pawns - self.moving_player.pins.get_ape_diagonal();

            let left_possible_pawn =
                self.ep_info.capture_point.move_down_right() & unpinned_left_pawns;
            let right_possible_pawn =
                self.ep_info.capture_point.move_down_left() & unpinned_right_pawns;

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
        let legal_left_pawns = !self.moving_player.pins.get_ape_anti_diagonal();
        let legal_right_pawns = !self.moving_player.pins.get_ape_diagonal();

        let mut left_attacks = self.moving_player.check_mask
            & self.moved_player.pieces
            & (self.moving_player.pawns & legal_left_pawns).move_up_left();

        let mut right_attacks = self.moving_player.check_mask
            & self.moved_player.pieces
            & (self.moving_player.pawns & legal_right_pawns).move_up_right();

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
        let unpinned_locations =
            !(self.moving_player.pins.get_diag_pins() | self.moving_player.pins.horizontal);

        let mut pushes = self.moving_player.check_mask
            & self.empty_squares
            & (self.moving_player.pawns & unpinned_locations).move_up(1);

        let mut double_pushes = self.moving_player.check_mask
            & self.empty_squares.smear_zeroes_up()
            & (self.moving_player.pawns & SECOND_RANK & unpinned_locations).move_up(2);

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
        let mut bishops = self.moving_player.bishops - self.moving_player.pins.get_hv_pins();

        while bishops.isnt_empty() {
            let (origin, bishop) = bishops.pfo_with_bitboard();

            let mut moves = (slides::get_up_right_attacks(bishop, self.empty_squares)
                | slides::get_up_left_attacks(bishop, self.empty_squares)
                | slides::get_down_left_attacks(bishop, self.empty_squares)
                | slides::get_down_right_attacks(bishop, self.empty_squares))
                & self.moving_player.check_mask
                & !self.moving_player.pieces
                & self.moving_player.pins.get_pin_mask(bishop);

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
        let origin = self.moving_player.king.pop_first_one(); // There's only one king.
        let mut moves = KING_MOVES[origin] - self.moving_player.pieces - self.moved_player.attacks;

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
        let mut knights = self.moving_player.knights - self.moving_player.pins.get_all_pins();

        while knights.isnt_empty() {
            let origin = knights.pop_first_one();

            let mut moves =
                self.moving_player.check_mask & !self.moving_player.pieces & KNIGHT_MOVES[origin];

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
        while self.moving_player.queens.isnt_empty() {
            let (origin, queen) = self.moving_player.queens.pfo_with_bitboard();

            let mut moves = (slides::get_right_attacks(queen, self.empty_squares)
                | slides::get_up_attacks(queen, self.empty_squares)
                | slides::get_left_attacks(queen, self.empty_squares)
                | slides::get_down_attacks(queen, self.empty_squares)
                | slides::get_up_right_attacks(queen, self.empty_squares)
                | slides::get_up_left_attacks(queen, self.empty_squares)
                | slides::get_down_left_attacks(queen, self.empty_squares)
                | slides::get_down_right_attacks(queen, self.empty_squares))
                & !self.moving_player.pieces
                & self.moving_player.check_mask
                & self.moving_player.pins.get_pin_mask(queen);

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
        let mut rooks = self.moving_player.rooks - self.moving_player.pins.get_diag_pins();

        while rooks.isnt_empty() {
            let (origin, rook) = rooks.pfo_with_bitboard();

            if (rook & self.moving_player.pins.get_diag_pins()).isnt_empty() {
                continue;
            }

            let mut moves = (slides::get_right_attacks(rook, self.empty_squares)
                | slides::get_up_attacks(rook, self.empty_squares)
                | slides::get_left_attacks(rook, self.empty_squares)
                | slides::get_down_attacks(rook, self.empty_squares))
                & !self.moving_player.pieces
                & self.moving_player.check_mask
                & self.moving_player.pins.get_pin_mask(rook);

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
    // See: https://en.wikipedia.org/wiki/Castling#Requirements
    pub fn white_castle_king_side(&mut self) {
        // The king cannot enter check after the castle.
        if self.moving_player.can_castle_ks
            && W_CASTLE_KS_SPACE.does_contain_none(!self.empty_squares | self.moved_player.attacks)
        {
            self.moves.push(Move::CastleKS);
        }
    }

    pub fn white_castle_queen_side(&mut self) {
        // The king cannot enter check after the castle.
        if self.moving_player.can_castle_qs
            && W_CASTLE_QS_SPACE.does_contain_none(!self.empty_squares)
            && W_CASTLE_QS_KING_PASS.does_contain_none(self.moved_player.attacks)
        {
            self.moves.push(Move::CastleQS);
        }
    }

    pub fn black_castle_king_side(&mut self) {
        // The king cannot enter check after the castle.
        if self.moving_player.can_castle_ks
            && B_CASTLE_KS_SPACE.does_contain_none(!self.empty_squares | self.moved_player.attacks)
        {
            self.moves.push(Move::CastleKS);
        }
    }

    pub fn black_castle_queen_side(&mut self) {
        // The king cannot enter check after the castle.
        if self.moving_player.can_castle_qs
            && B_CASTLE_QS_SPACE.does_contain_none(!self.empty_squares)
            && B_CASTLE_QS_KING_PASS.does_contain_none(self.moved_player.attacks)
        {
            self.moves.push(Move::CastleQS);
        }
    }
}
