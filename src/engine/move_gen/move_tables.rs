use crate::game::board::BitBoard;

macro_rules! gen_move_table {
    ($function:expr) => {{
        let mut square = 0;
        let mut moves = [BitBoard::empty(); 64];

        while square < 64 {
            // Create a bit-board of all knight moves assuming the board is empty.
            moves[square] = $function(square);
            square += 1;
        }

        moves
    }};
}

const fn gen_knight_moves(square: usize) -> BitBoard {
    let piece = BitBoard(1 << square);

    // Create a bit-board of all knight moves assuming the board is empty.
    piece.move_up(1).move_up_right()
        | piece.move_up(1).move_up_left()
        | piece.move_left().move_up_left()
        | piece.move_left().move_down_left()
        | piece.move_down(1).move_down_left()
        | piece.move_down(1).move_down_right()
        | piece.move_right().move_down_right()
        | piece.move_right().move_up_right()
}

const fn gen_king_moves(square: usize) -> BitBoard {
    let piece = BitBoard(1 << square);

    let middle = piece.move_left() | piece.move_right();
    let horizontal = middle | piece;

    horizontal.move_up(1) | middle | horizontal.move_down(1)
}

// This is a look-up table for each move a knight could make in a given square. The index of the move bitboard is the origin square of the knight.
pub const KNIGHT_MOVES: [BitBoard; 64] = gen_move_table!(gen_knight_moves);

// Same deal, but for the king.
pub const KING_MOVES: [BitBoard; 64] = gen_move_table!(gen_king_moves);
