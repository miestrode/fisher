use crate::tables::{KING_MOVES, KNIGHT_MOVES};

use super::{slides, AttackGen};

impl AttackGen<'_> {
    // NOTICE: Make sure these functions and the white pawn functions are synced!
    pub fn gen_black_pawn_attacks(&mut self) {
        *self.attacks |= self.attacking_player.pawns.move_down_left();
        *self.attacks |= self.attacking_player.pawns.move_down_right();
    }

    pub fn gen_white_pawn_attacks(&mut self) {
        *self.attacks |= self.attacking_player.pawns.move_up_left();
        *self.attacks |= self.attacking_player.pawns.move_up_right();
    }

    pub fn gen_bishop_attacks(&mut self) {
        *self.attacks |=
            slides::get_up_right_attacks(self.attacking_player.bishops, self.empty_squares)
                | slides::get_up_left_attacks(self.attacking_player.bishops, self.empty_squares)
                | slides::get_down_left_attacks(self.attacking_player.bishops, self.empty_squares)
                | slides::get_down_right_attacks(self.attacking_player.bishops, self.empty_squares);
    }

    pub fn gen_king_attacks(&mut self) {
        let origin = self.attacking_player.king.pop_first_one(); // Theres only one king.

        *self.attacks |= KING_MOVES[origin];
    }

    // TODO: Refactor the while loop away.
    pub fn gen_knight_attacks(&mut self) {
        let mut knights = self.attacking_player.knights;

        while knights.isnt_empty() {
            let origin = knights.pop_first_one();

            *self.attacks |= KNIGHT_MOVES[origin];
        }
    }

    pub fn gen_queen_attacks(&mut self) {
        *self.attacks |=
            slides::get_up_right_attacks(self.attacking_player.queens, self.empty_squares)
                | slides::get_up_left_attacks(self.attacking_player.queens, self.empty_squares)
                | slides::get_down_left_attacks(self.attacking_player.queens, self.empty_squares)
                | slides::get_down_right_attacks(self.attacking_player.queens, self.empty_squares)
                | slides::get_up_attacks(self.attacking_player.queens, self.empty_squares)
                | slides::get_right_attacks(self.attacking_player.queens, self.empty_squares)
                | slides::get_down_attacks(self.attacking_player.queens, self.empty_squares)
                | slides::get_left_attacks(self.attacking_player.queens, self.empty_squares);
    }

    pub fn gen_rook_attacks(&mut self) {
        *self.attacks |= slides::get_up_attacks(self.attacking_player.rooks, self.empty_squares)
            | slides::get_right_attacks(self.attacking_player.rooks, self.empty_squares)
            | slides::get_down_attacks(self.attacking_player.rooks, self.empty_squares)
            | slides::get_left_attacks(self.attacking_player.rooks, self.empty_squares);
    }
}
