pub mod king;
pub mod knight;
pub mod pawns;
pub mod rook;
pub mod sliding;

#[derive(Clone, Copy)]
pub struct Position(pub u8);

impl Position {
    pub fn new(value: u8) -> Self {
        Self(value)
    }
}

pub struct Move {
    from: Position,
    to: Position,
}

impl Move {
    // This assumes the piece making the move is a pawn.
    pub fn is_double_push(&self) -> bool {
        (self.to.0 - 16) == self.from.0
    }
}

pub trait GenMoves {
    fn gen_moves(self) -> Vec<Move>;
}
