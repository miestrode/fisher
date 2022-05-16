use crate::{
    generators::{
        slides::{
            get_down_attacks, get_down_left_attacks, get_down_right_attacks, get_left_attacks,
            get_right_attacks, get_up_attacks, get_up_left_attacks, get_up_right_attacks,
        },
        AttackGen, Move, Square,
    },
    tables::KNIGHT_MOVES,
    BitBoard, Piece, PieceKind, Pins, Player, LEFT_ROOK_ORIGINS, RIGHT_ROOK_ORIGINS,
};
use std::{mem, str::FromStr};

#[derive(Clone, Copy)]
pub struct PlayerState {
    pub king: BitBoard,
    pub queens: BitBoard,
    pub rooks: BitBoard,
    pub bishops: BitBoard,
    pub knights: BitBoard,
    pub pawns: BitBoard,
    pub pieces: BitBoard,
    pub check_mask: BitBoard,
    pub attacks: BitBoard,
    pub pins: Pins,
    pub can_castle_ks: bool,
    pub can_castle_qs: bool,
    pub king_must_move: bool,
}

impl PlayerState {
    pub fn blank() -> Self {
        Self {
            king: BitBoard::empty(),
            queens: BitBoard::empty(),
            rooks: BitBoard::empty(),
            bishops: BitBoard::empty(),
            knights: BitBoard::empty(),
            pawns: BitBoard::empty(),
            pieces: BitBoard::empty(),
            check_mask: BitBoard::full(),
            pins: Pins::new(),
            can_castle_ks: false,
            can_castle_qs: false,
            king_must_move: false,
            attacks: BitBoard::empty(),
        }
    }

    pub fn place_piece(&mut self, piece_kind: PieceKind, square: Square) {
        self.get_mut_piece_bitboard(piece_kind).turn_on(square);
        self.pieces.turn_on(square);
    }

    pub fn remove_piece(&mut self, piece_kind: PieceKind, square: Square) {
        self.get_mut_piece_bitboard(piece_kind).turn_off(square);
        self.pieces.turn_off(square);
    }

    pub fn move_piece(&mut self, piece_kind: PieceKind, origin: Square, target: Square) {
        self.get_mut_piece_bitboard(piece_kind)
            .move_bit(origin, target);
        self.pieces.move_bit(origin, target);
    }

    pub fn isnt_in_check(&self) -> bool {
        self.check_mask.is_full()
    }

    fn get_mut_piece_bitboard(&mut self, piece_kind: PieceKind) -> &mut BitBoard {
        match piece_kind {
            PieceKind::King => &mut self.king,
            PieceKind::Queen => &mut self.queens,
            PieceKind::Rook => &mut self.rooks,
            PieceKind::Bishop => &mut self.bishops,
            PieceKind::Knight => &mut self.knights,
            PieceKind::Pawn => &mut self.pawns,
        }
    }
}

#[derive(Clone, Copy)]
pub struct BoardPieces {
    pub pieces: [Option<Piece>; 64],
}

impl BoardPieces {
    pub fn empty() -> Self {
        Self { pieces: [None; 64] }
    }

    pub fn get_piece(&self, square: Square) -> &Option<Piece> {
        &self.pieces[square]
    }

    pub fn get_mut_piece(&mut self, square: Square) -> &mut Option<Piece> {
        &mut self.pieces[square]
    }

    pub fn remove_piece(&mut self, square: Square) {
        *self.get_mut_piece(square) = None;
    }

    pub fn move_piece(&mut self, origin: Square, target: Square) {
        self.pieces.swap(origin.0 as usize, target.0 as usize);
        self.pieces[origin] = None;
    }
}

#[derive(Clone, Copy)]
pub struct Board {
    pub active: PlayerState,
    pub inactive: PlayerState,
    pub current_player: Player,
    pub ep_capture_point: BitBoard,
    pub pieces: BoardPieces,
    pub half_moves: u32,
}

impl Default for Board {
    fn default() -> Self {
        let starting_position = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        Self::from_str(starting_position).unwrap()
    }
}

