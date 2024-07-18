use smallvec::{smallvec, SmallVec};

use super::utility;
use crate::constants::{BLACK_VALUE, WHITE_VALUE};
use core::panic;
use std::{ascii::AsciiExt, collections::HashMap};

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
        bitboard.flags |= (if *s_turn == "w" {
            WHITE_VALUE
        } else {
            BLACK_VALUE
        }) << TURN_F_INDEX;

        // Load castle rights
        bitboard.load_castle_rights(s_castle);

        // Load en_passant
        if let Some((row, col)) = utility::string_to_square(s_enpassant) {
            let board_index = utility::square_to_index(row, col);
            bitboard.en_passant |= 1 << board_index;
        }

        return bitboard;
    }

    pub fn to_fen(&mut self) -> SmallVec<[String; 4]> {
        let board_fen = self.board_to_fen();
        let castle_fen = self.castle_to_fen();
        let turn_fen = self.turn_to_fen();
        let en_passant_fen = self.en_passant_to_fen();

        return smallvec![board_fen, turn_fen, castle_fen, en_passant_fen];
    }

    fn turn_to_fen(&mut self) -> String {
        let turn = (self.flags >> 4) & 0b1;

        if turn == WHITE_VALUE {
            return "w".to_string();
        }

        return "b".to_string();
    }

    fn en_passant_to_fen(&mut self) -> String {
        if self.en_passant == 0 {
            return "-".to_string();
        }

        let current_turn: u8 = (self.en_passant >> 4) & 0b1;
        let col = self.en_passant.trailing_zeros();

        let row = if current_turn == WHITE_VALUE { 5 } else { 2 };

        return utility::square_to_string(row, col);
    }

    fn castle_to_fen(&mut self) -> String {
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

        return s;
    }

    fn board_to_fen(&mut self) -> String {
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

        for i in 0..64 {
            if i % 8 == 0 && i != 0 {
                if blank != 0 {
                    s.push_str(&blank.to_string());
                }
                s.push_str("/");

                blank = 0;
                continue;
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
                        continue;
                    }

                    s.push(*char);
                    continue;
                }
            }

            blank += 1;
        }

        return s;
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
}
