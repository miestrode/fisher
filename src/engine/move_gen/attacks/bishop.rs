use crate::engine::move_gen::{
    slides::{
        get_down_left_attacks, get_down_right_attacks, get_up_left_attacks, get_up_right_attacks,
    },
    PieceAttackGen,
};

impl PieceAttackGen<'_> {
    pub fn gen_bishop_moves(&mut self) {
        while self.pieces.is_not_empty() {
            let bishop = self.pieces.pfo_as_bitboard();

            if (bishop & self.pins.get_hv_pins()).is_not_empty() {
                continue;
            }

            let attacks = self
                .update_ecm_for_sliding(get_up_right_attacks(bishop, self.empty_squares))
                | self.update_ecm_for_sliding(get_up_left_attacks(bishop, self.empty_squares))
                | self.update_ecm_for_sliding(get_down_left_attacks(bishop, self.empty_squares))
                | self.update_ecm_for_sliding(get_down_right_attacks(bishop, self.empty_squares))
                    & self.check_mask
                    & self.pins.get_pin_mask(bishop);

            *self.attacks |= attacks;
        }
    }
}
