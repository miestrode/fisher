use crate::game::board::PieceKind;

use super::{
    slides::{
        get_down_left_attacks, get_down_right_attacks, get_up_left_attacks, get_up_right_attacks,
    },
    Move, PieceMoveGen, Position,
};

impl PieceMoveGen<'_, '_> {
    pub fn gen_bishop_moves(&mut self) {
        while self.pieces.is_not_empty() {
            let bishop = self.pieces.isolate_first_one();

            if (bishop & self.pins.get_hv_pins()).is_not_empty() {
                continue;
            }

            let mut moves = get_up_right_attacks(bishop, self.empty_squares, self.friendly_pieces)
                | get_up_left_attacks(bishop, self.empty_squares, self.friendly_pieces)
                | get_down_left_attacks(bishop, self.empty_squares, self.friendly_pieces)
                | get_down_right_attacks(bishop, self.empty_squares, self.friendly_pieces)
                    & self.check_mask
                    & self.pins.get_pin_mask(bishop);

            let origin = Position(bishop.0.trailing_zeros() as u8);

            while moves.is_not_empty() {
                let target = moves.pop_first_one();

                self.moves.push(Move {
                    origin,
                    target,
                    piece_kind: PieceKind::Bishop,
                });
            }
        }
    }
}
