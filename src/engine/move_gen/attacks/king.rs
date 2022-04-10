use crate::engine::move_gen::{move_tables::KING_MOVES, PieceAttackGen};

impl PieceAttackGen<'_> {
    pub fn gen_king_moves(&mut self) {
        let (origin, king) = self.pieces.pfo_with_bitboard(); // Theres only one king.

        let moves = self.update_ecm(king, !self.enemy_attacks & KING_MOVES[origin.0 as usize]);

        *self.attacks |= moves;
    }
}
