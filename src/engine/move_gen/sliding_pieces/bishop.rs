use crate::{
    engine::move_gen::{GenMoves, Move},
    game::board::{BitBoard, PieceKind, PiecePins},
};

pub struct BishopMoveGen<'a> {
    pub empty_squares: BitBoard,
    pub friendly_pieces: BitBoard,
    pub bishops: BitBoard,
    pub pins: PiecePins,
    pub check_mask: BitBoard,
    pub moves: &'a mut Vec<Move>,
}

impl<'a> GenMoves for BishopMoveGen<'a> {
    fn gen_moves(mut self) {
        while !self.bishops.is_empty() {
            let bishop = self.bishops.isolate_first_one();

            if !(bishop & self.pins.get_hv_pins()).is_empty() {
                continue; // The bishop is pinned on the cross and so it cannot move at all.
            }

            let mut attacks = self.check_mask
                & super::get_diagonal_slides(bishop, self.empty_squares, self.friendly_pieces)
                & self.pins.get_pin_mask(bishop);

            let bishop_position = self.bishops.pop_first_one();

            while !attacks.is_empty() {
                self.moves.push(Move {
                    from: bishop_position,
                    to: attacks.pop_first_one(),
                    piece_kind: PieceKind::Bishop,
                })
            }
        }
    }
}
