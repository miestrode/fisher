use crate::game::board::PieceKind;

use super::{
    slides::{
        get_down_attacks, get_down_left_attacks, get_down_right_attacks, get_left_attacks,
        get_right_attacks, get_up_attacks, get_up_left_attacks, get_up_right_attacks,
    },
    Move, PieceMoveGen, Position,
};

impl PieceMoveGen<'_, '_> {
    pub fn gen_queen_moves(&mut self) {
        while self.pieces.is_not_empty() {
            let queen = self.pieces.isolate_first_one();

            let mut moves = get_right_attacks(queen, self.empty_squares, self.friendly_pieces)
                | get_up_attacks(queen, self.empty_squares, self.friendly_pieces)
                | get_left_attacks(queen, self.empty_squares, self.friendly_pieces)
                | get_down_attacks(queen, self.empty_squares, self.friendly_pieces)
                | get_up_right_attacks(queen, self.empty_squares, self.friendly_pieces)
                | get_up_left_attacks(queen, self.empty_squares, self.friendly_pieces)
                | get_down_left_attacks(queen, self.empty_squares, self.friendly_pieces)
                | get_down_right_attacks(queen, self.empty_squares, self.friendly_pieces)
                    & self.check_mask
                    & self.pins.get_pin_mask(queen);

            let origin = Position(queen.0.trailing_zeros() as u8);

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
