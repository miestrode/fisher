use crate::game::board::PieceKind;

pub mod generator;
pub mod king;
pub mod knight;
pub mod move_tables;
pub mod pawns;
pub mod sliding_pieces;

#[derive(Clone, Copy)]
pub struct Position(pub u8);

pub struct Move {
    pub from: Position,
    pub to: Position,
    pub piece_kind: PieceKind, // If we know which piece is moving, it's easier to locate it.
}

pub trait GenMoves {
    fn gen_moves(self);
}
