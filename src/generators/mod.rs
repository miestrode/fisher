use crate::{
    game::board::{Board, PlayerState},
    BitBoard, PieceKind, Player,
};

pub mod attacks;
pub mod move_tables;
pub mod moves;
pub mod slides;

#[derive(Clone, Copy, Debug)]
pub struct Position(pub u32);

impl Position {
    /*
    Currently unneeded:

    fn move_right(self, amount: u8) -> Position {
        Position(self.0 + amount)
    }

    fn move_left(self, amount: u8) -> Position {
        Position(self.0 - amount)
    }
    */

    fn move_up(self, amount: u32) -> Position {
        Position(self.0 + 8 * amount)
    }

    fn move_down(self, amount: u32) -> Position {
        Position(self.0 - 8 * amount)
    }

    fn move_up_right(&self, amount: u32) -> Position {
        Position(self.0 + 9 * amount)
    }

    fn move_down_left(&self, amount: u32) -> Position {
        Position(self.0 - 9 * amount)
    }

    fn move_up_left(self, amount: u32) -> Position {
        Position(self.0 + 7 * amount)
    }

    fn move_down_right(self, amount: u32) -> Position {
        Position(self.0 - 7 * amount)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Move {
    pub origin: Position,
    pub target: Position,
    pub piece_kind: PieceKind, // If we know which piece is moving, it's easier to locate it.
}

pub struct AttackGen<'brd> {
    active: PlayerState,
    inactive: PlayerState,
    empty_squares: BitBoard,
    attacks: &'brd mut BitBoard,
}

impl<'brd> AttackGen<'brd> {
    pub fn run(board: &'brd mut Board) {
        Self {
            active: board.active,
            inactive: board.inactive,
            empty_squares: !(board.active.occupied | board.inactive.occupied),
            attacks: &mut board.active.attacks,
        }
        .gen_attacks(board.player_to_play);
    }

    fn gen_attacks(&mut self, player_to_play: Player) {
        self.gen_king_moves();
        self.gen_queen_moves();
        self.gen_rook_moves();
        self.gen_bishop_moves();
        self.gen_knight_moves();

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
    moves: Vec<Move>,
}

impl MoveGen {
    pub fn run(board: Board) -> Vec<Move> {
        let mut move_gen = Self {
            active: board.active,
            inactive: board.inactive,
            empty_squares: !(board.active.occupied | board.inactive.occupied),
            moves: Vec::with_capacity(31), // Chess has a branching factor of 31 on average.
        };

        move_gen.gen_moves(board.player_to_play);

        move_gen.moves
    }

    fn gen_moves(&mut self, player_to_play: Player) {
        self.gen_king_moves();
        self.gen_queen_moves();
        self.gen_rook_moves();
        self.gen_bishop_moves();
        self.gen_knight_moves();

        match player_to_play {
            Player::White => {
                self.gen_white_pawn_pushes();
                self.gen_white_pawn_attacks();
            }
            Player::Black => {
                self.gen_black_pawn_pushes();
                self.gen_black_pawn_attacks();
            }
        }
    }
}
