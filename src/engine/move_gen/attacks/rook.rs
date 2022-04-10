use crate::game::board::PieceKind;

use super::super::{
    slides::{get_down_attacks, get_left_attacks, get_right_attacks, get_up_attacks},
    Move, PieceAttackGen, Position,
};

impl PieceAttackGen<'_> {
    pub fn gen_rook_moves(&mut self) {
        while self.pieces.is_not_empty() {
            let rook = self.pieces.isolate_first_one();

            if (rook & self.pins.get_diag_pins()).is_not_empty() {
                continue;
            }

            self.attacks |= self.update_enemy_check_mask(get_right_attacks(
                rook,
                self.empty_squares,
                self.friendly_pieces,
            )) | self.update_enemy_check_mask(get_up_attacks(
                rook,
                self.empty_squares,
                self.friendly_pieces,
            )) | self.update_enemy_check_mask(get_left_attacks(
                rook,
                self.empty_squares,
                self.friendly_pieces,
            )) | self.update_enemy_check_mask(get_down_attacks(
                rook,
                self.empty_squares,
                self.friendly_pieces,
            )) & self.check_mask
                & self.pins.get_pin_mask(rook);
        }
    }
}
