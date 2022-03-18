use crate::{
    engine::move_gen::{GenMoves, Move},
    game::board::BitBoard,
};

use super::get_diagonal_attacks;

struct PsuedoBishopMoveGen {
    empty_squares: BitBoard,
    friendly_pieces: BitBoard,
    bishops: BitBoard,
    moves: Vec<Move>,
}

impl GenMoves for PsuedoBishopMoveGen {
    fn gen_moves(mut self) -> Vec<Move> {
        while self.bishops.0 != 0 {
            let bishop = self.bishops.isolate_first_one();
            let mut attacks =
                get_diagonal_attacks(bishop, self.empty_squares, self.friendly_pieces);

            let rook_position = self.bishops.pop_first_one();

            while attacks.0 != 0 {
                self.moves.push(Move {
                    from: rook_position,
                    to: attacks.pop_first_one(),
                })
            }
        }

        self.moves
    }
}
