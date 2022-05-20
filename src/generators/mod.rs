use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
    str::FromStr,
};

use crate::{
    game::board::{Board, EnPassant, PlayerState},
    BitBoard, PieceKind, Player, PROMOTION_PIECES,
};

pub mod attacks;
pub mod moves;
pub mod slides;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Square(pub u32);

impl<T> Index<Square> for [T; 64] {
    type Output = T;

    fn index(&self, index: Square) -> &Self::Output {
        &self[index.0 as usize]
    }
}

impl<T> IndexMut<Square> for [T; 64] {
    fn index_mut(&mut self, index: Square) -> &mut Self::Output {
        &mut self[index.0 as usize]
    }
}

impl Square {
    pub const A1: Square = Square(0b000000);
    pub const B1: Square = Square(0b000001);
    pub const C1: Square = Square(0b000010);
    pub const D1: Square = Square(0b000011);
    pub const E1: Square = Square(0b000100);
    pub const F1: Square = Square(0b000101);
    pub const G1: Square = Square(0b000110);
    pub const H1: Square = Square(0b000111);
    pub const A2: Square = Square(0b001000);
    pub const B2: Square = Square(0b001001);
    pub const C2: Square = Square(0b001010);
    pub const D2: Square = Square(0b001011);
    pub const E2: Square = Square(0b001100);
    pub const F2: Square = Square(0b001101);
    pub const G2: Square = Square(0b001110);
    pub const H2: Square = Square(0b001111);
    pub const A3: Square = Square(0b010000);
    pub const B3: Square = Square(0b010001);
    pub const C3: Square = Square(0b010010);
    pub const D3: Square = Square(0b010011);
    pub const E3: Square = Square(0b010100);
    pub const F3: Square = Square(0b010101);
    pub const G3: Square = Square(0b010110);
    pub const H3: Square = Square(0b010111);
    pub const A4: Square = Square(0b011000);
    pub const B4: Square = Square(0b011001);
    pub const C4: Square = Square(0b011010);
    pub const D4: Square = Square(0b011011);
    pub const E4: Square = Square(0b011100);
    pub const F4: Square = Square(0b011101);
    pub const G4: Square = Square(0b011110);
    pub const H4: Square = Square(0b011111);
    pub const A5: Square = Square(0b100000);
    pub const B5: Square = Square(0b100001);
    pub const C5: Square = Square(0b100010);
    pub const D5: Square = Square(0b100011);
    pub const E5: Square = Square(0b100100);
    pub const F5: Square = Square(0b100101);
    pub const G5: Square = Square(0b100110);
    pub const H5: Square = Square(0b100111);
    pub const A6: Square = Square(0b101000);
    pub const B6: Square = Square(0b101001);
    pub const C6: Square = Square(0b101010);
    pub const D6: Square = Square(0b101011);
    pub const E6: Square = Square(0b101100);
    pub const F6: Square = Square(0b101101);
    pub const G6: Square = Square(0b101110);
    pub const H6: Square = Square(0b101111);
    pub const A7: Square = Square(0b110000);
    pub const B7: Square = Square(0b110001);
    pub const C7: Square = Square(0b110010);
    pub const D7: Square = Square(0b110011);
    pub const E7: Square = Square(0b110100);
    pub const F7: Square = Square(0b110101);
    pub const G7: Square = Square(0b110110);
    pub const H7: Square = Square(0b110111);
    pub const A8: Square = Square(0b111000);
    pub const B8: Square = Square(0b111001);
    pub const C8: Square = Square(0b111010);
    pub const D8: Square = Square(0b111011);
    pub const E8: Square = Square(0b111100);
    pub const F8: Square = Square(0b111101);
    pub const G8: Square = Square(0b111110);
    pub const H8: Square = Square(0b111111);

    pub fn move_up(self, amount: u32) -> Square {
        Square(self.0 + 8 * amount)
    }

    pub fn move_down(self, amount: u32) -> Square {
        Square(self.0 - 8 * amount)
    }

    pub fn move_up_right(&self, amount: u32) -> Square {
        Square(self.0 + 9 * amount)
    }

    pub fn move_down_left(&self, amount: u32) -> Square {
        Square(self.0 - 9 * amount)
    }

    pub fn move_up_left(self, amount: u32) -> Square {
        Square(self.0 + 7 * amount)
    }

    pub fn move_down_right(self, amount: u32) -> Square {
        Square(self.0 - 7 * amount)
    }

    // Returns a value representing the row from 0 to 7.
    pub fn get_row(&self) -> u32 {
        self.0 / 8
    }
}

impl TryFrom<&str> for Square {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 2 {
            Err("wrong length")
        } else {
            let mut characters = value.chars();

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
                    _ => return Err("invalid character for file"),
                } + 8
                    * (if let Ok(value) = characters.next().unwrap().to_string().parse::<u32>() {
                        value
                    } else {
                        return Err("invalid character for rank");
                    } - 1),
            ))
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Move {
    Regular {
        origin: Square,
        target: Square,
        piece_kind: PieceKind,
        double_push: bool,
    },
    EnPassant {
        origin: Square,
    },
    Promotion {
        origin: Square,
        target: Square,
        promotion_to: PieceKind,
    },
    CastleKS,
    CastleQS,
}

