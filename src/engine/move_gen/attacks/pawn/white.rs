use crate::engine::{
    move_gen::PieceAttackGen,
    utility::{NOT_A_FILE, NOT_H_FILE},
};

impl PieceAttackGen<'_> {
    pub fn gen_white_pawn_attacks(&mut self) {
        *self.attacks |= self.check_mask
            & self.enemy_pieces
            & !(self.pins.get_hv_pins() | self.pins.diagonal)
            & NOT_A_FILE
            & self.pieces.move_up_left();

        *self.attacks |= self.check_mask
            & self.enemy_pieces
            & !(self.pins.get_hv_pins() | self.pins.anti_diagonal)
            & NOT_H_FILE
            & self.pieces.move_up_right();
    }
}
