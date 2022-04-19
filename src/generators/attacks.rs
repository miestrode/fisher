use crate::{NOT_A_FILE, NOT_H_FILE};

use super::{
    move_tables::{KING_MOVES, KNIGHT_MOVES},
    slides, AttackGen,
};

impl AttackGen<'_> {
    pub fn gen_black_pawn_attacks(&mut self) {
        *self.attacks |= (self.active.check_mask
            & !(self.active.pins.get_hv_pins() | self.active.pins.anti_diagonal)
            & NOT_A_FILE
            & self.active.pawns.move_down_left())
            | (self.active.check_mask
                & self.inactive.occupied
                & !(self.active.pins.get_hv_pins() | self.active.pins.diagonal)
                & NOT_H_FILE
                & self.active.pawns.move_down_right());
    }

    pub fn gen_white_pawn_attacks(&mut self) {
        *self.attacks |= (self.active.check_mask
            & !(self.active.pins.get_hv_pins() | self.active.pins.diagonal)
            & NOT_A_FILE
            & self.active.pawns.move_up_left())
            | (self.active.check_mask
                & !(self.active.pins.get_hv_pins() | self.active.pins.anti_diagonal)
                & NOT_H_FILE
                & self.active.pawns.move_up_right());
    }

    pub fn gen_bishop_attacks(&mut self) {
        let allow_diagonal_pins = !self.active.pins.get_ape_diagonal();
        let allow_anti_diagonal_pins = !self.active.pins.get_ape_anti_diagonal();

        *self.attacks |= self.active.check_mask
            & (slides::get_up_right_attacks(
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
            ));
    }

    pub fn gen_king_attacks(&mut self) {
        let origin = self.active.king.pop_first_one(); // Theres only one king.

        *self.attacks |= !self.inactive.attacks & KING_MOVES[origin.0 as usize];
    }

    pub fn gen_knight_attacks(&mut self) {
        let mut knights = self.active.knights & !self.active.pins.get_all_pins();

        while knights.is_not_empty() {
            let origin = knights.pop_first_one();

            *self.attacks |= self.active.check_mask & KNIGHT_MOVES[origin.0 as usize];
        }
    }

    pub fn gen_queen_attacks(&mut self) {
        let allow_diagonal_pins = !self.active.pins.get_ape_diagonal();
        let allow_anti_diagonal_pins = !self.active.pins.get_ape_anti_diagonal();
        let allow_horizontal_pins = !self.active.pins.get_ape_horizontal();
        let allow_vertical_pins = !self.active.pins.get_ape_vertical();

        *self.attacks |= self.active.check_mask
            & (slides::get_up_right_attacks(
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
            ));
    }

    pub fn gen_rook_attacks(&mut self) {
        let allow_horizontal_pins = !self.active.pins.get_ape_horizontal();
        let allow_vertical_pins = !self.active.pins.get_ape_vertical();

        *self.attacks |= self.active.check_mask
            & (slides::get_up_attacks(self.active.rooks & allow_vertical_pins, self.empty_squares)
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
                ));
    }
}
