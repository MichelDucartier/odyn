use std::{
    collections::{HashMap, HashSet},
    fmt,
};

use ilog::IntLog;

use super::{
    bitboard::Bitboard,
    chess_move::{self, Move},
    mailbox::{self, MailboxBoard},
    utility,
};
use crate::{
    constants::{
        self, ALL_PIECES_ID, BISHOP_ID, BLACK_ID, EMPTY_ID, KING_ID, KNIGHT_ID, PAWN_ID,
        POSSIBLE_PROMOTION, QUEEN_ID, RANK_1_INDEX, ROOK_ID, WHITE_ID,
    },
    game::move_generator::{
        generate_bishop_moves, generate_rook_moves, generate_xray_bishop_attacks,
        generate_xray_rook_attacks,
    },
};
use crate::{
    constants::{NON_SLIDING_PIECES_ID, RANK_8_INDEX},
    game::bitboard,
};

/// High-level chess board composed of bitboard and mailbox representations.
#[derive(Default, Debug)]
pub struct Chessboard {
    bitboard: bitboard::Bitboard,
    mailbox: mailbox::MailboxBoard,

    white_moves: u32,
    black_moves: u32,
}

impl Clone for Chessboard {
    fn clone(&self) -> Self {
        let bitboard = self.bitboard;
        let mailbox = self.mailbox.clone();

        Self {
            bitboard,
            mailbox,
            white_moves: self.white_moves,
            black_moves: self.black_moves,
        }
    }
}

impl Chessboard {
    /// Builds a chessboard from a FEN string.
    pub fn from_fen(fen: &str, separator: &str) -> Chessboard {
        let fen_parts: Vec<&str> = fen.split(separator).collect();

        let [_s_board, _s_turn, _s_castle, _s_enpassant, s_bmoves, s_wmoves] = &fen_parts[..]
        else {
            panic!("Invalid fen, invalid number of parts")
        };

        Chessboard {
            bitboard: Bitboard::from_fen(fen, separator),
            mailbox: MailboxBoard::from_fen(fen, separator),
            white_moves: s_wmoves.parse().unwrap(),
            black_moves: s_bmoves.parse().unwrap(),
        }
    }

    /// Iterates over occupied squares, yielding `(index, (piece_id, color_id))`.
    pub fn get_iterator_on_pieces(&self) -> impl Iterator<Item = (u32, (u8, u8))> + '_ {
        let mut remaining = self.bitboard.white_board | self.bitboard.black_board;

