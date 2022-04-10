use crate::{
    engine::move_gen::{move_tables::KING_MOVES, Move, PieceMoveGen},
    game::board::PieceKind,
};

impl PieceMoveGen<'_, '_> {
    pub fn gen_king_moves(&mut self) {
        let origin = self.pieces.pop_first_one(); // There's only one king.
        let mut moves = !self.friendly_pieces & !self.enemy_attacks & KING_MOVES[origin.0 as usize];

        while moves.is_not_empty() {
            let target = moves.pop_first_one();

            self.moves.push(Move {
                origin,
                target,
                piece_kind: PieceKind::King,
            });
        }
    }
}
