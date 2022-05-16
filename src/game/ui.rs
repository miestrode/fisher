use yansi::{Color, Style};

use crate::{
    game::board::Board,
    generators::{Move, Square},
    BitBoard, Piece, PieceKind, Player,
};

use std::fmt::{self, Debug, Display, Formatter, Write};

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // The order of reading of the board should be from top left to bottom right so we can't just go over all the squares linearly.
        // I.E: "0, 1, 2, ... 62, 63" won't work.
        for row in 0..8 {
            for column in 0..8 {
                let square = Square((56 - row * 8) + column);
                let piece = self.pieces.get_piece(square);

                let style = Style::new(
                    if let Some(Piece {
                        player: Player::White,
                        ..
                    }) = piece
                    {
                        Color::White
                    } else {
                        Color::Black
                    },
                )
                .bg(if (square.0 + row % 2) % 2 == 0 {
                    Color::Blue
                } else {
                    Color::Cyan
                });

                f.write_str(
                    style
                        .paint(if let Some(piece) = piece {
                            format!("{} ", piece.piece_kind)
                        } else {
                            String::from("  ")
                        })
                        .to_string()
                        .as_str(),
                )?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

// TODO: Refactor this.
impl Debug for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let occupied = self.active.pieces | self.inactive.pieces;
        let current_attacks = self.inactive.attacks;
        let illegal_squares = !self.active.check_mask;
        let pinned_squares = self.active.pins.get_all_pins();
        let ep_square = self.ep_capture_point;

        // The order of reading of the board should be from top left to bottom right so we can't just go over all the squares linearly.
        // I.E: "0, 1, 2, ... 62, 63" won't work.
        for row in 0..8 {
            for column in 0..8 {
                let square = Square((56 - row * 8) + column);
                let piece = self.pieces.get_piece(square);

                let style = Style::new(if let Some(piece) = piece {
                    if !occupied.get_bit(square) {
                        Color::Red
                    } else {
                        match piece.player {
                            Player::Black => Color::Black,
                            Player::White => Color::White,
                        }
                    }
                } else {
                    Color::Red
                })
                .bg(if pinned_squares.get_bit(square) {
                    Color::Magenta
                } else if illegal_squares.get_bit(square) {
                    Color::Yellow
                } else if ep_square.get_bit(square) {
                    Color::Green
                } else if current_attacks.get_bit(square) {
                    Color::Red
                } else if (square.0 + row % 2) % 2 == 0 {
                    Color::Blue
                } else {
                    Color::Cyan
                });

                f.write_str(
                    style
                        .paint(if let Some(piece) = self.pieces.get_piece(square) {
                            format!("{} ", piece.piece_kind)
                        } else {
                            String::from("  ")
                        })
                        .to_string()
                        .as_str(),
                )?;
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
                if self.get_bit(Square((64 - row * 8) + column)) {
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

impl Display for PieceKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            PieceKind::King => '♚',
            PieceKind::Queen => '♛',
            PieceKind::Rook => '♜',
            PieceKind::Bishop => '♝',
            PieceKind::Knight => '♞',
            PieceKind::Pawn => '♟',
        })
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let row = self.0 / 8 + 1; // Make the range be from 1 to 8.
        let column = self.0 % 8; // The range of this doesn't matter as it is converted into letters.

        write!(
            f,
            "{}{}",
            match column {
                0 => 'a',
                1 => 'b',
                2 => 'c',
                3 => 'd',
                4 => 'e',
                5 => 'f',
                6 => 'g',
                7 => 'h',
                _ => unreachable!(),
            },
            row
        )
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Player::White => "white",
            Player::Black => "black",
        })
    }
}

// TODO: Rework this implementation.
impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Move::Promotion {
                origin,
                target,
                promotion_to,
            } => {
                Display::fmt(origin, f)?;
                Display::fmt(target, f)?;
                f.write_char('=')?;
                f.write_char(promotion_to.into_piece_char())
            }
            Move::CastleKS => f.write_str("ks"),
            Move::CastleQS => f.write_str("qs"),
            Move::Regular {
                origin,
                target,
                piece_kind,
                ..
            } => {
                f.write_char(piece_kind.into_piece_char())?;
                Display::fmt(origin, f)?;
                Display::fmt(target, f)
            }
            Move::EnPassant { origin } => Display::fmt(origin, f),
        }
    }
}
