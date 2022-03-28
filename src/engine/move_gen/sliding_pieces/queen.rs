use crate::{
    engine::move_gen::{GenMoves, Move},
    game::board::{BitBoard, PieceKind, PiecePins},
};

pub struct QueenMoveGen<'a> {
    pub empty_squares: BitBoard,
    pub friendly_pieces: BitBoard,
    pub queens: BitBoard,
    pub pins: PiecePins,
    pub check_mask: BitBoard,
    pub moves: &'a mut Vec<Move>,
}

impl<'a> GenMoves for QueenMoveGen<'a> {
    fn gen_moves(mut self) {
        while !self.queens.is_empty() {
            let queen = self.queens.isolate_first_one();

            let mut attacks = self.check_mask
                & super::get_cross_slides(queen, self.empty_squares, self.friendly_pieces)
                | super::get_diagonal_slides(queen, self.empty_squares, self.friendly_pieces)
                    & self.pins.get_pin_mask(queen);

            let queen_position = self.queens.pop_first_one();

            while !attacks.is_empty() {
                self.moves.push(Move {
                    from: queen_position,
                    to: attacks.pop_first_one(),
                    piece_kind: PieceKind::Queen,
                })
            }
        }
    }
}
