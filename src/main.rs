use std::str::FromStr;

use fisher::{divide, game::board::Board, generators::Move};

fn main() {
    let mut board = Board::default();

    board.make_move(Move::from_str("pb2b3").unwrap());
    board.make_move(Move::from_str("pe7e6").unwrap());
    board.make_move(Move::from_str("bc1a3").unwrap());
    board.make_move(Move::from_str("bf8b4").unwrap());
    board.make_move(Move::from_str("ba3b4").unwrap());

    println!("{:?}", board);

    println!("Found {} total leaves", divide(board, 1));
}
