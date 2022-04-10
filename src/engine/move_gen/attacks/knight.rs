use crate::engine::move_gen::{move_tables::KNIGHT_MOVES, PieceAttackGen};

impl PieceAttackGen<'_> {
    pub fn gen_knight_moves(&mut self) {
        while self.pieces.is_not_empty() {
            let (origin, knight) = self.pieces.pfo_with_bitboard();

            let moves = self.update_ecm(
                knight,
                // A knight cannot be pinned, and move.
                self.check_mask & !self.pins.get_all_pins() & KNIGHT_MOVES[origin.0 as usize],
            );

            *self.attacks |= moves;
        }
    }
}
