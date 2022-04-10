use crate::engine::move_gen::{
    slides::{
        get_down_attacks, get_down_left_attacks, get_down_right_attacks, get_right_attacks,
        get_up_attacks, get_up_left_attacks, get_up_right_attacks,
    },
    PieceAttackGen,
};

impl PieceAttackGen<'_> {
    pub fn gen_queen_moves(&mut self) {
        while self.pieces.is_not_empty() {
            let queen = self.pieces.pfo_as_bitboard();

            let attacks = self.update_ecm_for_sliding(get_right_attacks(queen, self.empty_squares))
                | self.update_ecm_for_sliding(get_up_attacks(queen, self.empty_squares))
                | self.update_ecm_for_sliding(get_up_left_attacks(queen, self.empty_squares))
                | self.update_ecm_for_sliding(get_down_attacks(queen, self.empty_squares))
                | self.update_ecm_for_sliding(get_up_right_attacks(queen, self.empty_squares))
                | self.update_ecm_for_sliding(get_up_left_attacks(queen, self.empty_squares))
                | self.update_ecm_for_sliding(get_down_left_attacks(queen, self.empty_squares))
                | self.update_ecm_for_sliding(get_down_right_attacks(queen, self.empty_squares))
                    & self.check_mask
                    & self.pins.get_pin_mask(queen);

            *self.attacks |= attacks;
        }
    }
}
