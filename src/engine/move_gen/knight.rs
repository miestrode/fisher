use crate::game::board::{BitBoard, PieceKind, PiecePins};

use super::{move_tables::KNIGHT_MOVES, GenMoves, Move};

pub struct KnightMoveGen<'a> {
    pub friendly_pieces: BitBoard,
    pub knights: BitBoard,
    pub pins: PiecePins,
    pub check_mask: BitBoard,
    pub moves: &'a mut Vec<Move>,
}

impl<'a> GenMoves for KnightMoveGen<'a> {
    fn gen_moves(self) {
        while !self.knights.is_empty() {
            let mut knight = self.knights.isolate_first_one();

            if !(self.pins.get_all_pins() & knight).is_empty() {
                continue; // Knight is pinned and therefore, cannot move.
            }

            let knight_pos = knight.pop_first_one();
            let mut moves =
                self.check_mask & KNIGHT_MOVES[knight.0 as usize] & !self.friendly_pieces;

            while !moves.is_empty() {
                self.moves.push(Move {
                    from: knight_pos,
                    to: moves.pop_first_one(),
                    piece_kind: PieceKind::Knight,
                })
            }
        }
    }
}
