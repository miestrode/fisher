use std::mem::size_of;

enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

struct Piece {
    is_white: bool,
    piece_type: PieceType,
}

pub fn get_piece_size() -> usize {
    size_of::<Piece>()
}
