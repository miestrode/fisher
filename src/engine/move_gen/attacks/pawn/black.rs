use crate::{
    engine::{
        move_gen::{Move, PieceAttackGen},
        utility::{NOT_A_FILE, NOT_H_FILE, SEVENTH_RANK},
    },
    game::board::PieceKind,
};

impl PieceAttackGen<'_, '_> {
    pub fn gen_black_pawn_attacks(&mut self) {
        self.attacks |= self.check_mask
            & self.enemy_pieces
            & !(self.pins.get_hv_pins() | self.pins.anti_diagonal)
            & NOT_A_FILE
            & self.pieces.move_down_left(1);
        self.attacks |= self.check_mask
            & self.enemy_pieces
            & !(self.pins.get_hv_pins() | self.pins.diagonal)
            & NOT_H_FILE
            & self.pieces.move_down_right(1);
    }
}
