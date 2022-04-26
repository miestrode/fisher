use std::fmt::Debug;

use crate::{
    game::board::{Board, PlayerState},
    BitBoard, PieceKind, Player, PROMOTION_PIECES,
};

pub mod attacks;
pub mod moves;
pub mod slides;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Square(pub u32);

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
        is_en_passant: bool,
    },
    Promotion {
        origin: Square,
        target: Square,
        promotion_to: PieceKind,
    },
    CastleKS,
    CastleQS,
}

pub struct AttackGen<'brd> {
    active: PlayerState,
    empty_squares: BitBoard,
    attacks: &'brd mut BitBoard,
}

impl<'brd> AttackGen<'brd> {
    pub fn run(board: &'brd mut Board) {
        board.active.attacks = BitBoard::empty();

        Self {
            active: board.active,
            // We remove the inactive player's king from the empty Positionuares since A piece cannot protect itself from a ray.
            empty_squares: !((board.active.occupied | board.inactive.occupied)
                ^ board.inactive.king),
            attacks: &mut board.active.attacks,
        }
        .gen_attacks(board.player_to_play);
    }

    fn gen_attacks(&mut self, player_to_play: Player) {
        // All the pieces move no matter what, since both the check mask and the KMM flag must have been satisfied.
        self.gen_king_attacks();
        self.gen_queen_attacks();
        self.gen_rook_attacks();
        self.gen_bishop_attacks();
        self.gen_knight_attacks();

        match player_to_play {
            Player::White => self.gen_white_pawn_attacks(),
            Player::Black => self.gen_black_pawn_attacks(),
        }
    }
}

pub struct MoveGen {
    active: PlayerState,
    inactive: PlayerState,
    empty_squares: BitBoard,
    en_passant: BitBoard,
    moves: Vec<Move>,
}

impl MoveGen {
    pub fn run(board: Board) -> Vec<Move> {
        let mut move_gen = Self {
            active: board.active,
            inactive: board.inactive,
            empty_squares: !(board.active.occupied | board.inactive.occupied),
            en_passant: board.en_passant,
            moves: Vec::with_capacity(31), // Chess has a branching factor of 31 on average.
        };

        move_gen.gen_moves(board.player_to_play);

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
        self.gen_king_moves();

        if !self.active.king_must_move {
            self.gen_queen_moves();
            self.gen_rook_moves();
            self.gen_bishop_moves();
            self.gen_knight_moves();

            match player_to_play {
                Player::White => {
                    self.gen_white_pawn_pushes();
                    self.gen_white_pawn_attacks();
                    self.gen_white_pawn_en_passants();
                }
                Player::Black => {
                    self.gen_black_pawn_pushes();
                    self.gen_black_pawn_attacks();
                    self.gen_black_pawn_en_passants();
                }
            }

            if self.active.check_mask.is_full() {
                self.castle_king_side();
                self.castle_queen_side();
            }
        }
    }
}
