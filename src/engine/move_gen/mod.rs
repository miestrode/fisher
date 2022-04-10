use crate::game::board::{BitBoard, Board, PieceKind, Pins, Player};

pub mod attacks;
pub mod move_tables;
pub mod regular;
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

pub enum MoveGenKind {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    // Darn you different moving rules for pawns!
    WhitePawn,
    BlackPawn,
}

/*
TODO: Consider storing more data behind a board reference,
so that we don't copy unneccessary things for each move generator.
The pawn move generator for example doesn't need the enemy attack data.
*/
pub struct PieceAttackGen<'brd> {
    pub kind: MoveGenKind,
    pub pieces: BitBoard, // The pieces to generate moves for.
    pub friendly_pieces: BitBoard,
    pub enemy_pieces: BitBoard,
    pub empty_squares: BitBoard, // 1 represents an empty square in this Bit-Board.
    pub enemy_check_mask: &'brd mut BitBoard,
    pub check_mask: BitBoard,
    pub enemy_king_must_move: &'brd mut bool,
    pub pins: &'brd Pins,
    pub attacks: &'brd mut BitBoard,
    pub enemy_attacks: BitBoard,
    pub enemy_king: BitBoard,
}

impl PieceAttackGen<'_> {
    fn update_ecm(&mut self, origin: BitBoard, attacks: BitBoard) -> BitBoard {
        let hits_king = (attacks & self.enemy_king).is_not_empty();

        if self.enemy_check_mask.is_full() && hits_king {
            *self.enemy_check_mask = origin;
        } else if hits_king {
            *self.enemy_king_must_move = true;
        }

        attacks
    }

    fn update_ecm_for_sliding(&mut self, mask: BitBoard) -> BitBoard {
        let hits_king = (mask & self.enemy_king).is_not_empty();

        if self.enemy_check_mask.is_full() && hits_king {
            *self.enemy_check_mask = mask;
        } else if hits_king {
            *self.enemy_king_must_move = true;
        }

        mask
    }

    fn gen_attacks(&mut self) {
        match self.kind {
            MoveGenKind::King => self.gen_king_moves(),
            MoveGenKind::Queen => self.gen_queen_moves(),
            MoveGenKind::Rook => self.gen_rook_moves(),
            MoveGenKind::Bishop => self.gen_bishop_moves(),
            MoveGenKind::Knight => self.gen_knight_moves(),
            MoveGenKind::WhitePawn => {
                self.gen_white_pawn_attacks();
            }
            MoveGenKind::BlackPawn => {
                self.gen_black_pawn_attacks();
            }
        }
    }
}

/*
TODO: Consider storing more data behind a board reference,
so that we don't copy unneccessary things for each move generator.
The pawn move generator for example doesn't need the enemy attack data.
*/
pub struct PieceMoveGen<'mg, 'brd> {
    pub kind: MoveGenKind,
    pub pieces: BitBoard, // The pieces to generate moves for.
    pub friendly_pieces: BitBoard,
    pub enemy_pieces: BitBoard,
    pub empty_squares: BitBoard, // 1 represents an empty square in this Bit-Board.
    pub pins: &'brd Pins,
    pub moves: &'mg mut Vec<Move>,
    pub check_mask: BitBoard,
    pub enemy_attacks: BitBoard,
    pub enemy_king: BitBoard,
}

impl PieceMoveGen<'_, '_> {
    fn gen_moves(&mut self) {
        match self.kind {
            MoveGenKind::King => self.gen_king_moves(),
            MoveGenKind::Queen => self.gen_queen_moves(),
            MoveGenKind::Rook => self.gen_rook_moves(),
            MoveGenKind::Bishop => self.gen_bishop_moves(),
            MoveGenKind::Knight => self.gen_knight_moves(),
            MoveGenKind::WhitePawn => {
                self.gen_white_pawn_attacks();
                self.gen_white_pawn_pushes()
            }
            MoveGenKind::BlackPawn => {
                self.gen_black_pawn_attacks();
                self.gen_black_pawn_pushes()
            }
        }
    }
}

pub struct MoveGen<'brd> {
    pub board: &'brd Board,
    pub moves: Vec<Move>,
}

impl<'brd> MoveGen<'brd> {
    pub fn new(board: &'brd Board) -> Self {
        Self {
            board,
            moves: Vec::new(),
        }
    }

