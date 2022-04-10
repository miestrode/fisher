use crate::game::board::PieceKind;

use super::{move_tables::KNIGHT_MOVES, Move, PieceAttackGen};

impl PieceAttackGen<'_, '_> {
    pub fn gen_knight_moves(&mut self) {
        while self.pieces.is_not_empty() {
            let origin = self.pieces.pop_first_one();

            // A knight cannot be pinned, and move.
            self.attacks |= self.check_mask
                & self.friendly_pieces
                & !self.pins.get_all_pins()
                & KNIGHT_MOVES[origin.0 as usize];
        }
    }
}
