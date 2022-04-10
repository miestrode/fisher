use crate::game::board::PieceKind;

use super::{
    slides::{
        get_down_left_attacks, get_down_right_attacks, get_up_left_attacks, get_up_right_attacks,
    },
    Move, PieceAttackGen, Position,
};

impl PieceAttackGen<'_, '_> {
    pub fn gen_bishop_moves(&mut self) {
        while self.pieces.is_not_empty() {
            let bishop = self.pieces.isolate_first_one();

            if (bishop & self.pins.get_hv_pins()).is_not_empty() {
                continue;
            }

            self.attacks |= self.update_enemy_check_mask(get_up_right_attacks(
                bishop,
                self.empty_squares,
                self.friendly_pieces,
            )) | self.update_enemy_check_mask(get_up_left_attacks(
                bishop,
                self.empty_squares,
                self.friendly_pieces,
            )) | self.update_enemy_check_mask(get_down_left_attacks(
                bishop,
                self.empty_squares,
                self.friendly_pieces,
            )) | self.update_enemy_check_mask(get_down_right_attacks(
                bishop,
                self.empty_squares,
                self.friendly_pieces,
            )) & self.check_mask
                & self.pins.get_pin_mask(bishop);
        }
    }
}
