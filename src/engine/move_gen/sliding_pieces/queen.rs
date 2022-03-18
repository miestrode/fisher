use crate::{
    engine::move_gen::{GenMoves, Move},
    game::board::BitBoard,
};

use super::{get_cross_attacks, get_diagonal_attacks};

struct PsuedoQueenMoveGen {
    empty_squares: BitBoard,
    friendly_pieces: BitBoard,
    queens: BitBoard,
    moves: Vec<Move>,
}

impl GenMoves for PsuedoQueenMoveGen {
    fn gen_moves(mut self) -> Vec<Move> {
        while self.queens.0 != 0 {
            let queen = self.queens.isolate_first_one();
            let mut attacks = get_cross_attacks(queen, self.empty_squares, self.friendly_pieces)
                | get_diagonal_attacks(queen, self.empty_squares, self.friendly_pieces);

            let rook_position = self.queens.pop_first_one();

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