    pub fn gen_moves(self) -> Vec<Move> {
        let empty_squares = !(self.board.white_state.occupied | self.board.black_state.occupied);

        let (state, enemy_state) = match self.board.player_to_play {
            Player::White => (self.board.white_state, self.board.black_state),
            Player::Black => (self.board.black_state, self.board.white_state),
        };

        // TODO: Consider paralliazing this code with a channel.
        let mut moves = Vec::new();

        PieceMoveGen {
            kind: MoveGenKind::King,
            pieces: state.king,
            friendly_pieces: state.occupied,
            enemy_pieces: enemy_state.occupied,
            empty_squares,
            pins: &state.pins,
            enemy_attacks: enemy_state.attacks,
            enemy_king: enemy_state.king,
            moves: &mut moves,
            check_mask: state.check_mask,
        }
        .gen_moves();

        PieceMoveGen {
            kind: MoveGenKind::Queen,
            pieces: state.queens,
            friendly_pieces: state.occupied,
            enemy_pieces: enemy_state.occupied,
            empty_squares,
            pins: &state.pins,
            enemy_attacks: enemy_state.attacks,
            enemy_king: enemy_state.king,
            moves: &mut moves,
            check_mask: state.check_mask,
        }
        .gen_moves();

        PieceMoveGen {
            kind: MoveGenKind::Rook,
            pieces: state.rooks,
            friendly_pieces: state.occupied,
            enemy_pieces: enemy_state.occupied,
            empty_squares,
            pins: &state.pins,
            enemy_attacks: enemy_state.attacks,
            enemy_king: enemy_state.king,
            moves: &mut moves,
            check_mask: state.check_mask,
        }
        .gen_moves();

        PieceMoveGen {
            kind: MoveGenKind::Bishop,
            pieces: state.bishops,
            friendly_pieces: state.occupied,
            enemy_pieces: enemy_state.occupied,
            empty_squares,
            pins: &state.pins,
            enemy_attacks: enemy_state.attacks,
            enemy_king: enemy_state.king,
            moves: &mut moves,
            check_mask: state.check_mask,
        }
        .gen_moves();

        PieceMoveGen {
            kind: MoveGenKind::Knight,
            pieces: state.knights,
            friendly_pieces: state.occupied,
            enemy_pieces: enemy_state.occupied,
            empty_squares,
            pins: &state.pins,
            enemy_attacks: enemy_state.attacks,
            enemy_king: enemy_state.king,
            moves: &mut moves,
            check_mask: state.check_mask,
        }
        .gen_moves();

        PieceMoveGen {
            kind: match self.board.player_to_play {
                Player::White => MoveGenKind::WhitePawn,
                Player::Black => MoveGenKind::BlackPawn,
            },
            pieces: state.pawns,
            friendly_pieces: state.occupied,
            enemy_pieces: enemy_state.occupied,
            empty_squares,
            pins: &state.pins,
            enemy_attacks: enemy_state.attacks,
            enemy_king: enemy_state.king,
            moves: &mut moves,
            check_mask: state.check_mask,
        }
        .gen_moves();

        moves
    }
}

pub struct AttackGen<'brd> {
    pub board: &'brd mut Board,
}

impl AttackGen<'_> {
    pub fn gen_attacks(&mut self) {
        let empty_squares = !(self.board.white_state.occupied | self.board.black_state.occupied);

        let (state, enemy_state) = match self.board.player_to_play {
            Player::White => (&mut self.board.white_state, &mut self.board.black_state),
            Player::Black => (&mut self.board.black_state, &mut self.board.white_state),
        };

        let mut attacks = BitBoard::empty();

        // TODO: Consider paralliazing this code with a channel.
        PieceAttackGen {
            kind: MoveGenKind::King,
            pieces: state.king,
            friendly_pieces: state.occupied,
            enemy_pieces: enemy_state.occupied,
            empty_squares,
            pins: &state.pins,
            enemy_attacks: enemy_state.attacks,
            enemy_king: enemy_state.king,
            enemy_check_mask: &mut enemy_state.check_mask,
            check_mask: state.check_mask,
            enemy_king_must_move: &mut enemy_state.king_must_move,
            attacks: &mut attacks,
        }
        .gen_attacks();

        PieceAttackGen {
            kind: MoveGenKind::Queen,
            pieces: state.queens,
            friendly_pieces: state.occupied,
            enemy_pieces: enemy_state.occupied,
            empty_squares,
            pins: &state.pins,
            enemy_attacks: enemy_state.attacks,
            enemy_king: enemy_state.king,
            enemy_check_mask: &mut enemy_state.check_mask,
            check_mask: state.check_mask,

            enemy_king_must_move: &mut enemy_state.king_must_move,
            attacks: &mut attacks,
        }
        .gen_attacks();

        PieceAttackGen {
            kind: MoveGenKind::Rook,
            pieces: state.rooks,
            friendly_pieces: state.occupied,
            enemy_pieces: enemy_state.occupied,
            empty_squares,
            pins: &state.pins,
            enemy_attacks: enemy_state.attacks,
            enemy_king: enemy_state.king,
            enemy_check_mask: &mut enemy_state.check_mask,
            check_mask: state.check_mask,

            enemy_king_must_move: &mut enemy_state.king_must_move,
            attacks: &mut attacks,
        }
        .gen_attacks();

        PieceAttackGen {
            kind: MoveGenKind::Bishop,
            pieces: state.bishops,
            friendly_pieces: state.occupied,
            enemy_pieces: enemy_state.occupied,
            empty_squares,
            pins: &state.pins,
            enemy_attacks: enemy_state.attacks,
            enemy_king: enemy_state.king,
            check_mask: state.check_mask,
            enemy_check_mask: &mut enemy_state.check_mask,
            enemy_king_must_move: &mut enemy_state.king_must_move,
            attacks: &mut attacks,
        }
        .gen_attacks();

        PieceAttackGen {
            kind: MoveGenKind::Knight,
            pieces: state.knights,
            friendly_pieces: state.occupied,
            enemy_pieces: enemy_state.occupied,
            empty_squares,
            pins: &state.pins,
            enemy_attacks: enemy_state.attacks,
            enemy_king: enemy_state.king,
            check_mask: state.check_mask,
            enemy_check_mask: &mut enemy_state.check_mask,
            enemy_king_must_move: &mut enemy_state.king_must_move,
            attacks: &mut attacks,
        }
        .gen_attacks();

        PieceAttackGen {
            kind: match self.board.player_to_play {
                Player::White => MoveGenKind::WhitePawn,
                Player::Black => MoveGenKind::BlackPawn,
            },
            pieces: state.pawns,
            friendly_pieces: state.occupied,
            enemy_pieces: enemy_state.occupied,
            empty_squares,
            pins: &state.pins,
            enemy_attacks: enemy_state.attacks,
            enemy_king: enemy_state.king,
            check_mask: state.check_mask,
            enemy_check_mask: &mut enemy_state.check_mask,
            enemy_king_must_move: &mut enemy_state.king_must_move,
            attacks: &mut attacks,
        }
        .gen_attacks();

        state.attacks = attacks;
    }
}
