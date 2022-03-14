use crate::game::board::BitBoard;

use super::{GenMoves, Move};

struct PsuedoRookMoveGen {
    enemies: BitBoard,
    friendly_pieces: BitBoard,
    rooks: BitBoard,
    moves: Vec<Move>,
}

impl GenMoves for PsuedoRookMoveGen {
    fn gen_moves(self) -> Vec<Move> {
        todo!()
    }
}
