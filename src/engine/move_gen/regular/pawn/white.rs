use crate::{
    engine::{
        move_gen::{Move, PieceMoveGen},
        utility::{NOT_A_FILE, NOT_H_FILE, SECOND_RANK},
    },
    game::board::PieceKind,
};

impl PieceMoveGen<'_, '_> {
    pub fn gen_white_pawn_attacks(&mut self) {
        let mut left_attacks = self.check_mask
            & self.enemy_pieces
            & !(self.pins.get_hv_pins() | self.pins.diagonal)
            & NOT_A_FILE
            & self.pieces.move_up_left(1);

        let mut right_attacks = self.check_mask
            & self.enemy_pieces
            & !(self.pins.get_hv_pins() | self.pins.anti_diagonal)
            & NOT_H_FILE
            & self.pieces.move_up_right(1);

        self.attacks |= left_attacks | right_attacks;

        while left_attacks.is_not_empty() {
            let target = left_attacks.pop_first_one();

            self.moves.push(Move {
                origin: target.move_down_right(1),
                target,
                piece_kind: PieceKind::Pawn,
            });
        }

        while right_attacks.is_not_empty() {
            let target = right_attacks.pop_first_one();

            self.moves.push(Move {
                origin: target.move_down_left(1),
                target,
                piece_kind: PieceKind::Pawn,
            });
        }
    }

    pub fn gen_white_pawn_pushes(&mut self) {
        let mut pushes = self.pieces.move_up(1);
        let mut double_pushes = (self.pieces & SECOND_RANK).move_up(2);

        while pushes.is_not_empty() {
            let target = pushes.pop_first_one();

            self.moves.push(Move {
                origin: target.move_down(1),
                target,
                piece_kind: PieceKind::Pawn,
            })
        }

        while double_pushes.is_not_empty() {
            let target = double_pushes.pop_first_one();

            self.moves.push(Move {
                origin: target.move_down(2),
                target,
                piece_kind: PieceKind::Pawn,
            })
        }
    }
}
