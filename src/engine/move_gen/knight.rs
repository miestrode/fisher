use crate::game::board::BitBoard;

use super::{GenMoves, Move, Position};

struct PsuedoKnightMoveGen {
    friendly_pieces: BitBoard,
    knights: BitBoard,

    moves: Vec<Move>,
}

// TODO: Optimize this section of code.
impl PsuedoKnightMoveGen {
    fn try_gen_move(&mut self, from: Position, to: Position) {
        if !self.friendly_pieces.get_bit(to) {
            self.moves.push(Move { from, to });
        }
    }

    // See: https://www.chessprogramming.org/Knight_Pattern
    fn gen_knight_moves(&mut self, from: Position) {
        self.try_gen_move(from, Position(from.0 + 15));
        self.try_gen_move(from, Position(from.0 + 17));
        self.try_gen_move(from, Position(from.0 + 6));
        self.try_gen_move(from, Position(from.0 + 10));
        self.try_gen_move(from, Position(from.0 - 10));
        self.try_gen_move(from, Position(from.0 - 6));
        self.try_gen_move(from, Position(from.0 - 17));
        self.try_gen_move(from, Position(from.0 - 15));
    }
}

impl GenMoves for PsuedoKnightMoveGen {
    fn gen_moves(mut self) -> Vec<Move> {
        while self.knights.0 != 0 {
            let knight_position = self.knights.pop_first_one();
            self.gen_knight_moves(knight_position);
        }

        self.moves
    }
}
