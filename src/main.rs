use std::{io, time::Instant};

use fisher::{
    game::board::Board,
    generators::{move_tables::KNIGHT_MOVES, MoveGen},
};

fn search(depth: u32) -> u32 {
    search_inner(Board::new(), depth)
}

fn search_inner(board: Board, depth: u32) -> u32 {
    if depth == 0 {
        1
    } else {
        let moves = MoveGen::run(board);

        moves
            .into_iter()
            .map(|chess_move| {
                let mut board_copy = board;

                board_copy.make_move(chess_move);

                search_inner(board, depth - 1)
            })
            .sum()
    }
}

fn main() {
    let mut plies = String::new();
    io::stdin()
        .read_line(&mut plies)
        .expect("Failed to read line.");

    let plies: u32 = plies
        .trim()
        .parse::<u32>()
        .expect("Please enter a valid unsigned 32 bit integer.");

    let instant = Instant::now();

    println!(
        "Generated {} moves, took: {:.3}s",
        search(plies),
        instant.elapsed().as_secs_f64()
    )
}
