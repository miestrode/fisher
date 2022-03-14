use crate::game::board::BitBoard;

use super::{GenMoves, Move};

struct PsuedoKingMoveGen {
    friendly_pieces: BitBoard,
    king: BitBoard,
    moves: Vec<Move>,
}

impl GenMoves for PsuedoKingMoveGen {
    fn gen_moves(mut self) -> Vec<Move> {
        let mut attacks = self.king.move_left() | self.king.move_right();
        let horizontal = self.king | attacks;

        let king_pos = self.king.pop_first_one(); // This modifies the "king" field and so must be placed before the attack set generation code.

        attacks = (attacks | horizontal.move_up() | horizontal.move_down()) & !self.friendly_pieces;

        while attacks.0 != 0 {
            self.moves.push(Move {
                from: king_pos,
                to: attacks.pop_first_one(),
            })
        }

        self.moves
    }
}
