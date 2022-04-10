use fisher::{engine::move_gen::MoveGen, game::board::Board};

fn main() {
    let mut board = Board::new();
    let moves = MoveGen::new(&board).gen_moves();

    board.make_move(moves[0]);

    println!("{}", board);
}
