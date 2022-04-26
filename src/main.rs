use fisher::{game::board::Board, generators::MoveGen};

use rand::prelude::*;

fn play() {
    let mut board = Board::new();

    for _ in 0..100 {
        let moves = MoveGen::run(board);

        if moves.len() == 0 {
            return;
        }

        print!("{}", board);

        let played_move = *moves.choose(&mut thread_rng()).unwrap();

        println!("Playing {} as {}.\n", played_move, board.player_to_play);

        board.make_move(played_move);
    }
}

fn main() {
    loop {
        play();
    }
}
