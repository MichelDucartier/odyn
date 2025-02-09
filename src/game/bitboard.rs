use smallvec::{smallvec, SmallVec};

use super::{
    chess_move::{self, Move},
    utility,
};
use crate::constants::{
    BISHOP_ID, BLACK_ID, EMPTY_ID, FILE_A_INDEX, FILE_D_INDEX, FILE_F_INDEX, FILE_G_INDEX,
    FILE_H_INDEX, KING_ID, KNIGHT_ID, PAWN_ID, QUEEN_ID, RANK_4_INDEX, RANK_5_INDEX, ROOK_ID,
    WHITE_ID,
};
use std::{collections::HashMap, panic};

#[derive(Debug, Default)]
pub struct Bitboard {
    pub white_board: u64,
    pub black_board: u64,
    pub pawn_board: u64,
    pub knight_board: u64,
    pub bishop_board: u64,
    pub rook_board: u64,
    pub queen_board: u64,
    pub king_board: u64,
    pub en_passant: u8,

    // | - | - | - | turn | K | Q | k | q
    pub flags: u8,
}

pub const TURN_F_INDEX: u8 = 4;
pub const WKCASTLE_F_INDEX: u8 = 3;
pub const WQCASTLE_F_INDEX: u8 = 2;
pub const BKCASTLE_F_INDEX: u8 = 1;
pub const BQCASTLE_F_INDEX: u8 = 0;

impl Bitboard {
    pub fn from_fen(fen: &str, separator: &str) -> Bitboard {
        let fen_parts: Vec<&str> = fen.split(separator).collect();

        let [s_board, s_turn, s_castle, s_enpassant, _s_bmoves, _s_wmoves] = &fen_parts[..] else {
            panic!("Invalid fen, invalid number of parts")
        };

        // Load board
        let mut bitboard = Bitboard::default();
        bitboard.load_bitboard(s_board);

        // Load turn
        bitboard.flags |= (if *s_turn == "w" { WHITE_ID } else { BLACK_ID }) << TURN_F_INDEX;

        // Load castle rights
        bitboard.load_castle_rights(s_castle);

        // Load en_passant
        if let Some((_row, col)) = utility::string_to_square(s_enpassant) {
            bitboard.en_passant |= 1 << col;
        }

        bitboard
    }

    // pub fn get_board(&self, key: u8) -> Option<u64> {
    //     match key {
    //         PAWN_ID => Some(self.pawn_board),
    //         KNIGHT_ID => Some(self.knight_board),
    //         BISHOP_ID => Some(self.bishop_board),
    //         ROOK_ID => Some(self.rook_board),
    //         QUEEN_ID => Some(self.queen_board),
    //         KING_ID => Some(self.king_board),
    //         EMPTY_ID => None,
    //         _ => panic!("Invalid key for get_board"),
    //     }
    // }

    // pub fn set_board(&mut self, key: u8, value: u64) {
    //     match key {
    //         PAWN_ID => self.pawn_board = value,
    //         KNIGHT_ID => self.knight_board = value,
    //         BISHOP_ID => self.bishop_board = value,
    //         ROOK_ID => self.rook_board = value,
    //         QUEEN_ID => self.queen_board = value,
    //         KING_ID => self.king_board = value,
    //         EMPTY_ID => (),
    //         _ => panic!("Invalid key for set_board"),
    //     }
    // }

    fn remove_piece_from_board(&mut self, piece_id: u8, color_id: u8, index: u32) {
        // Remove piece from board
        match piece_id {
            PAWN_ID => self.pawn_board &= !(1 << index),
            KNIGHT_ID => self.knight_board &= !(1 << index),
            BISHOP_ID => self.bishop_board &= !(1 << index),
            ROOK_ID => self.rook_board &= !(1 << index),
            QUEEN_ID => self.queen_board &= !(1 << index),
            KING_ID => self.king_board &= !(1 << index),
            _ => panic!("Invalid piece_id for remove_piece_from_board"),
        }

        match color_id {
            BLACK_ID => self.black_board &= !(1 << index),
            WHITE_ID => self.white_board &= !(1 << index),
            _ => panic!("Invalid color_id for remove_piece_from_board"),
        }
    }

