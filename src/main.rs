use std::{io, time::Instant};

use fisher::{game::board::Board, generators::MoveGen};

use rand::prelude::*;

fn play() {
    let mut board = Board::new();

    loop {
        let now = Instant::now();

        let moves = MoveGen::run(board);

        println!(
            "{}Moves generated: {} | Took: {:.3}ns",
            board,
            moves.len(),
            now.elapsed().as_nanos()
        );

        io::stdin()
            .read_line(&mut String::new())
            .expect("Failed to wait for input.");

        board.make_move(*moves.choose(&mut thread_rng()).unwrap());
    }
}

fn main() {
    play();
}
