use std::ops::BitAnd;

use crate::{
    engine::utility::{NOT_A_FILE, NOT_H_FILE},
    game::board::{BitBoard, Player},
};

use super::{GenMoves, Move, Position};

struct PsuedoPawnMoveGen {
    enemy_pieces: BitBoard,
    occupied_squares: BitBoard,
    pawns: BitBoard,
    unmoved_pawns: BitBoard,
    player: Player,
    moves: Vec<Move>,
}

impl PsuedoPawnMoveGen {
    fn gen_left_attack_board(&self) -> BitBoard {
        self.enemy_pieces
            & match self.player {
                Player::White => self.pawns.move_up().move_left(),
                Player::Black => self.pawns.move_down().move_left(),
            }
    }

    fn gen_right_attack_board(&self) -> BitBoard {
        self.enemy_pieces
            & match self.player {
                Player::White => self.pawns.move_up().move_right(),
                Player::Black => self.pawns.move_down().move_right(),
            }
    }

    fn gen_push_board(&self) -> BitBoard {
        !self.occupied_squares
            & match self.player {
                Player::White => self.pawns.move_up(),
                Player::Black => self.pawns.move_down(),
            }
    }

    fn gen_double_push_board(&self) -> BitBoard {
        !self.occupied_squares
            & match self.player {
                Player::White => self.pawns.move_up().move_up(),
                Player::Black => self.pawns.move_down().move_up(),
            }
    }

    fn gen_left_attacks(&mut self) {
        let mut attacks = self.gen_left_attack_board();

        // While there are still some attacks on the bit board.
        while attacks.0 != 0 {
            let to = attacks.pop_first_one();

            // TODO: Move this check to the start of the function. It should only be done once.
            // This is currently not done for ergonomics.
            let from = Position::new(match self.player {
                Player::White => ({ to.0 }) - 7,
                Player::Black => ({ to.0 }) + 9,
            });

            self.moves.push(Move { from, to })
        }
    }

    fn gen_right_attacks(&mut self) {
        let mut attacks = self.gen_left_attack_board();

        // While there are still some attacks on the bit board.
        while attacks.0 != 0 {
            let to = attacks.pop_first_one();

            // TODO: Refactor this in accordance to what has been written above (In the previous check like this).
            let from = Position::new(match self.player {
                Player::White => ({ to.0 }) - 9,
                Player::Black => ({ to.0 }) + 7,
            });

            self.moves.push(Move { from, to })
        }
    }

    fn gen_pushes(&mut self) {
        let mut pushes = self.gen_push_board();

        // While there are still some attacks on the bit board.
        while pushes.0 != 0 {
            let to = pushes.pop_first_one();

            // TODO: Refactor this in accordance to what has been written above (In the previous check like this).
            let from = Position::new(match self.player {
                Player::White => ({ to.0 }) - 8,
                Player::Black => ({ to.0 }) + 8,
            });

            self.moves.push(Move { from, to })
        }
    }

    fn gen_double_pushes(&mut self) {
        let mut pushes = self.gen_double_push_board();

        // While there are still some attacks on the bit board.
        while pushes.0 != 0 {
            let to = pushes.pop_first_one();

            // TODO: Refactor this in accordance to what has been written above (In the previous check like this).
            let from = Position::new(match self.player {
                Player::White => ({ to.0 }) - 16,
                Player::Black => ({ to.0 }) + 16,
            });

            self.moves.push(Move { from, to })
        }
    }
}

impl GenMoves for PsuedoPawnMoveGen {
    fn gen_moves(mut self) -> Vec<Move> {
        self.gen_left_attacks();
        self.gen_right_attacks();
        self.gen_pushes();
        self.gen_double_pushes();

        self.moves
    }
}
