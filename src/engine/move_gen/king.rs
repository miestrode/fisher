use crate::game::board::{BitBoard, PieceKind};

use super::{move_tables::KING_MOVES, GenMoves, Move};

struct PsuedoKingMoveGen<'a> {
    friendly_pieces: BitBoard,
    king: BitBoard,
    enemy_attacks: BitBoard,
    moves: &'a mut Vec<Move>,
}

impl<'a> GenMoves for PsuedoKingMoveGen<'a> {
    fn gen_moves(mut self) {
        let king_pos = self.king.pop_first_one();
        let mut moves =
            KING_MOVES[king_pos.0 as usize] & !self.enemy_attacks & !self.friendly_pieces;

        while !moves.is_empty() {
            self.moves.push(Move {
                from: king_pos,
                to: moves.pop_first_one(),
                piece_kind: PieceKind::King,
            })
        }
    }
}
