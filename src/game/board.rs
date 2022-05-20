use crate::{
    generators::{
        slides::{
            get_down_attacks, get_down_left_attacks, get_down_right_attacks, get_left_attacks,
            get_right_attacks, get_up_attacks, get_up_left_attacks, get_up_right_attacks,
        },
        AttackGen, Move, Square,
    },
    tables::KNIGHT_MOVES,
    BitBoard, Piece, PieceKind, Pins, Player, BLACK_LEFT_ROOK_ORIGIN, BLACK_RIGHT_ROOK_ORIGIN,
    WHITE_LEFT_ROOK_ORIGIN, WHITE_RIGHT_ROOK_ORIGIN,
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
pub struct EnPassant {
    pub capture_point: BitBoard,
    pub pawn: BitBoard,
}

impl EnPassant {
    pub fn new() -> Self {
        Self {
            capture_point: BitBoard::empty(),
            pawn: BitBoard::empty(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Board {
    pub moving_player: PlayerState,
    pub moved_player: PlayerState,
    pub current_player: Player,
    pub ep_info: EnPassant,
    pub pieces: BoardPieces,
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
        let ep_attackers = self.ep_info.pawn.move_left() | self.ep_info.pawn.move_right();

        let super_piece = self.moving_player.king;
        let diagonal_attackers = self.moved_player.queens | self.moved_player.bishops;
        let cross_attackers = self.moved_player.queens | self.moved_player.rooks;

        // I ignore the en-passantable pawn since It's needed in some special checks
        // (because the capture point of the En-Passant differs to the captured piece's position).
        let empty_squares = !self.moved_player.pieces | self.ep_info.pawn;

        // NOTICE: "assets/pin_update_logic.svg" is an SVG graph to explain all the logic behind this.
        macro_rules! update {
            ($ray:expr, $possible_casters:expr, $pin_mask:expr, $is_horizontal:literal, $is_non_vertical:literal) => {{
                let ray = $ray;

                // The ray stops when it finds an "enemy" piece, thus this check is sufficient (mostly).
                if (ray & $possible_casters).isnt_empty() {
                    let does_contain_ep_pawn = (ray & self.ep_info.pawn).isnt_empty();

                    let is_single_blocker = (ray & self.moving_player.pieces).is_single_1();
                    let is_empty_of_blockers = (ray & self.moving_player.pieces).is_empty();

                    let ep_horizontal_condition = $is_horizontal && is_single_blocker && (ray & ep_attackers).isnt_empty();
                    let ep_non_vertical_condition = $is_non_vertical && is_empty_of_blockers;

                    if does_contain_ep_pawn {
                        if (ep_horizontal_condition || ep_non_vertical_condition) {
                        self.ep_info = EnPassant::new();
                        }
                    } else { // "does_contain_ep_pawn" needs to be false if we go here.
                        if is_single_blocker {
                            $pin_mask = $ray;
                        } else if is_empty_of_blockers {
                            if self.moving_player.isnt_in_check() {
                                self.moving_player.check_mask = $ray;
                            } else {
                                self.moving_player.king_must_move = true;
                                return; // I early return from the top level function as after the king must move, pin and check data become irrelevant.
                            }
                        }
                    }
                }
            }};
        }

        // Now, we use the update macro to update the pins and check mask.
        {
            update!(
                get_up_attacks(super_piece, empty_squares),
                cross_attackers,
                self.moving_player.pins.vertical,
                false,
                false
            );

            update!(
                get_up_right_attacks(super_piece, empty_squares),
                diagonal_attackers,
                self.moving_player.pins.diagonal,
                false,
                true
            );

            update!(
                get_right_attacks(super_piece, empty_squares),
                cross_attackers,
                self.moving_player.pins.horizontal,
                true,
                true
            );

            update!(
                get_down_right_attacks(super_piece, empty_squares),
                diagonal_attackers,
                self.moving_player.pins.anti_diagonal,
                false,
                true
            );

            update!(
                get_down_attacks(super_piece, empty_squares),
                cross_attackers,
                self.moving_player.pins.vertical,
                false,
                false
            );

            update!(
                get_down_left_attacks(super_piece, empty_squares),
                diagonal_attackers,
                self.moving_player.pins.diagonal,
                false,
                true
            );

            update!(
                get_left_attacks(super_piece, empty_squares),
                cross_attackers,
                self.moving_player.pins.horizontal,
                true,
                true
            );

            update!(
                get_up_left_attacks(super_piece, empty_squares),
                diagonal_attackers,
                self.moving_player.pins.anti_diagonal,
                false,
                true
            );
        }
    }

    pub fn update_non_sliding_cm(&mut self) {
        if self.moving_player.king_must_move {
            return;
        }

        // The non-sliding attackers have to be either knights or pawns.
        let attackers = (KNIGHT_MOVES[self.moving_player.king.first_one_square()]
            & self.moved_player.knights)
            | (match self.current_player {
                // The movement is inverse to the current player, since we are attacked by the inactive one.
                Player::White => {
                    self.moving_player.king.move_up_left() | self.moving_player.king.move_up_right()
                }
                Player::Black => {
                    self.moving_player.king.move_down_left()
                        | self.moving_player.king.move_down_right()
                }
            } & self.moved_player.pawns);

        if attackers.is_single_1() {
            if self.moving_player.isnt_in_check() {
                self.moving_player.check_mask = attackers;
            } else {
                self.moving_player.king_must_move = true;
            }
        } else if attackers.isnt_empty() {
            // If this is true, there must be two attackers or more.
            self.moving_player.king_must_move = true;
        }
    }

    // This function will update the pins and check mask of the current side.
    // It will also compute the attacks against the current side.
    // All of these are needed to ensure moves generated later are indeed legal.
    pub fn update_move_constraints(&mut self) {
        self.moving_player.king_must_move = false;
        self.moving_player.pins = Pins::new();
        self.moving_player.check_mask = BitBoard::full();

        AttackGen::run(self);
        self.update_pins_and_scm();
        self.update_non_sliding_cm();
    }

    pub fn switch_sides(&mut self) {
        mem::swap(&mut self.moving_player, &mut self.moved_player);

        self.current_player = !self.current_player;
    }

    pub fn make_move(&mut self, chess_move: Move) {
        let (moving_lro, moving_rro, moved_lro, moved_rro) = match self.current_player {
            Player::White => (
                WHITE_LEFT_ROOK_ORIGIN,
                WHITE_RIGHT_ROOK_ORIGIN,
                BLACK_LEFT_ROOK_ORIGIN,
                BLACK_RIGHT_ROOK_ORIGIN,
            ),
            Player::Black => (
                BLACK_LEFT_ROOK_ORIGIN,
                BLACK_RIGHT_ROOK_ORIGIN,
                WHITE_LEFT_ROOK_ORIGIN,
                WHITE_RIGHT_ROOK_ORIGIN,
            ),
        };

        match chess_move {
            Move::EnPassant { origin } => {
                let move_to = self.ep_info.capture_point.first_one_square();
                let captured_square = match self.current_player {
                    Player::White => move_to.move_down(1),
                    Player::Black => move_to.move_up(1),
                };

                self.moving_player
                    .move_piece(PieceKind::Pawn, origin, move_to);
                self.moved_player
                    .remove_piece(PieceKind::Pawn, captured_square);

                // We must keep the board pieces in-sync with the actual board representation.
                self.pieces.move_piece(origin, move_to);
                self.pieces.remove_piece(captured_square);

                self.ep_info = EnPassant::new(); // The en-passant square must be reset (or set again) after each move, since it has a single move timeframe.
            }
            Move::Regular {
                origin,
                target,
                piece_kind,
                double_push,
            } => {
                if piece_kind == PieceKind::King {
                    // If the king is moved, castling rights are revoked.
                    self.moving_player.can_castle_ks = false;
                    self.moving_player.can_castle_qs = false;
                }

                // We must have moved a rook!
                if moving_lro == origin {
                    self.moving_player.can_castle_qs = false;
                } else if moving_rro == origin {
                    // We must have moved a rook!
                    self.moving_player.can_castle_ks = false;
                } else if moved_lro == target {
                    // We must have captured a rook!
                    self.moved_player.can_castle_qs = false;
                } else if moved_rro == target {
                    // We must have captured a rook!
                    self.moved_player.can_castle_ks = false;
                }

                self.moving_player.move_piece(piece_kind, origin, target);

                if let &Some(Piece { piece_kind, .. }) = self.pieces.get_piece(target) {
                    self.moved_player.remove_piece(piece_kind, target)
                }

                self.pieces.move_piece(origin, target); // We must keep the board pieces in-sync with the actual board representation.

                self.ep_info = if double_push {
                    let target = BitBoard::from(target);

                    EnPassant {
                        capture_point: match self.current_player {
                            Player::White => target.move_down(1),
                            Player::Black => target.move_up(1),
                        },
                        pawn: target,
                    }
                } else {
                    EnPassant::new()
                };
            }
            Move::Promotion {
                origin,
                target,
                promotion_to,
            } => {
                // Promotion moves can only capture rooks, not move them.
                if moved_lro == target {
                    // We must have captured a rook!
                    self.moved_player.can_castle_qs = false;
                } else if moved_rro == target {
                    // We must have captured a rook!
                    self.moved_player.can_castle_ks = false;
                }

                self.moving_player.remove_piece(PieceKind::Pawn, origin);
                self.moving_player.place_piece(promotion_to, target);

                if let &Some(Piece { piece_kind, .. }) = self.pieces.get_piece(target) {
                    self.moved_player.remove_piece(piece_kind, target)
                }

                // We must keep the board pieces in-sync with the actual board representation.
                self.pieces.move_piece(origin, target);
                self.pieces
                    .get_mut_piece(target)
                    .as_mut()
                    .unwrap()
                    .piece_kind = promotion_to;

                self.ep_info = EnPassant::new(); // The en-passant square must be reset (or set again) after each move, since it has a single move timeframe.
            }
            Move::CastleKS => {
                match self.current_player {
                    Player::White => {
                        let king_to = Square::G1;
                        let rook_to = Square::F1;

                        self.moving_player
                            .move_piece(PieceKind::King, Square::E1, king_to);
                        self.pieces.move_piece(Square::E1, king_to);

                        self.moving_player
                            .move_piece(PieceKind::Rook, Square::H1, rook_to);
                        self.pieces.move_piece(Square::H1, rook_to);
                    }
                    Player::Black => {
                        let king_to = Square::G8;
                        let rook_to = Square::F8;

                        self.moving_player
                            .move_piece(PieceKind::King, Square::E8, king_to);
                        self.pieces.move_piece(Square::E8, king_to);

                        self.moving_player
                            .move_piece(PieceKind::Rook, Square::H8, rook_to);
                        self.pieces.move_piece(Square::H8, rook_to);
                    }
                }

                // Once a player castles, he loses the right to do so again, on either side.
                self.moving_player.can_castle_ks = false;
                self.moving_player.can_castle_qs = false;

                self.ep_info = EnPassant::new(); // The en-passant square must be reset (or set again) after each move, since it has a single move timeframe.
            }
            Move::CastleQS => {
                match self.current_player {
                    Player::White => {
                        let king_to = Square::C1;
                        let rook_to = Square::D1;

                        self.moving_player
                            .move_piece(PieceKind::King, Square::E1, king_to);
                        self.pieces.move_piece(Square::E1, king_to);

                        self.moving_player
                            .move_piece(PieceKind::Rook, Square::A1, rook_to);
                        self.pieces.move_piece(Square::A1, rook_to);
                    }
                    Player::Black => {
                        let king_to = Square::C8;
                        let rook_to = Square::D8;

                        self.moving_player
                            .move_piece(PieceKind::King, Square::E8, king_to);
                        self.pieces.move_piece(Square::E8, king_to);

                        self.moving_player
                            .move_piece(PieceKind::Rook, Square::A8, rook_to);
                        self.pieces.move_piece(Square::A8, rook_to);
                    }
                }
                // Once a player castles, he loses the right to do so again, on either side.
                self.moving_player.can_castle_ks = false;
                self.moving_player.can_castle_qs = false;

                self.ep_info = EnPassant::new(); // The en-passant square must be reset (or set again) after each move, since it has a single move timeframe.
            }
        };

        // NOTICE: The move constraints are updated for the now moving player only.
        // Previously, there was a need to update it for the moved player, since the attacks generated here used the pin data for that player,
        // which could have changed during this move, but now that is no longer the case, as that data is useless there.
        self.switch_sides();
        self.update_move_constraints();
    }
}
