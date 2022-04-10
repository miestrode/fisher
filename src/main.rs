use std::{io, thread, time::Duration};

use crossterm::{
    cursor,
    style::Print,
    terminal::{Clear, ClearType},
    Result,
};
use fisher::{engine::move_gen::MoveGen, game::board::Board};
use rand::prelude::*;

fn main() -> Result<()> {
    let mut board = Board::new();

    loop {
        let moves = MoveGen::new(&board).gen_moves();
        board.make_move(moves[thread_rng().gen_range(0..moves.len())]);

        crossterm::execute!(
            io::stdout(),
            cursor::MoveTo(0, 0),
            Clear(ClearType::All),
            Print(board)
        )?;

        thread::sleep(Duration::from_secs_f32(0.3));
    }
}
