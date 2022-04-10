use crate::{
    engine::move_gen::{
        slides::{get_down_attacks, get_left_attacks, get_right_attacks, get_up_attacks},
        Move, PieceMoveGen,
    },
    game::board::PieceKind,
};

impl PieceMoveGen<'_, '_> {
    pub fn gen_rook_moves(&mut self) {
        while self.pieces.is_not_empty() {
            let (origin, rook) = self.pieces.pfo_with_bitboard();

            if (rook & self.pins.get_diag_pins()).is_not_empty() {
                continue;
            }

            let mut moves = (get_right_attacks(rook, self.empty_squares)
                | get_up_attacks(rook, self.empty_squares)
                | get_left_attacks(rook, self.empty_squares)
                | get_down_attacks(rook, self.empty_squares))
                & !self.friendly_pieces
                & self.check_mask
                & self.pins.get_pin_mask(rook);

            while moves.is_not_empty() {
                let target = moves.pop_first_one();

                self.moves.push(Move {
                    origin,
                    target,
                    piece_kind: PieceKind::Rook,
                });
            }
        }
    }
}
