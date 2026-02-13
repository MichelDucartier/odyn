use std::cmp::{max, min};

use super::magic::{self, BISHOP_LOOKUP, ROOK_LOOKUP};
use crate::constants::{self, A_FILE_MASK, H_FILE_MASK, RANK_1_MASK, RANK_8_MASK};
use bit_reverse::ParallelReverse;

pub fn string_to_square(s: &str) -> Option<(u32, u32)> {
    // Check if the input string has exactly 2 characters
    if s.len() != 2 {
        return None;
    }

    // Extract the characters from the string
    let chars: Vec<char> = s.chars().collect();
    let first_char = chars[0];
    let second_char = chars[1];

    // Convert the first character to a zero-based index (assuming it is a letter)
    let alphabet_index = match first_char {
        'a'..='z' => (first_char as u32) - ('a' as u32),
        _ => return None, // Return None if the first character is not a letter
    };

    let digit_value = second_char.to_digit(10).map(|d| 8 - d);

    // Return None if the second character is not a digit
    digit_value.map(|d| (d, alphabet_index))
}

pub const fn square_to_index(row: u32, col: u32) -> u32 {
    (row << 3) + col
}

pub const fn index_to_square(index: u32) -> (u32, u32) {
    let row = index >> 3;
    let col = index & 0b111;
    (row, col)
}

pub fn square_to_string(row: u32, col: u32) -> String {
    let str_col = std::char::from_u32(col + ('a' as u32)).unwrap();
    format!("{}{}", str_col, 8 - row)
}

pub fn index_to_string(index: u32) -> String {
    let (row, col) = index_to_square(index);
    square_to_string(row, col)
}

pub fn extract_bit(bits: u64, index: u8) -> u64 {
    (bits >> index) & 0b1
}

pub fn west_one(bits: u64) -> u64 {
    (bits & !constants::A_FILE_MASK) >> 1
}

pub fn east_one(bits: u64) -> u64 {
    (bits & !constants::H_FILE_MASK) << 1
}

pub fn north_one(bits: u64) -> u64 {
    bits >> 8
}

pub fn south_one(bits: u64) -> u64 {
    bits << 8
}

pub fn mask_row_col(board: u64, row: i32, col: i32) -> u64 {
    board & (A_FILE_MASK >> row) & (RANK_1_MASK >> col)
}

pub fn mask_index(board: u64, index: u32) -> u64 {
    board & (1 << index)
}

pub fn remove_index(board: u64, index: u32) -> u64 {
    board & !(1 << index)
}

pub fn flip_diag_a8h1(board: u64) -> u64 {
    let mut x = board;

    const K1: u64 = 0x5500550055005500;
    const K2: u64 = 0x3333000033330000;
    const K4: u64 = 0x0f0f0f0f00000000;
    let t = K4 & (x ^ (x << 28));
    x ^= t ^ (t >> 28);
    let t = K2 & (x ^ (x << 14));
    x ^= t ^ (t >> 14);
    let t = K1 & (x ^ (x << 7));
    x ^= t ^ (t >> 7);

    x
}

pub fn flip_diag_a1h8(board: u64) -> u64 {
    let mut x = board;
    let mut t: u64 = x ^ (x << 36);

    let k1: u64 = 0xaa00aa00aa00aa00;
    let k2: u64 = 0xcccc0000cccc0000;
    let k4: u64 = 0xf0f0f0f00f0f0f0f;
    x ^= k4 & (t ^ (x >> 36));
    t = k2 & (x ^ (x << 18));
    x ^= t ^ (t >> 18);
    t = k1 & (x ^ (x << 9));
    x ^= t ^ (t >> 9);
    x
}

pub fn enumerate_subsets(board: u64) -> Vec<u64> {
    let mut res = Vec::new();

    let mut current: i64 = 0;
    let board: i64 = board.try_into().unwrap();

    loop {
        res.push(current.try_into().unwrap());
        current = (current - board) & board;

        if current == 0 {
            break;
        }
    }

    res
}

pub fn rook_rank_to_board(row_rank: u8, col_rank: u8, rook_index: u32) -> u64 {
    let col_rank: u64 = col_rank.into();
    let row_rank: u64 = row_rank.into();

    let (row, col) = index_to_square(rook_index);
    (flip_diag_a8h1(col_rank) << col) | (row_rank << (8 * row))
}

pub fn board_to_rook_ranks(board: u64, rook_index: u32) -> (u8, u8) {
    let (row, col) = index_to_square(rook_index);

    let row_rank: u8 = ((board >> (8 * row)) & 0xff).try_into().unwrap();
    let col_rank: u8 = (flip_diag_a8h1(board >> col) & 0xff).try_into().unwrap();

    (row_rank, col_rank)
}

pub fn relevant_rook_blocking(board: u64, rook_index: u32) -> u64 {
    let (row_rank, col_rank) = board_to_rook_ranks(board, rook_index);
    rook_rank_to_board(row_rank, col_rank, rook_index)
        & !A_FILE_MASK
        & !H_FILE_MASK
        & !RANK_1_MASK
        & !RANK_8_MASK
}

pub fn format_bitboard(bitboard: u64) -> String {
    let mut s = "".to_owned();

    for i in 0..64 {
        let bit = extract_bit(bitboard, i);

        if (i % 8) == 0 && i != 0 {
            s.push('\n');
        }

        s.push_str(&bit.to_string());
    }

    s
}

pub fn pseudo_rotate_45_clockwise(board: u64) -> u64 {
    let mut x = board;

    const K1: u64 = 0xAAAAAAAAAAAAAAAA;
    const K2: u64 = 0xCCCCCCCCCCCCCCCC;
    const K4: u64 = 0xF0F0F0F0F0F0F0F0;
    x ^= K1 & (x ^ x.rotate_right(8));
    x ^= K2 & (x ^ x.rotate_right(16));
    x ^= K4 & (x ^ x.rotate_right(32));

    x
}

pub fn pseudo_rotate_45_anticlockwise(board: u64) -> u64 {
    let mut x = board;

    const K1: u64 = 0x5555555555555555;
    const K2: u64 = 0x3333333333333333;
    const K4: u64 = 0x0f0f0f0f0f0f0f0f;
    x ^= K1 & (x ^ x.rotate_right(8));
    x ^= K2 & (x ^ x.rotate_right(16));
    x ^= K4 & (x ^ x.rotate_right(32));

    x
}

pub fn bishop_mask(bishop_index: u32) -> u64 {
    let magic = BISHOP_LOOKUP.magics[bishop_index as usize];
    let hash = magic::hash_board(0, magic, 13);

    BISHOP_LOOKUP.lookup[bishop_index as usize][hash]
}

// pub fn rook_mask(rook_index: u32) -> u64 {
//     let magic = ROOK_LOOKUP.magics[rook_index as usize];
//     let hash = magic::hash_board(0, magic, 13);
//
//     ROOK_LOOKUP.lookup[rook_index as usize][hash]
// }

pub fn get_indices_of_ones(board: u64) -> Vec<u32> {
    let mut indices = Vec::new();
    let mut board = board;
    while board != 0 {
        let trailing_zeros = board.trailing_zeros();
        indices.push(trailing_zeros);
        board &= !(1 << trailing_zeros);
    }
    indices
}

pub fn fill_between_indices(index_1: u32, index_2: u32) -> u64 {
    let min_index = min(index_1, index_2);
    let max_index = max(index_1, index_2);

    let max_ones = (1_u64 << (max_index + 1)) - 1;
    let min_ones = (1_u64 << min_index) - 1;

    max_ones ^ min_ones
}