impl Board {
    // It is faster to update pins (which are caused by a sliding piece) and the check mask (effected by said sliding pieces) together.
    // For that reason, the term SCM (Sliding Check Mask) is used.
    // NOTICE: This function is assumed to be ran before "update_non_sliding_cm".
    pub fn update_pins_and_scm(&mut self) {
        // The en-passant position is based on the side that last moved. In other words, the side which is opposite to the current one.
        let ep_pawn = match self.current_player {
            Player::White => self.ep_capture_point.move_down(1),
            Player::Black => self.ep_capture_point.move_up(1),
        };
        let super_piece = self.active.king;
        let diagonal_attackers = self.inactive.queens | self.inactive.bishops;
        let cross_attackers = self.inactive.queens | self.inactive.rooks;

        // I ignore the En-Passant pawn since It's needed in some special checks (because the capture point of the En-Passant differs to the captured piece's position).
        let empty_squares = !self.inactive.pieces | ep_pawn;

        // This is a dirty macro in order to speed up the updates.
        // It returns from the top level function (this) once it has determined the king already has to move.
        macro_rules! update {
            ($ray:expr, $possible_casters:expr, $pin_mask:expr, $check_ep:literal) => {{
                let ray = $ray;

                if (ray & ($possible_casters)).is_empty() {
                    // No check or pin can be done!
                } else if (ray & ep_pawn).is_empty() {
                    // If we are here, that means no enemy piece blocks the ray (and that the ray actually exists).
                    let blocking = ray & self.active.pieces;

                    if blocking.is_single_1() {
                        // We found a pin!
                        $pin_mask |= ray
                    } else if blocking.is_empty() {
                        // We found a check!
                        if self.active.isnt_in_check() {
                            self.active.check_mask = ray;
                        } else {
                            // If the king already has to move, all other check/pin data is useless.
                            self.active.king_must_move = true;
                            return;
                        }
                    }
                } else if $check_ep { // If the ray is blocked by the en-passant pawn, it cannot be captured, as a pin would be broken (based on the "check_ep" flag).
                    self.ep_capture_point = BitBoard::empty();
                }
            }};
        }

        // Now, we use the update macro to update the pins and check mask.
        {
            update!(
                get_up_attacks(super_piece, empty_squares),
                cross_attackers,
                self.active.pins.vertical,
                false
            );

            update!(
                get_up_right_attacks(super_piece, empty_squares),
                diagonal_attackers,
                self.active.pins.diagonal,
                true
            );

            update!(
                get_right_attacks(super_piece, empty_squares),
                cross_attackers,
                self.active.pins.horizontal,
                true
            );

            update!(
                get_down_right_attacks(super_piece, empty_squares),
                diagonal_attackers,
                self.active.pins.anti_diagonal,
                true
            );

            update!(
                get_down_attacks(super_piece, empty_squares),
                cross_attackers,
                self.active.pins.vertical,
                true
            );

            update!(
                get_down_left_attacks(super_piece, empty_squares),
                diagonal_attackers,
                self.active.pins.diagonal,
                true
            );

            update!(
                get_left_attacks(super_piece, empty_squares),
                cross_attackers,
                self.active.pins.horizontal,
                true
            );

            update!(
                get_up_left_attacks(super_piece, empty_squares),
                diagonal_attackers,
                self.active.pins.anti_diagonal,
                true
            );
        }
    }

    pub fn update_non_sliding_cm(&mut self) {
        if self.active.king_must_move {
            return;
        }

        // The non-sliding attackers have to be either knights or pawns.
        let attackers = (KNIGHT_MOVES[self.active.king.first_one_square()] & self.inactive.knights)
            | (match self.current_player {
                // The movement is inverse to the current player, since we are attacked by the inactive one.
                Player::White => self.active.king.move_up_left() | self.active.king.move_up_right(),
                Player::Black => {
                    self.active.king.move_down_left() | self.active.king.move_down_right()
                }
            } & self.inactive.pawns);

        if attackers.is_single_1() {
            if self.active.isnt_in_check() {
                self.active.check_mask = attackers;
            } else {
                self.active.king_must_move = true;
            }
        } else if attackers.isnt_empty() {
            // If this is true, there must be two attackers or more.
            self.active.king_must_move = true;
        }
    }

    // This function will update the pins and check mask of the current side.
    // It will also compute the attacks against the current side.
    // All of these are needed to ensure moves generated later are indeed legal.
    pub fn update_move_constraints(&mut self) {
        self.active.king_must_move = false;
        self.active.pins = Pins::new();
        self.active.check_mask = BitBoard::full();

        AttackGen::run(self);
        self.update_pins_and_scm();
        self.update_non_sliding_cm();
    }

    pub fn switch_sides(&mut self) {
        mem::swap(&mut self.active, &mut self.inactive);

        self.current_player = !self.current_player;
    }