        std::iter::from_fn(move || {
            if remaining == 0 {
                return None;
            }

            let idx = remaining.trailing_zeros();
            remaining &= remaining - 1;

            let piece = self.mailbox.get_piece(idx);
            Some((idx, piece))
        })
    }

    /// Returns the piece id and color id on a square.
    pub fn piece_at(&self, index: u32) -> (u8, u8) {
        self.mailbox.get_piece(index)
    }

    /// Serializes the board back to FEN.
    pub fn to_fen(&self, separator: &str) -> String {
        let mut bitboard_fen = self.bitboard.to_fen();
        let move_counts = format!("{} {}", self.black_moves, self.white_moves);
        bitboard_fen.push(move_counts);

        bitboard_fen.join(separator)
    }

    /// Applies a move without validating legality and returns packed move flags.
    pub fn make_move_unchecked(&mut self, move_: Move) -> u16 {
        let flags = self.mailbox.move_piece(&move_);
        self.bitboard.move_piece(&move_, flags);
        flags
    }

    /// Builds a board from `start_fen` after applying all `moves` in order.
    pub fn from_moves(start_fen: &str, moves: Vec<Move>) -> Chessboard {
        let mut cboard = Chessboard::from_fen(start_fen, " ");
        for move_ in moves {
            cboard.make_move_unchecked(move_);
        }
        cboard
    }

    /// Computes pseudo-legal moves for `color_id` (ignores king safety).
    pub fn legal_moves(&self, color_id: u8) -> HashSet<chess_move::Move> {
        let opponent_color = constants::opposite(color_id);
        let allied_board = self.bitboard.get_color_board(color_id);
        let opponent_board = self.bitboard.get_color_board(opponent_color);

        let allied_king_board = self.bitboard.king_board & allied_board;

        let king_index = allied_king_board.log2() as u32;

        // Compute checkers with their indices + attacks
        let checkers = utility::bit_scan(opponent_board)
            .iter()
            .filter_map(|idx| {
                let (piece_id, piece_color) = self.mailbox.get_piece(*idx);
                let attacks = self
                    .bitboard
                    .generate_attacks(piece_id, piece_color, 1_u64 << *idx);
                if (attacks & allied_king_board) == 0 {
                    return None;
                }

                Some((*idx, attacks))
            })
            .collect::<HashMap<u32, u64>>();

        println!("Number of checkers: {}", checkers.len());

        // Compute pinned pieces
        let pinned_pieces = self.compute_pinned_pieces(color_id);

        // Compute ennemy attacks
        let ennemy_attacks = self
            .bitboard
            .generate_pieces_attacks(opponent_color, ALL_PIECES_ID.to_vec());

        // King moves (keep only the moves that are not blocked by allied pieces + not attacked by
        // ennemies)
        let king_moves = self
            .bitboard
            .generate_moves(KING_ID, color_id, allied_king_board)
            & !allied_board
            & !ennemy_attacks;

        if checkers.len() >= 2 {
            // Double check, the only move is a king move
            return utility::unpack_moves(king_index, king_moves).collect();
        }

        let allied_attacks: HashSet<Move> = utility::bit_scan(allied_board)
            .iter()
            .flat_map(|start_idx| {
                let (piece_id, color_id) = self.mailbox.get_piece(*start_idx);
                let mut piece_attacks =
                    self.bitboard
                        .generate_attacks(piece_id, color_id, 1_u64 << *start_idx)
                        & opponent_board;

                // If the attacker is a king, need to check whether the potential captured piece
                // will cause the king to be in check
                if piece_id == KING_ID {
                    piece_attacks = piece_attacks & !ennemy_attacks;
                }

                utility::unpack_moves(*start_idx, piece_attacks)
                    .flat_map(move |move_| self.maybe_promotion_moves(move_, piece_id, color_id))
            })
            .collect();

        // We already generated the attacks, now we need to generate the moves
        let allied_moves = utility::bit_scan(allied_board)
            .iter()
            .flat_map(|start_index| {
                let (piece_id, color_id) = self.mailbox.get_piece(*start_index);

                let mut allowed_moves =
                    self.bitboard
                        .generate_moves(piece_id, color_id, 1_u64 << start_index);

                if piece_id == KING_ID {
                    allowed_moves = allowed_moves & !ennemy_attacks;
                }

                // 2 choices: pinned or not pinned
                if let Some(allow_mask) = pinned_pieces.get(start_index) {
                    // If pinned
                    allowed_moves = allowed_moves & allow_mask;
                }

                utility::unpack_moves(*start_index, allowed_moves)
                    .flat_map(move |move_| self.maybe_promotion_moves(move_, piece_id, color_id))
            })
            .collect();

        if checkers.len() >= 1 {
            let (checker_idx, checker_attack) = checkers.iter().next().unwrap();
            let moves = self.handle_single_check(
                *checker_idx,
                *checker_attack,
                king_index,
                king_moves,
                allied_attacks,
                allied_moves,
            );
            return moves;
        }

        let mut allowed_moves = allied_moves;
        allowed_moves.extend(allied_attacks);

        println!("Allowed moves: {:?}", allowed_moves);
        allowed_moves
    }

    fn maybe_promotion_moves(&self, move_: Move, piece_id: u8, color_id: u8) -> HashSet<Move> {
        if piece_id != PAWN_ID {
            return HashSet::from([move_]);
        }

        let (end_row, _end_col) = utility::index_to_square(move_.end_index);

        // It's a pawn, check if it's a promotion move
        if (end_row == RANK_8_INDEX && color_id == WHITE_ID)
            || (end_row == RANK_1_INDEX && color_id == BLACK_ID)
        {
            let mut possible_moves = HashSet::new();
            for promotion_piece in POSSIBLE_PROMOTION {
                possible_moves.insert(Move::new(
                    move_.start_index,
                    move_.end_index,
                    promotion_piece,
                ));
            }

            return possible_moves;
        }

        HashSet::from([move_])
    }

    fn handle_single_check(
        &self,
        checker_idx: u32,
        checker_attack: u64,
        king_index: u32,
        king_moves: u64,
        allied_attacks: HashSet<chess_move::Move>,
        allied_moves: HashSet<chess_move::Move>,
    ) -> HashSet<chess_move::Move> {
        let (piece_id, _color_id) = self.mailbox.get_piece(checker_idx);

        let capture_moves: HashSet<chess_move::Move> = allied_attacks
            .iter()
            .filter(|move_| move_.end_index == checker_idx)
            .copied()
            .collect();

        println!("Capture moves: {:?}", capture_moves);

        // King moves
        let king_moves_set: HashSet<chess_move::Move> =
            utility::unpack_moves(king_index, king_moves).collect();

        if NON_SLIDING_PIECES_ID.contains(&piece_id) {
            // A single knight or pawn is checking the king
            // Only moves available : capture or king moves
            let mut legal = king_moves_set;
            legal.extend(capture_moves);
            return legal;
        }

        // If the checking piece is a sliding piece, we have 3 choices:
        let checker_board = 1u64 << checker_idx;
        let king_board = 1u64 << king_index;

        // Squares between king and checker
        let ray_mask = Self::build_ray_mask(king_index, checker_idx);
        let block_squares = ray_mask & checker_attack & !checker_board & !king_board;

        let block_moves: HashSet<Move> = allied_moves
            .iter()
            .filter(|m| ((1u64 << m.end_index) & block_squares) != 0)
            .copied()
            .collect();

        let mut legal = king_moves_set;
        legal.extend(capture_moves);
        legal.extend(block_moves);
        legal
    }

    /// Compute the pinned pieces and return  a HashMap from index to allowed ray mask for the
    /// pinned piece
    fn compute_pinned_pieces(&self, color_id: u8) -> HashMap<u32, u64> {
        let allied_board = self.bitboard.get_color_board(color_id);
        let opponent_board = self.bitboard.get_color_board(constants::opposite(color_id));

        let occupancy = allied_board | opponent_board;

        let allied_king_board = self.bitboard.king_board & allied_board;
        if allied_king_board == 0 {
            return HashMap::new();
        }
        let king_index = allied_king_board.trailing_zeros();

        let opponent_rook_like =
            (self.bitboard.rook_board | self.bitboard.queen_board) & opponent_board;
        let opponent_bishop_like =
            (self.bitboard.bishop_board | self.bitboard.queen_board) & opponent_board;

        let rook_pinners =
            generate_xray_rook_attacks(occupancy, allied_board, king_index) & opponent_rook_like;
        let bishop_pinners = generate_xray_bishop_attacks(occupancy, allied_board, king_index)
            & opponent_bishop_like;

        let mut pinners = rook_pinners | bishop_pinners;
        let mut pinned_pieces = HashMap::new();

        while pinners != 0 {
            let pinner_index = pinners.trailing_zeros();
            pinners &= pinners - 1;

            let allowed_ray_mask = Self::build_ray_mask(king_index, pinner_index);
            // let occupied_on_ray = occupancy & allowed_ray_mask;
            let pinned_piece_board = allowed_ray_mask & allied_board & !allied_king_board;

            // If there is only one piece on the ray between the pinner and the king, then this
            // piece is pinned
            if pinned_piece_board.count_ones() == 1 {
                let pinned_piece_index = pinned_piece_board.trailing_zeros();
                pinned_pieces.insert(pinned_piece_index, allowed_ray_mask);
            }
        }

        pinned_pieces
    }

    fn build_ray_mask(start_index: u32, end_index: u32) -> u64 {
        let start_board = 1_u64 << start_index;
        let end_board = 1_u64 << end_index;
        let occupancy = start_board | end_board;

        let start_rook_rays = generate_rook_moves(start_board, occupancy);
        if (start_rook_rays & end_board) != 0 {
            let end_rook_rays = generate_rook_moves(end_board, occupancy);
            return start_rook_rays & end_rook_rays;
        }

        let start_bishop_rays = generate_bishop_moves(start_board, occupancy);
        if (start_bishop_rays & end_board) != 0 {
            let end_bishop_rays = generate_bishop_moves(end_board, occupancy);
            return start_bishop_rays & end_bishop_rays;
        }

        0
    }

    fn unpack_moves(start_index: u32, piece_moves: u64) -> impl Iterator<Item = chess_move::Move> {
        let mut remaining_moves = piece_moves;

        std::iter::from_fn(move || {
            if remaining_moves == 0 {
                return None;
            }
            let end_index = remaining_moves.trailing_zeros();
            remaining_moves &= !(1_u64 << end_index);
            Some(Move::new_no_promotion(start_index, end_index))
        })
    }

    /// Returns whether `color_id` is currently in check.
    pub fn is_in_check(&self, color_id: u8, opponent_attacks: u64) -> bool {
        self.bitboard.is_in_check(color_id, opponent_attacks)
    }

    /// Returns the color id of the side to move.
    pub fn current_turn(&self) -> u8 {
        self.bitboard.current_turn()
    }

    // Returns `true` when the side to move has no legal escape from check.
    // pub fn is_checkmate(&self, opponent_attacks: u64) -> bool {
    //     let current_color = (self.bitboard.flags >> bitboard::TURN_F_INDEX) & 0b1;
    //     self.is_in_check(current_color, opponent_attacks) && !self.exists_legal_moves(current_color)
    // }

    // Returns `true` when the side to move has no legal move and is not in check.
    // pub fn is_stalemate(&self, opponent_attacks: u64) -> bool {
    //     let current_color = (self.bitboard.flags >> bitboard::TURN_F_INDEX) & 0b1;
    //     !self.is_in_check(current_color, opponent_attacks) && self.exists_legal_moves(current_color)
    // }

    // pub fn legal_moves(&self, current_color: u8) -> impl Iterator<Item = chess_move::Move> + '_ {
    //     let opponent_id = constants::opposite(current_color);
    //     let pseudo_legal_moves = self.legal_moves(current_color);
    //     let opponent_attacks = self.bitboard.generate_all_attacks(opponent_id);
    //
    //     pseudo_legal_moves.filter(move |move_| {
    //         let mut cboard_copy = self.clone();
    //         let flags = cboard_copy.make_move_unchecked(*move_);
    //
    //         // The move that we look at is a castle move
    //         if chess_move::get_castle_flag(flags)
    //             && !self.bitboard.is_castle_in_check(*move_, opponent_attacks)
    //         {
    //             return true;
    //         }
    //
    //         // Not a castle move
    //         let new_opponent_attacks = cboard_copy.bitboard.generate_all_attacks(opponent_id);
    //         if !cboard_copy.is_in_check(current_color, new_opponent_attacks) {
    //             return true;
    //         }
    //
    //         false
    //     })
    // }
}

