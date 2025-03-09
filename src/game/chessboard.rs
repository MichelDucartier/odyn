use std::collections::HashSet;

use super::{
    bitboard::Bitboard,
    chess_move::{self, Move},
    mailbox::{self, MailboxBoard},
    utility,
};
use crate::{
    constants::{self, START_FEN},
    game::bitboard,
};

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

    pub fn from_moves(moves: Vec<Move>) -> Chessboard {
        let mut cboard = Chessboard::from_fen(START_FEN, " ");
        for move_ in moves {
            cboard.make_move_unchecked(move_);
        }
        cboard
    }

    pub fn compute_legal_moves(&self) -> HashSet<Move> {
        let current_color = (self.bitboard.flags >> bitboard::TURN_F_INDEX) & 0b1;
        let pseudo_legal_moves = self.pseudo_legal_moves(current_color);

        let mut legal_moves = HashSet::new();
        for move_ in pseudo_legal_moves {
            let mut cboard_copy = self.clone();
            cboard_copy.make_move_unchecked(move_);
            if !cboard_copy.is_in_check(current_color) {
                legal_moves.insert(move_);
            }
        }

        legal_moves
    }

    pub fn pseudo_legal_moves(&self, color_id: u8) -> HashSet<Move> {
        let mut legal_moves = HashSet::new();

        let allied_board = self.bitboard.get_color_board(color_id);
        let allied_indices = utility::get_indices_of_ones(allied_board);

        for start_index in allied_indices {
            let (piece_id, _color_id) = self.mailbox.get_piece(start_index);
            let piece_moves =
                self.bitboard
                    .generate_legal_moves(piece_id, color_id, 1_u64 << start_index);
            let moves = Self::unpack_moves(start_index, piece_moves);
            legal_moves.extend(moves);
        }

        legal_moves
    }

    fn unpack_moves(start_index: u32, piece_moves: u64) -> HashSet<Move> {
        let mut moves = HashSet::new();

        let mut remaining_moves = piece_moves;

        while remaining_moves != 0 {
            let end_index = remaining_moves.trailing_zeros();
            remaining_moves &= !(1_u64 << end_index);
            moves.insert(Move::new_no_promotion(start_index, end_index));
        }

        moves
    }

    pub fn is_in_check(&self, color_id: u8) -> bool {
        self.bitboard.is_in_check(color_id)
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_unpack_move() {
        let start_index = 0;
        let piece_moves = 0b1010;
        let moves = super::Chessboard::unpack_moves(start_index, piece_moves);
        assert_eq!(moves.len(), 2);
        assert!(moves.contains(&super::Move::new_no_promotion(start_index, 1)));
        assert!(moves.contains(&super::Move::new_no_promotion(start_index, 3)));
    }
}
