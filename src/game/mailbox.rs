use super::chess_move;
use super::{chess_move::Move, utility};
use crate::constants::{
    BISHOP_ID, BLACK_ID, EMPTY_ID, FILE_A_INDEX, FILE_D_INDEX, FILE_F_INDEX, FILE_G_INDEX,
    FILE_H_INDEX, KING_ID, KNIGHT_ID, PAWN_ID, QUEEN_ID, ROOK_ID, WHITE_ID,
};
use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct MailboxBoard {
    board: [u8; 64],
}

impl Default for MailboxBoard {
    fn default() -> Self {
        Self { board: [0; 64] }
    }
}

lazy_static! {
    static ref PIECE_ID_MAP: HashMap<char, u8> = [
        ('p', PAWN_ID),
        ('n', KNIGHT_ID),
        ('b', BISHOP_ID),
        ('r', ROOK_ID),
        ('q', QUEEN_ID),
        ('k', KING_ID)
    ]
    .iter()
    .copied()
    .collect();
}

fn get_piece(c: char) -> u8 {
    let color_id = if c.is_uppercase() { WHITE_ID } else { BLACK_ID };
    piece_id(
        *PIECE_ID_MAP.get(&c.to_ascii_lowercase()).unwrap(),
        color_id,
    )
}

fn piece_id(piece_id: u8, color_id: u8) -> u8 {
    color_id << 3 | piece_id
}

impl MailboxBoard {
    pub fn from_fen(fen: &str, separator: &str) -> MailboxBoard {
        let fen_parts: Vec<&str> = fen.split(separator).collect();

        let mut board = [0; 64];

        let [s_board, _, _, _, _, _] = &fen_parts[..] else {
            panic!("Invalid fen, invalid number of parts")
        };

        let mut board_index = 0;

        for c in s_board.chars() {
            if c == '/' {
                continue;
            }

            if c.is_numeric() {
                board_index += match c.to_digit(10) {
                    Some(x) => x,
                    None => panic!("Invalid fen, parsing character {c} into a digit"),
                };
                continue;
            }

            let piece_id = get_piece(c);
            board[board_index as usize] = piece_id;

            board_index += 1;
        }

        MailboxBoard { board }
    }

    pub fn get_piece(&self, index: u32) -> (u8, u8) {
        let square = self.board[index as usize];
        let piece_id = square & 0b111;
        let color_id = square >> 3;

        (piece_id, color_id)
    }

    /// Move a piece (in place operation). This function does not check the validity of the `move_`
    /// Returns a `u8` flag that identify the type of the move
    ///
    /// MSB
    /// 1 bit      | 1 bit  | 1 bit            | 3 bits           |
    /// -----------------------------------------------------------
    /// en-passant | castle | color identifier | piece identifier |
    pub fn move_piece(&mut self, move_: &Move) -> u16 {
        let (start_piece, color_id) = self.get_piece(move_.start_index);
        let (_start_row, start_col) = utility::index_to_square(move_.start_index);
        let (_end_row, end_col) = utility::index_to_square(move_.end_index);

        // Flag to return
        let mut flags: u16 = 0;
        flags |= (start_piece as u16) << chess_move::PIECE_INDEX;
        flags |= (color_id as u16) << chess_move::COLOR_INDEX;
        flags |= (self.get_piece(move_.end_index).0 as u16) << chess_move::CAPTURED_PIECE_INDEX;

        // Castle move
        if start_piece == KING_ID && move_.start_index.abs_diff(move_.end_index) == 2 {
            self.castle_move(move_);
            flags |= 1 << chess_move::CASTLE_INDEX; // set castle flag
            return flags;
        }

        // En-passant
        if start_piece == PAWN_ID
            && start_col.abs_diff(end_col) == 1
            && self.board[move_.end_index as usize] == EMPTY_ID
        {
            self.en_passant_move(move_);
            flags |= 1 << chess_move::EN_PASSANT_INDEX; // set en-passant flag
            return flags;
        }

        self.board[move_.end_index as usize] = self.board[move_.start_index as usize];
        self.board[move_.start_index as usize] = EMPTY_ID;

        flags
    }

    fn en_passant_move(&mut self, move_: &Move) {
        let (end_row, end_col) = utility::index_to_square(move_.end_index);

        let captured_row = if end_row == 2 { 3 } else { 4 };

        let captured_index = utility::square_to_index(captured_row, end_col);

        // Move allied pawn and capture enemy pawn
        self.board[move_.end_index as usize] = self.board[move_.start_index as usize];
        self.board[move_.start_index as usize] = EMPTY_ID;

        self.board[captured_index as usize] = EMPTY_ID;
    }

    fn castle_move(&mut self, move_: &Move) {
        let (king_row, end_col) = utility::index_to_square(move_.end_index);

        // Compute rook positions
        let (rook_start_col, rook_end_col) = if end_col == FILE_G_INDEX {
            (FILE_H_INDEX, FILE_F_INDEX)
        } else {
            (FILE_A_INDEX, FILE_D_INDEX)
        };

        let start_rook_index = utility::square_to_index(king_row, rook_start_col) as usize;
        let end_rook_index = utility::square_to_index(king_row, rook_end_col) as usize;

        // Move king and rook
        self.board[end_rook_index] = self.board[start_rook_index];
        self.board[start_rook_index] = EMPTY_ID;

        self.board[move_.end_index as usize] = self.board[move_.start_index as usize];
        self.board[move_.start_index as usize] = EMPTY_ID;
    }
}
