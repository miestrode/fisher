use crate::tables::{KING_MOVES, KNIGHT_MOVES};

use super::{slides, AttackGen};

impl AttackGen<'_> {
    // NOTICE: Make sure these functions and the white pawn functions are synced!
    pub fn gen_black_pawn_attacks(&mut self) {
        let unpinned_left_pawns = !self.active.pins.get_ape_diagonal();
        let unpinned_right_pawns = !self.active.pins.get_ape_anti_diagonal();

        *self.attacks |= (self.active.pawns & unpinned_left_pawns).move_down_left();
        *self.attacks |= (self.active.pawns & unpinned_right_pawns).move_down_right();
    }

    pub fn gen_white_pawn_attacks(&mut self) {
        let unpinned_left_pawns = !self.active.pins.get_ape_anti_diagonal();
        let unpinned_right_pawns = !self.active.pins.get_ape_diagonal();

        *self.attacks |= (self.active.pawns & unpinned_left_pawns).move_up_left();
        *self.attacks |= (self.active.pawns & unpinned_right_pawns).move_up_right();
    }

    pub fn gen_bishop_attacks(&mut self) {
        let allow_diagonal_pins = !self.active.pins.get_ape_diagonal();
        let allow_anti_diagonal_pins = !self.active.pins.get_ape_anti_diagonal();

        *self.attacks |= slides::get_up_right_attacks(
            self.active.bishops & allow_diagonal_pins,
            self.empty_squares,
        ) | slides::get_up_left_attacks(
            self.active.bishops & allow_anti_diagonal_pins,
            self.empty_squares,
        ) | slides::get_down_left_attacks(
            self.active.bishops & allow_diagonal_pins,
            self.empty_squares,
        ) | slides::get_down_right_attacks(
            self.active.bishops & allow_anti_diagonal_pins,
            self.empty_squares,
        );
    }

    pub fn gen_king_attacks(&mut self) {
        let origin = self.active.king.pop_first_one(); // Theres only one king.

        *self.attacks |= KING_MOVES[origin.0 as usize];
    }

    pub fn gen_knight_attacks(&mut self) {
        let mut knights = self.active.knights & !self.active.pins.get_all_pins();

        while knights.is_not_empty() {
            let origin = knights.pop_first_one();

            *self.attacks |= KNIGHT_MOVES[origin.0 as usize];
        }
    }

    pub fn gen_queen_attacks(&mut self) {
        let allow_diagonal_pins = !self.active.pins.get_ape_diagonal();
        let allow_anti_diagonal_pins = !self.active.pins.get_ape_anti_diagonal();
        let allow_horizontal_pins = !self.active.pins.get_ape_horizontal();
        let allow_vertical_pins = !self.active.pins.get_ape_vertical();

        *self.attacks |= slides::get_up_right_attacks(
            self.active.queens & allow_diagonal_pins,
            self.empty_squares,
        ) | slides::get_up_left_attacks(
            self.active.queens & allow_anti_diagonal_pins,
            self.empty_squares,
        ) | slides::get_down_left_attacks(
            self.active.queens & allow_diagonal_pins,
            self.empty_squares,
        ) | slides::get_down_right_attacks(
            self.active.queens & allow_anti_diagonal_pins,
            self.empty_squares,
        ) | slides::get_up_attacks(
            self.active.queens & allow_vertical_pins,
            self.empty_squares,
        ) | slides::get_right_attacks(
            self.active.queens & allow_horizontal_pins,
            self.empty_squares,
        ) | slides::get_down_attacks(
            self.active.queens & allow_vertical_pins,
            self.empty_squares,
        ) | slides::get_left_attacks(
            self.active.queens & allow_horizontal_pins,
            self.empty_squares,
        );
    }

    pub fn gen_rook_attacks(&mut self) {
        let allow_horizontal_pins = !self.active.pins.get_ape_horizontal();
        let allow_vertical_pins = !self.active.pins.get_ape_vertical();

        *self.attacks |=
            slides::get_up_attacks(self.active.rooks & allow_vertical_pins, self.empty_squares)
                | slides::get_right_attacks(
                    self.active.rooks & allow_horizontal_pins,
                    self.empty_squares,
                )
                | slides::get_down_attacks(
                    self.active.rooks & allow_vertical_pins,
                    self.empty_squares,
                )
                | slides::get_left_attacks(
                    self.active.rooks & allow_horizontal_pins,
                    self.empty_squares,
                );
    }
}
