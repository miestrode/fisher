use super::super::{
    slides::{get_down_attacks, get_left_attacks, get_right_attacks, get_up_attacks},
    PieceAttackGen,
};

impl PieceAttackGen<'_> {
    pub fn gen_rook_moves(&mut self) {
        while self.pieces.is_not_empty() {
            let rook = self.pieces.pfo_as_bitboard();

            if (rook & self.pins.get_diag_pins()).is_not_empty() {
                continue;
            }

            let attacks = self.update_ecm_for_sliding(get_right_attacks(rook, self.empty_squares))
                | self.update_ecm_for_sliding(get_up_attacks(rook, self.empty_squares))
                | self.update_ecm_for_sliding(get_left_attacks(rook, self.empty_squares))
                | self.update_ecm_for_sliding(get_down_attacks(rook, self.empty_squares))
                    & self.check_mask
                    & self.pins.get_pin_mask(rook);

            *self.attacks |= attacks;
        }
    }
}
