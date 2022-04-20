use crate::{game::board::Board, generators::Position, BitBoard, Piece, PieceKind, Player};

use std::fmt::{self, Display, Write};

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        // The order of reading of the board should be from top left to bottom right so we can't just go over all the squares linearly.
        // I.E: "0, 1, 2, ... 62, 63" won't work.
        for row in 0..8 {
            for column in 0..8 {
                if let Some(piece) = self.get_piece(Position((56 - row * 8) + column)) {
                    f.write_fmt(format_args!("{} ", piece))?;
                } else {
                    f.write_str("~ ")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Same deal as above.
        for row in 1..=8 {
            for column in 0..8 {
                if self.get_bit(Position((64 - row * 8) + column)) {
                    f.write_char('1')?;
                } else {
                    f.write_char('.')?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self.player {
            Player::White => match self.piece_kind {
                PieceKind::King => '♔',
                PieceKind::Queen => '♕',
                PieceKind::Rook => '♖',
                PieceKind::Bishop => '♗',
                PieceKind::Knight => '♘',
                PieceKind::Pawn => '♙',
            },
            Player::Black => match self.piece_kind {
                PieceKind::King => '♚',
                PieceKind::Queen => '♛',
                PieceKind::Rook => '♜',
                PieceKind::Bishop => '♝',
                PieceKind::Knight => '♞',
                PieceKind::Pawn => '♟',
            },
        })
    }
}
