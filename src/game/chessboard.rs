use std::fmt;

use super::{
    bitboard::Bitboard,
    chess_move::{self, Move},
    mailbox::{self, MailboxBoard},
    utility,
};
use crate::constants::{
    BISHOP_ID, EMPTY_ID, KING_ID, KNIGHT_ID, PAWN_ID, QUEEN_ID, ROOK_ID, WHITE_ID,
};
use crate::game::bitboard;

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

    pub fn to_fen(&self, separator: &str) -> String {
        let mut bitboard_fen = self.bitboard.to_fen();
        let move_counts = format!("{} {}", self.black_moves, self.white_moves);
        bitboard_fen.push(move_counts);

        bitboard_fen.join(separator)
    }

    pub fn make_move_unchecked(&mut self, move_: Move) -> u16 {
        let flags = self.mailbox.move_piece(&move_);
        self.bitboard.move_piece(&move_, flags);
        flags
    }

    pub fn from_moves(start_fen: &str, moves: Vec<Move>) -> Chessboard {
        let mut cboard = Chessboard::from_fen(start_fen, " ");
        for move_ in moves {
            cboard.make_move_unchecked(move_);
        }
        cboard
    }

    pub fn compute_legal_moves(&self) -> impl Iterator<Item = chess_move::Move> + '_ {
        let current_color = (self.bitboard.flags >> bitboard::TURN_F_INDEX) & 0b1;
        let pseudo_legal_moves = self.pseudo_legal_moves(current_color);

        pseudo_legal_moves.filter(move |move_| {
            let mut cboard_copy = self.clone();
            cboard_copy.make_move_unchecked(*move_);
            !cboard_copy.is_in_check(current_color)
        })
    }

    pub fn pseudo_legal_moves(&self, color_id: u8) -> impl Iterator<Item = chess_move::Move> + '_ {
        let allied_board = self.bitboard.get_color_board(color_id);

        utility::get_indices_of_ones(allied_board)
            .into_iter()
            .flat_map(move |start_index| {
                let (piece_id, _color_id) = self.mailbox.get_piece(start_index);
                let piece_moves =
                    self.bitboard
                        .generate_legal_moves(piece_id, color_id, 1_u64 << start_index);
                Self::unpack_moves(start_index, piece_moves)
            })
    }

    fn unpack_moves(start_index: u32, piece_moves: u64) -> impl Iterator<Item = chess_move::Move> {
        // let mut moves = HashSet::new();

        let mut remaining_moves = piece_moves;

        // while remaining_moves != 0 {
        //     let end_index = remaining_moves.trailing_zeros();
        //     remaining_moves &= !(1_u64 << end_index);
        //     moves.insert(Move::new_no_promotion(start_index, end_index));
        // }

        std::iter::from_fn(move || {
            if remaining_moves == 0 {
                return None;
            }
            let end_index = remaining_moves.trailing_zeros();
            remaining_moves &= !(1_u64 << end_index);
            Some(Move::new_no_promotion(start_index, end_index))
        })
    }

    pub fn is_in_check(&self, color_id: u8) -> bool {
        self.bitboard.is_in_check(color_id)
    }

    pub fn current_turn(&self) -> u8 {
        self.bitboard.current_turn()
    }

    pub fn is_checkmate(&self) -> bool {
        let current_color = (self.bitboard.flags >> bitboard::TURN_F_INDEX) & 0b1;
        self.is_in_check(current_color) && !self.exists_legal_moves(current_color)
    }

    pub fn is_stalemate(&self) -> bool {
        let current_color = (self.bitboard.flags >> bitboard::TURN_F_INDEX) & 0b1;
        !self.is_in_check(current_color) && self.exists_legal_moves(current_color)
    }

    pub fn is_castle_in_check(&self, move_: Move, color_id: u8) -> bool {
        self.bitboard.is_castle_in_check(move_, color_id)
    }

    fn exists_legal_moves(&self, current_color: u8) -> bool {
        let pseudo_legal_moves = self.pseudo_legal_moves(current_color);

        for move_ in pseudo_legal_moves {
            let mut cboard_copy = self.clone();
            let flags = cboard_copy.make_move_unchecked(move_);

            // The move that we look at is a castle move
            if chess_move::get_castle_flag(flags)
                && !self.bitboard.is_castle_in_check(move_, current_color)
            {
                return true;
            }

            // Not a castle move
            if !cboard_copy.is_in_check(current_color) {
                return true;
            }
        }

        false
    }
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
