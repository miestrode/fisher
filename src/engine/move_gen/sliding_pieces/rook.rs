use crate::game::board::BitBoard;

use super::{sliding::get_cross_attacks, GenMoves, Move};

struct PsuedoRookMoveGen {
    empty_squares: BitBoard,
    friendly_pieces: BitBoard,
    rooks: BitBoard,
    moves: Vec<Move>,
}

impl GenMoves for PsuedoRookMoveGen {
    fn gen_moves(mut self) -> Vec<Move> {
        while self.rooks.0 != 0 {
            let rook = self.rooks.isolate_first_one();
            let mut attacks = get_cross_attacks(rook, self.empty_squares, self.friendly_pieces);

            let rook_position = self.rooks.pop_first_one();

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
