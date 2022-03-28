use crate::{
    engine::move_gen::{GenMoves, Move},
    game::board::{BitBoard, PieceKind, PiecePins},
};

pub struct RookMoveGen<'a> {
    pub empty_squares: BitBoard,
    pub friendly_pieces: BitBoard,
    pub rooks: BitBoard,
    pub pins: PiecePins,
    pub check_mask: BitBoard,
    pub moves: &'a mut Vec<Move>,
}

impl<'a> GenMoves for RookMoveGen<'a> {
    fn gen_moves(mut self) {
        while !self.rooks.is_empty() {
            let rook = self.rooks.isolate_first_one();

            if !(rook & self.pins.get_diag_pins()).is_empty() {
                continue; // The rook is pinned diagonally and it cannot move at all.
            }

            let mut attacks = self.check_mask
                & super::get_cross_slides(rook, self.empty_squares, self.friendly_pieces)
                & self.pins.get_pin_mask(rook);

            let rook_position = self.rooks.pop_first_one();

            while !attacks.is_empty() {
                self.moves.push(Move {
                    from: rook_position,
                    to: attacks.pop_first_one(),
                    piece_kind: PieceKind::Rook,
                })
            }
        }
    }
}