/// Returns the character representing a piece given its piece_id and color_id.
///
/// White pieces are uppercase, black pieces are lowercase.
/// Empty squares return `'.'`.
fn piece_to_char(piece_id: u8, color_id: u8) -> char {
    let c = match piece_id {
        PAWN_ID => 'p',
        KNIGHT_ID => 'n',
        BISHOP_ID => 'b',
        ROOK_ID => 'r',
        QUEEN_ID => 'q',
        KING_ID => 'k',
        EMPTY_ID => return '.',
        _ => '?',
    };
    if color_id == WHITE_ID {
        c.to_ascii_uppercase()
    } else {
        c
    }
}

impl fmt::Display for Chessboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  +---+---+---+---+---+---+---+---+")?;
        for row in 0..8u32 {
            let rank_label = 8 - row;
            write!(f, "{rank_label} ")?;
            for col in 0..8u32 {
                let index = utility::square_to_index(row, col);
                let (piece_id, color_id) = self.mailbox.get_piece(index);
                let ch = piece_to_char(piece_id, color_id);
                write!(f, "| {ch} ")?;
            }
            writeln!(f, "|")?;
            writeln!(f, "  +---+---+---+---+---+---+---+---+")?;
        }
        write!(f, "    a   b   c   d   e   f   g   h")?;
        Ok(())
    }
}

