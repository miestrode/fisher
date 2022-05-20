use std::{mem, str::FromStr};

use crate::{generators::Square, BitBoard, Piece, Player};

use super::board::{Board, BoardPieces, EnPassant, PlayerState};

impl FromStr for BoardPieces {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut board_pieces = BoardPieces::empty();

        let rows = s.split('/').collect::<Vec<_>>();

        if rows.len() != 8 {
            Err("Input contains the wrong amount of rows")
        } else {
            let mut row_offset = 64;

            for row in rows {
                row_offset -= 8;
                let mut column_offset = -1; // This goes from 0-7, so we want to make sure the first increase puts us at index 0.

                for character in row.chars() {
                    match character {
                        '1' => column_offset += 1,
                        '2' => column_offset += 2,
                        '3' => column_offset += 3,
                        '4' => column_offset += 4,
                        '5' => column_offset += 5,
                        '6' => column_offset += 6,
                        '7' => column_offset += 7,
                        '8' => column_offset += 8,
                        _ => {
                            column_offset += 1;
                            board_pieces.pieces[(row_offset + column_offset) as usize] =
                                Some(match character {
                                    'K' => Piece::WHITE_KING,
                                    'Q' => Piece::WHITE_QUEEN,
                                    'R' => Piece::WHITE_ROOK,
                                    'B' => Piece::WHITE_BISHOP,
                                    'N' => Piece::WHITE_KNIGHT,
                                    'P' => Piece::WHITE_PAWN,
                                    'k' => Piece::BLACK_KING,
                                    'q' => Piece::BLACK_QUEEN,
                                    'r' => Piece::BLACK_ROOK,
                                    'b' => Piece::BLACK_BISHOP,
                                    'n' => Piece::BLACK_KNIGHT,
                                    'p' => Piece::BLACK_PAWN,
                                    _ => return Err(
                                        "Input contains an invalid character in one of the rows",
                                    ),
                                });
                        }
                    }

                    if column_offset > 7 {
                        return Err(
                            "Input contains an overflowed row (The column offset is too high)",
                        );
                    }
                }
            }

            Ok(board_pieces)
        }
    }
}

impl FromStr for Player {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            Err("Input must contain a single character")
        } else {
            match s.chars().next().unwrap() {
                'w' => Ok(Player::White),
                'b' => Ok(Player::Black),
                _ => Err("Input must be a 'w' or 'b'"),
            }
        }
    }
}

impl FromStr for Square {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            Err("Input must contain two characters")
        } else {
            let mut characters = s.chars();

            Ok(Square(
                match characters.next().unwrap() {
                    'a' => 0,
                    'b' => 1,
                    'c' => 2,
                    'd' => 3,
                    'e' => 4,
                    'f' => 5,
                    'g' => 6,
                    'h' => 7,
                    _ => return Err("Input's column descriptor must be a character from a to h"),
                } + match characters.next().unwrap() {
                    '1' => 0,
                    '2' => 8,
                    '3' => 16,
                    '4' => 24,
                    '5' => 32,
                    '6' => 40,
                    '7' => 48,
                    '8' => 56,
                    _ => return Err("Input's row descriptor must be a digit from 1 to 8"),
                },
            ))
        }
    }
}

impl FromStr for Board {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ").collect::<Vec<_>>();

        if parts.len() != 6 {
            Err("Input must contain 6 parts separated by spaces")
        } else {
            let board_pieces = BoardPieces::from_str(parts[0])?;
            let current_player = Player::from_str(parts[1])?;

            let ep_capture_point = match parts[3] {
                "-" => BitBoard::empty(),
                square => Square::from_str(square)?.into(),
            };

            let ep_pawn = match current_player {
                Player::White => ep_capture_point.move_down(1),
                Player::Black => ep_capture_point.move_up(1),
            };

            // These values currently aren't needed anywhere.
            if let Err(_) = parts[4].parse::<u32>() {
                return Err("Input contains invalid number for half-moves");
            }
            if let Err(_) = parts[5].parse::<u32>() {
                return Err("Input contains invalid number for full-moves");
            }

            let mut moving_player = PlayerState::blank();
            for (square_index, piece) in board_pieces.pieces.into_iter().enumerate() {
                if let Some(Piece {
                    piece_kind,
                    player: Player::White,
                }) = piece
                {
                    moving_player.place_piece(piece_kind, Square(square_index as u32))
                }
            }

            let mut moved_player = PlayerState::blank();
            for (square_index, piece) in board_pieces.pieces.into_iter().enumerate() {
                if let Some(Piece {
                    piece_kind,
                    player: Player::Black,
                }) = piece
                {
                    moved_player.place_piece(piece_kind, Square(square_index as u32))
                }
            }

            if parts[2] != "-" {
                moving_player.can_castle_ks = parts[2].contains("K");
                moving_player.can_castle_qs = parts[2].contains("Q");
                moved_player.can_castle_ks = parts[2].contains("k");
                moved_player.can_castle_qs = parts[2].contains("q");

                // This would indicate the part contains some characters other than K, Q, k or q.
                if (moving_player.can_castle_ks as usize
                    + moving_player.can_castle_qs as usize
                    + moved_player.can_castle_ks as usize
                    + moved_player.can_castle_qs as usize)
                    != parts[2].len()
                {
                    return Err("Input contains invalid data for castling information");
                }
            }

            // So far "moving_player" and "moved_player" were used as white and black. This is of course not actually true, and this code fixes that.
            if current_player == Player::Black {
                mem::swap(&mut moving_player, &mut moved_player);
            }

            let mut board = Board {
                moving_player,
                moved_player,
                current_player,
                ep_info: EnPassant {
                    capture_point: ep_capture_point,
                    pawn: ep_pawn,
                },
                pieces: board_pieces,
            };

            board.update_move_constraints();

            Ok(board)
        }
    }
}