impl FromStr for Move {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ks" => Ok(Move::CastleKS),
            "qs" => Ok(Move::CastleQS),
            _ => Square::from_str(s)
                .map(|square| Move::EnPassant { origin: square })
                .or_else(|_| {
                    if s.len() == 5 {
                        let piece_kind = PieceKind::from_str(&s[0..1])?;
                        let origin = Square::from_str(&s[1..3])?;
                        let target = Square::from_str(&s[3..5])?;

                        Ok(Move::Regular {
                            origin,
                            target,
                            piece_kind,
                            double_push: piece_kind == PieceKind::Pawn
                                && ((origin.get_row() == 1 && target.get_row() == 3)
                                    || (origin.get_row() == 6 && target.get_row() == 4)),
                        })
                    } else if s.len() == 6 && &s[4..5] == "=" {
                        let origin = Square::from_str(&s[0..2])?;
                        let target = Square::from_str(&s[2..4])?;
                        let promotion_to = PieceKind::from_str(&s[5..6])?;

                        Ok(Move::Promotion {
                            origin,
                            target,
                            promotion_to,
                        })
                    } else {
                        Err("Input is not a valid move")
                    }
                }),
        }
    }
}

pub struct AttackGen<'brd> {
    attacking_player: PlayerState,
    empty_squares: BitBoard,
    attacks: &'brd mut BitBoard,
}

impl<'brd> AttackGen<'brd> {
    pub fn run(board: &'brd mut Board) {
        board.moved_player.attacks = BitBoard::empty();

        Self {
            attacking_player: board.moved_player,
            // NOTICE: The king is ignored as a piece, since the attacks are supposed to show all attacked squares (when the king isn't on the board).
            // They do that since when the king is attacked by a sliding piece, we don't want him to retreat in the direction of the attacking slide.
            empty_squares: !(board.moving_player.pieces | board.moved_player.pieces)
                | board.moving_player.king,
            attacks: &mut board.moved_player.attacks,
        }
        .gen_attacks(!board.current_player); // The current player is the one being attacked, so we actually need the inactive player's color.
    }

    fn gen_attacks(&mut self, attacking_player: Player) {
        // All the pieces move no matter what, since both the check mask and the KMM flag must have been satisfied. (We were attacked by the player that last moved).
        self.gen_king_attacks();
        self.gen_queen_attacks();
        self.gen_rook_attacks();
        self.gen_bishop_attacks();
        self.gen_knight_attacks();

        match attacking_player {
            Player::White => self.gen_white_pawn_attacks(),
            Player::Black => self.gen_black_pawn_attacks(),
        }
    }
}

pub struct MoveGen {
    moving_player: PlayerState,
    moved_player: PlayerState,
    empty_squares: BitBoard,
    ep_info: EnPassant,
    moves: Vec<Move>,
}

impl MoveGen {
    pub fn run(board: Board) -> Vec<Move> {
        let mut move_gen = Self {
            moving_player: board.moving_player,
            moved_player: board.moved_player,
            empty_squares: !(board.moving_player.pieces | board.moved_player.pieces),
            ep_info: board.ep_info,
            moves: Vec::with_capacity(31), // Chess has a branching factor of 31 on average.
        };

        move_gen.gen_moves(board.current_player);

        move_gen.moves
    }

    pub fn add_promotions(&mut self, origin: Square, target: Square) {
        for piece in PROMOTION_PIECES {
            self.moves.push(Move::Promotion {
                origin,
                target,
                promotion_to: piece,
            })
        }
    }

    fn gen_moves(&mut self, player_to_play: Player) {
        if !self.moving_player.king_must_move {
            match player_to_play {
                Player::White => {
                    self.gen_white_pawn_pushes();
                    self.gen_white_pawn_attacks();
                    self.gen_white_pawn_en_passants();

                    // Even though castling is a king move, it cannot happen during check (The above conditional checks if the king is in double check).
                    if self.moving_player.isnt_in_check() {
                        self.white_castle_king_side();
                        self.white_castle_queen_side();
                    }
                }
                Player::Black => {
                    self.gen_black_pawn_pushes();
                    self.gen_black_pawn_attacks();
                    self.gen_black_pawn_en_passants();

                    // Even though castling is a king move, it cannot happen during check (The above conditional checks if the king is in double check).
                    if self.moving_player.isnt_in_check() {
                        self.black_castle_king_side();
                        self.black_castle_queen_side();
                    }
                }
            }

            self.gen_knight_moves();
            self.gen_bishop_moves();
            self.gen_rook_moves();
            self.gen_queen_moves();
        }

        self.gen_king_moves();
    }
}
