use std::time::Instant;

use fisher::{game::board::Board, generators::MoveGen};

fn search(depth: u32) -> u32 {
    search_inner(Board::new(), depth)
}

fn search_inner(board: Board, depth: u32) -> u32 {
    if depth == 0 {
        1
    } else {
        let moves = MoveGen::run(board);

        if moves.len() == 0 {
            println!("{}", board);
            1
        } else {
            moves
                .into_iter()
                .map(|chess_move| {
                    let mut board_copy = board;

                    board_copy.make_move(chess_move);

                    search_inner(board_copy, depth - 1)
                })
                .sum()
        }
    }
}

fn main() {
    let elapsed = Instant::now();

    println!(
        "Moves found: {}, Time spent: {:.4}s",
        search(6),
        elapsed.elapsed().as_secs_f64()
    );
}
