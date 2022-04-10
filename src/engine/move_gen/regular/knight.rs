use crate::{
    engine::move_gen::{move_tables::KNIGHT_MOVES, Move, PieceMoveGen},
    game::board::PieceKind,
};

impl PieceMoveGen<'_, '_> {
    pub fn gen_knight_moves(&mut self) {
        while self.pieces.is_not_empty() {
            let origin = self.pieces.pop_first_one();

            // A knight cannot be pinned, and move.
            let mut moves = self.check_mask
                & !self.friendly_pieces
                & !self.pins.get_all_pins()
                & KNIGHT_MOVES[origin.0 as usize];

            while moves.is_not_empty() {
                let target = moves.pop_first_one();

                self.moves.push(Move {
                    origin,
                    target,
                    piece_kind: PieceKind::Knight,
                });
            }
        }
    }
}