    pub fn make_move(&mut self, chess_move: Move) {
        match chess_move {
            Move::EnPassant { origin } => {
                let captured_square = self.ep_capture_point.first_one_square();
                let move_to = match self.current_player {
                    Player::White => captured_square.move_up(1),
                    Player::Black => captured_square.move_down(1),
                };

                self.active.move_piece(PieceKind::Pawn, origin, move_to);
                self.inactive.remove_piece(PieceKind::Pawn, captured_square);

                // We must keep the board pieces in-sync with the actual board representation.
                self.pieces.move_piece(origin, move_to);
                self.pieces.remove_piece(captured_square);

                self.ep_capture_point = BitBoard::empty(); // The en-passant square must be reset (or set again) after each move, since it has a single move timeframe.
            }
            Move::Regular {
                origin,
                target,
                piece_kind,
                double_push,
            } => {
                if piece_kind == PieceKind::King {
                    // If the king is moved, castling rights are revoked.
                    self.active.can_castle_ks = false;
                    self.active.can_castle_qs = false;
                }

                // If the moving squares are the starting positions of either rook, that rook must have either been captured or moved now, or previously.
                // In either case we can revoke castling rights.
                // NOTICE: Both pairs of rook positions must be accounted for here
                if LEFT_ROOK_ORIGINS.contains(&origin) || LEFT_ROOK_ORIGINS.contains(&target) {
                    self.active.can_castle_qs = false;
                } else if RIGHT_ROOK_ORIGINS.contains(&origin)
                    || RIGHT_ROOK_ORIGINS.contains(&target)
                {
                    self.active.can_castle_ks = false;
                }

                self.active.move_piece(piece_kind, origin, target);

                if let &Some(Piece { piece_kind, .. }) = self.pieces.get_piece(target) {
                    self.inactive.remove_piece(piece_kind, target)
                }

                self.pieces.move_piece(origin, target); // We must keep the board pieces in-sync with the actual board representation.

                // The en-passant square must be reset (or set again) after each move, since it has a single move timeframe.
                self.ep_capture_point = if double_push {
                    BitBoard::from(match self.current_player {
                        Player::White => target.move_down(1),
                        Player::Black => target.move_up(1),
                    })
                } else {
                    BitBoard::empty()
                };
            }
            Move::Promotion {
                origin,
                target,
                promotion_to,
            } => {
                self.active.remove_piece(PieceKind::Pawn, origin);
                self.active.place_piece(promotion_to, target);

                if let &Some(Piece { piece_kind, .. }) = self.pieces.get_piece(target) {
                    self.inactive.remove_piece(piece_kind, target)
                }

                // We must keep the board pieces in-sync with the actual board representation.
                self.pieces.move_piece(origin, target);
                self.pieces.get_mut_piece(target).unwrap().piece_kind = promotion_to;

                self.ep_capture_point = BitBoard::empty(); // The en-passant square must be reset (or set again) after each move, since it has a single move timeframe.
            }
            Move::CastleKS => {
                match self.current_player {
                    Player::White => {
                        let king_to = Square::G1;
                        let rook_to = Square::F1;

                        self.active.move_piece(PieceKind::King, Square::E1, king_to);
                        self.pieces.move_piece(Square::E1, king_to);

                        self.active.move_piece(PieceKind::King, Square::H1, rook_to);
                        self.pieces.move_piece(Square::H1, rook_to);
                    }
                    Player::Black => {
                        let king_to = Square::G8;
                        let rook_to = Square::F8;

                        self.active.move_piece(PieceKind::King, Square::E8, king_to);
                        self.pieces.move_piece(Square::E8, king_to);

                        self.active.move_piece(PieceKind::King, Square::H8, rook_to);
                        self.pieces.move_piece(Square::H8, rook_to);
                    }
                }

                self.active.can_castle_ks = false;
                self.ep_capture_point = BitBoard::empty(); // The en-passant square must be reset (or set again) after each move, since it has a single move timeframe.
            }
            Move::CastleQS => {
                match self.current_player {
                    Player::White => {
                        let king_to = Square::C1;
                        let rook_to = Square::D1;

                        self.active.move_piece(PieceKind::King, Square::E1, king_to);
                        self.pieces.move_piece(Square::E1, king_to);

                        self.active.move_piece(PieceKind::King, Square::A1, rook_to);
                        self.pieces.move_piece(Square::A1, rook_to);
                    }
                    Player::Black => {
                        let king_to = Square::C8;
                        let rook_to = Square::D8;

                        self.active.move_piece(PieceKind::King, Square::E8, king_to);
                        self.pieces.move_piece(Square::E8, king_to);

                        self.active.move_piece(PieceKind::King, Square::A8, rook_to);
                        self.pieces.move_piece(Square::A8, rook_to);
                    }
                }

                self.active.can_castle_qs = false;
                self.ep_capture_point = BitBoard::empty(); // The en-passant square must be reset (or set again) after each move, since it has a single move timeframe.
            }
        };

        // The move constraints are updated for the current player. The player about to play needs that data.
        self.switch_sides();
        self.update_move_constraints();
    }
}
