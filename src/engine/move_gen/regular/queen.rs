use crate::{
    engine::move_gen::{
        slides::{
            get_down_attacks, get_down_left_attacks, get_down_right_attacks, get_left_attacks,
            get_right_attacks, get_up_attacks, get_up_left_attacks, get_up_right_attacks,
        },
        Move, PieceMoveGen,
    },
    game::board::PieceKind,
};

impl PieceMoveGen<'_, '_> {
    pub fn gen_queen_moves(&mut self) {
        while self.pieces.is_not_empty() {
            let (origin, queen) = self.pieces.pfo_with_bitboard();

            let mut moves = (get_right_attacks(queen, self.empty_squares)
                | get_up_attacks(queen, self.empty_squares)
                | get_left_attacks(queen, self.empty_squares)
                | get_down_attacks(queen, self.empty_squares)
                | get_up_right_attacks(queen, self.empty_squares)
                | get_up_left_attacks(queen, self.empty_squares)
                | get_down_left_attacks(queen, self.empty_squares)
                | get_down_right_attacks(queen, self.empty_squares))
                & !self.friendly_pieces
                & self.check_mask
                & self.pins.get_pin_mask(queen);

            while moves.is_not_empty() {
                let target = moves.pop_first_one();

                self.moves.push(Move {
                    origin,
                    target,
                    piece_kind: PieceKind::Queen,
                });
            }
        }
    }
}