/// Returns a nicely formatted string representation of a chessboard.
///
/// White pieces are uppercase (P, N, B, R, Q, K), black pieces are lowercase
/// (p, n, b, r, q, k), and empty squares are shown as '.'.
///
/// # Example
/// ```
/// use odyn::game::chessboard::{Chessboard, format_chessboard};
/// use odyn::constants::START_FEN;
///
/// let board = Chessboard::from_fen(START_FEN, " ");
/// let display = format_chessboard(&board);
/// assert!(display.contains('K')); // white king
/// assert!(display.contains('p')); // black pawn
/// ```
pub fn format_chessboard(chessboard: &Chessboard) -> String {
    format!("{chessboard}")
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::game::chess_move;

    #[test]
    fn test_unpack_move() {
        let start_index = 0;
        let piece_moves = 0b1010;
        let moves: HashSet<chess_move::Move> =
            super::Chessboard::unpack_moves(start_index, piece_moves).collect();
        assert_eq!(moves.len(), 2);
        assert!(moves.contains(&super::Move::new_no_promotion(start_index, 1)));
        assert!(moves.contains(&super::Move::new_no_promotion(start_index, 3)));
    }

    #[test]
    fn test_display_start_position() {
        use crate::constants::START_FEN;

        let board = super::Chessboard::from_fen(START_FEN, " ");
        let display = format!("{board}");

        // Verify rank labels are present
        assert!(display.contains("8 "));
        assert!(display.contains("1 "));

        // Verify file labels are present
        assert!(display.contains("a   b   c   d   e   f   g   h"));

        // Verify all piece types appear
        assert!(display.contains('R')); // white rook
        assert!(display.contains('r')); // black rook
        assert!(display.contains('K')); // white king
        assert!(display.contains('k')); // black king
        assert!(display.contains('P')); // white pawn
        assert!(display.contains('p')); // black pawn

        // Verify empty squares
        assert!(display.contains('.'));
    }
}