    fn add_piece_to_board(&mut self, piece_id: u8, color_id: u8, index: u32) {
        // Add piece to board
        match piece_id {
            PAWN_ID => self.pawn_board |= 1 << index,
            KNIGHT_ID => self.knight_board |= 1 << index,
            BISHOP_ID => self.bishop_board |= 1 << index,
            ROOK_ID => self.rook_board |= 1 << index,
            QUEEN_ID => self.queen_board |= 1 << index,
            KING_ID => self.king_board |= 1 << index,
            _ => panic!("Invalid piece_id for add_piece_to_board"),
        }
        match color_id {
            BLACK_ID => self.black_board |= 1 << index,
            WHITE_ID => self.white_board |= 1 << index,
            _ => panic!("Invalid color_id for add_piece_to_board"),
        }
    }

    pub fn to_fen(&self) -> SmallVec<[String; 4]> {
        let board_fen = self.board_to_fen();
        let castle_fen = self.castle_to_fen();
        let turn_fen = self.turn_to_fen();
        let en_passant_fen = self.en_passant_to_fen();

        smallvec![board_fen, turn_fen, castle_fen, en_passant_fen]
    }

    fn turn_to_fen(&self) -> String {
        let turn = (self.flags >> 4) & 0b1;

        if turn == WHITE_ID {
            return "w".to_string();
        }

        "b".to_string()
    }

    fn en_passant_to_fen(&self) -> String {
        if self.en_passant == 0 {
            return "-".to_string();
        }

        let current_turn: u8 = (self.flags >> 4) & 0b1;
        let col = self.en_passant.trailing_zeros();

        // If current turn is white then it's black's pawn that can be taken in en passant
        let row = if current_turn == WHITE_ID { 2 } else { 5 };

        utility::square_to_string(row, col)
    }

    fn castle_to_fen(&self) -> String {
        let char_to_index: smallvec::SmallVec<[(char, u8); 4]> = smallvec![
            ('K', WKCASTLE_F_INDEX),
            ('Q', WQCASTLE_F_INDEX),
            ('k', BKCASTLE_F_INDEX),
            ('q', BQCASTLE_F_INDEX),
        ];

        let mut s = "".to_owned();

        for (char, index) in char_to_index.iter() {
            if utility::extract_bit(self.flags.into(), *index) == 1 {
                s.push(*char);
            }
        }

        // Handle case when no castle rights
        if s.is_empty() {
            return "-".to_string();
        }

        s
    }

    fn board_to_fen(&self) -> String {
        let mut s: String = "".to_owned();

        let char_to_board: smallvec::SmallVec<[(char, &u64); 6]> = smallvec::smallvec![
            ('p', &self.pawn_board),
            ('n', &self.knight_board),
            ('b', &self.bishop_board),
            ('r', &self.rook_board),
            ('q', &self.queen_board),
            ('k', &self.king_board),
        ];

        let mut blank = 0;

        'outer: for i in 0..64 {
            if i % 8 == 0 && i != 0 {
                if blank != 0 {
                    s.push_str(&blank.to_string());
                }
                s.push('/');

                blank = 0;
            }

            // Add piece to the final string if needed
            for (char, board) in &char_to_board {
                if utility::extract_bit(**board, i) == 1 {
                    if blank != 0 {
                        s.push_str(&blank.to_string());
                        blank = 0;
                    }

                    if utility::extract_bit(self.white_board, i) == 1 {
                        let upper_case = (*char).to_ascii_uppercase();
                        s.push(upper_case);
                        continue 'outer;
                    }

                    s.push(*char);
                    continue 'outer;
                }
            }

            blank += 1;
        }

