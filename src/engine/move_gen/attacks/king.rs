use crate::game::board::PieceKind;

use super::{move_tables::KING_MOVES, Move, PieceAttackGen};

impl PieceAttackGen<'_, '_> {
    pub fn gen_king_moves(&mut self) {
        let origin = self.pieces.pop_first_one(); // Theres only one king.
        self.attacks |= !self.friendly_pieces & !self.enemy_attacks & KING_MOVES[origin.0 as usize];
    }
}
