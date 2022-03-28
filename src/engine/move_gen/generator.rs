use crate::game::board::{Board, Player};

use super::{
    knight::KnightMoveGen,
    pawns::{BlackPawnMoveGen, WhitePawnMoveGen},
    sliding_pieces::{bishop::BishopMoveGen, queen::QueenMoveGen, rook::RookMoveGen},
    GenMoves, Move,
};

struct MoveGen {
    board: Board,
    moves: Vec<Move>,
}

impl MoveGen {
    fn run(mut self) -> Vec<Move> {
        let empty_squares = !(self.board.black_state.occupied & self.board.white_state.occupied);

        match self.board.player_to_play {
            Player::White => {
                WhitePawnMoveGen {
                    enemy_pieces: self.board.black_state.occupied,
                    empty_squares,
                    pawns: self.board.white_state.pawns,
                    check_mask: self.board.check_mask,
                    moves: &mut self.moves,
                    pins: self.board.white_state.pins,
                }
                .gen_moves();

                KnightMoveGen {
                    moves: &mut self.moves,
                    friendly_pieces: self.board.white_state.occupied,
                    knights: self.board.white_state.knights,
                    pins: self.board.white_state.pins,
                    check_mask: self.board.check_mask,
                }
                .gen_moves();

                BishopMoveGen {
                    moves: &mut self.moves,
                    friendly_pieces: self.board.white_state.occupied,
                    empty_squares,
                    pins: self.board.white_state.pins,
                    bishops: self.board.white_state.bishops,
                    check_mask: self.board.check_mask,
                }
                .gen_moves();

                RookMoveGen {
                    moves: &mut self.moves,
                    friendly_pieces: self.board.white_state.occupied,
                    empty_squares,
                    pins: self.board.white_state.pins,
                    rooks: self.board.white_state.rooks,
                    check_mask: self.board.check_mask,
                }
                .gen_moves();

                QueenMoveGen {
                    moves: &mut self.moves,
                    friendly_pieces: self.board.white_state.occupied,
                    empty_squares,
                    pins: self.board.white_state.pins,
                    queens: self.board.white_state.queens,
                    check_mask: self.board.check_mask,
                }
                .gen_moves();
            }
            Player::Black => {
                BlackPawnMoveGen {
                    enemy_pieces: self.board.white_state.occupied,
                    empty_squares,
                    pawns: self.board.black_state.pawns,
                    check_mask: self.board.check_mask,
                    moves: &mut self.moves,
                    pins: self.board.black_state.pins,
                }
                .gen_moves();

                KnightMoveGen {
                    moves: &mut self.moves,
                    friendly_pieces: self.board.black_state.occupied,
                    pins: self.board.black_state.pins,
                    knights: self.board.black_state.knights,
                    check_mask: self.board.check_mask,
                }
                .gen_moves();

                BishopMoveGen {
                    moves: &mut self.moves,
                    friendly_pieces: self.board.black_state.occupied,
                    empty_squares,
                    pins: self.board.black_state.pins,
                    bishops: self.board.black_state.bishops,
                    check_mask: self.board.check_mask,
                }
                .gen_moves();

                RookMoveGen {
                    moves: &mut self.moves,
                    friendly_pieces: self.board.black_state.occupied,
                    empty_squares,
                    pins: self.board.black_state.pins,
                    rooks: self.board.black_state.rooks,
                    check_mask: self.board.check_mask,
                }
                .gen_moves();

                QueenMoveGen {
                    moves: &mut self.moves,
                    friendly_pieces: self.board.black_state.occupied,
                    empty_squares,
                    pins: self.board.black_state.pins,
                    queens: self.board.black_state.queens,
                    check_mask: self.board.check_mask,
                }
                .gen_moves();
            }
        }

        self.moves
    }
}