        s
    }

    fn load_bitboard(&mut self, s_board: &str) {
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

            self.load_piece(c, board_index);
            board_index += 1;
        }
    }

    fn load_piece(&mut self, c: char, board_index: u32) {
        let mut char_to_board = HashMap::from([
            ('p', &mut self.pawn_board),
            ('n', &mut self.knight_board),
            ('b', &mut self.bishop_board),
            ('r', &mut self.rook_board),
            ('q', &mut self.queen_board),
            ('k', &mut self.king_board),
        ]);

        // Load type of piece
        match char_to_board.get_mut(&c.to_ascii_lowercase()) {
            Some(x) => **x |= 1 << board_index,
            None => panic!("Invalid fen for character {c}"),
        }

        // Load color
        if c.is_uppercase() {
            self.white_board |= 1 << board_index;
        } else {
            self.black_board |= 1 << board_index
        }
    }

    fn load_castle_rights(&mut self, s_castle: &str) {
        let char_to_index = HashMap::from([
            ('k', BKCASTLE_F_INDEX),
            ('q', BQCASTLE_F_INDEX),
            ('K', WKCASTLE_F_INDEX),
            ('Q', WQCASTLE_F_INDEX),
        ]);

        for c in s_castle.chars() {
            if let Some(x) = char_to_index.get(&c) {
                self.flags |= 1 << x;
            }
        }
    }

    pub fn move_piece(&mut self, move_: &Move, flags: u16) {
        let is_enpassant = chess_move::get_en_passant_flag(flags);
        let is_castle = chess_move::get_castle_flag(flags);

        let piece_id = chess_move::get_piece_flag(flags);
        let color_id = chess_move::get_color_flag(flags);

        let opposite_color = !chess_move::get_color_flag(flags);
        let captured_piece_id = chess_move::get_captured_piece_flag(flags);

        if !is_enpassant && !is_castle {
            // Remove piece from destination position + add it to the destination
            self.remove_piece_from_board(piece_id, color_id, move_.start_index);
            self.add_piece_to_board(piece_id, color_id, move_.end_index);

            // Remove captured piece if any
            if captured_piece_id != EMPTY_ID {
                self.remove_piece_from_board(captured_piece_id, opposite_color, move_.end_index);
            }
            return;
        }

        if is_enpassant {
            self.en_passant_move(move_, color_id);
            return;
        }

        if is_castle {
            self.castle_move(move_, color_id);
        }
    }

    fn en_passant_move(&mut self, move_: &Move, color_id: u8) {
        let (_end_row, end_col) = utility::index_to_square(move_.end_index);
        let captured_row = if color_id == WHITE_ID {
            RANK_5_INDEX
        } else {
            RANK_4_INDEX
        };

        let captured_index = utility::square_to_index(captured_row, end_col);

        // Remove captured pawn
        self.remove_piece_from_board(PAWN_ID, !color_id, captured_index);

        // Move pawn
        self.add_piece_to_board(PAWN_ID, color_id, move_.end_index);
        self.remove_piece_from_board(PAWN_ID, color_id, move_.start_index);
    }

    fn castle_move(&mut self, move_: &Move, color_id: u8) {
        let (row, end_col) = utility::index_to_square(move_.end_index);

        let (rook_start_col, rook_end_col) = if end_col == FILE_G_INDEX {
            (FILE_H_INDEX, FILE_F_INDEX)
        } else {
            (FILE_A_INDEX, FILE_D_INDEX)
        };

        // Move king
        self.remove_piece_from_board(KING_ID, color_id, move_.start_index);
        self.add_piece_to_board(KING_ID, color_id, move_.end_index);

        // Move rook
        let rook_start_index = utility::square_to_index(row, rook_start_col);
        let rook_end_index = utility::square_to_index(row, rook_end_col);
        self.remove_piece_from_board(ROOK_ID, color_id, rook_start_index);
        self.add_piece_to_board(ROOK_ID, color_id, rook_end_index);
    }
}
