use std::time::Instant;

use fisher::engine::move_gen::{sliding_pieces, Position};

fn main() {
    let duration = Instant::now();

    println!(
        "Direction: {:?}, time: {:.3}ns",
        sliding_pieces::gen_relative_direction(Position(0), Position(9)),
        duration.elapsed().as_nanos()
    );
}
