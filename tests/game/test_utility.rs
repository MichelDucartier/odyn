use std::collections::HashSet;

use odyn::assert_eq_bitboard;
use odyn::{
    assert_eq_u8,
    game::utility::{self, board_to_rook_ranks},
};

#[test]
fn test_string_to_square_h4() {
    let res = utility::string_to_square("h4");
    assert_eq!(Some((4, 7)), res);
}

#[test]
fn test_square_to_string_h4() {
    let res = utility::square_to_string(4, 7);
    assert_eq!("h4", res);
}

#[test]
fn test_square_to_index_f3() {
    let res = utility::square_to_index(3, 5);
    assert_eq!(29, res);
}

#[test]
fn test_index_to_square_b1() {
    let res = utility::index_to_square(57);
    assert_eq!((7, 1), res);
}

#[test]
fn test_extract_bit_with_1() {
    let res = utility::extract_bit(1 << 42, 42);
    assert_eq!(1, res);
}

#[test]
fn test_west_one_shifts_west() {
    let bitboard = (1 << 18) | (1 << 43);
    let res = utility::west_one(bitboard);
    let expected = (1 << 17) | (1 << 42);
    assert_eq_bitboard!(expected, res);
}

#[test]
fn test_west_one_no_overflow() {
    let bitboard = 1 << 8;
    let res = utility::west_one(bitboard);
    assert_eq_bitboard!(0, res);

    let bitboard = 1;
    let res = utility::west_one(bitboard);
    assert_eq_bitboard!(0, res);
}

#[test]
fn test_east_one_shifts_east() {
    let bitboard = (1 << 18) | (1 << 43);
    let res = utility::east_one(bitboard);
    let expected = (1 << 19) | (1 << 44);
    assert_eq_bitboard!(expected, res);
}

#[test]
fn test_east_one_no_overflow() {
    let bitboard = 1 << 39;
    let res = utility::east_one(bitboard);
    assert_eq_bitboard!(0, res);

    let bitboard = 1 << 7;
    let res = utility::east_one(bitboard);
    assert_eq_bitboard!(0, res);
}

#[test]
fn test_south_one_shifts_south() {
    let bitboard = (1 << 18) | (1 << 43);
    let res = utility::south_one(bitboard);
    let expected = (1 << 26) | (1 << 51);
    assert_eq_bitboard!(expected, res);
}

#[test]
fn test_south_one_no_overflow() {
    let bitboard = 1 << 59;
    let res = utility::south_one(bitboard);
    assert_eq_bitboard!(0, res);

    let bitboard = 1 << 63;
    let res = utility::south_one(bitboard);
    assert_eq_bitboard!(0, res);
}

#[test]
fn test_north_one_shifts_north() {
    let bitboard = (1 << 18) | (1 << 43);
    let res = utility::north_one(bitboard);
    let expected = (1 << 10) | (1 << 35);
    assert_eq_bitboard!(expected, res);
}

#[test]
fn test_north_one_no_overflow() {
    let bitboard = 1;
    let res = utility::north_one(bitboard);
    assert_eq_bitboard!(0, res);

    let bitboard = 1 << 5;
    let res = utility::north_one(bitboard);
    assert_eq_bitboard!(0, res);
}

#[test]
fn test_flip_diag_a8h1() {
    let board: u64 = 0b11110000;
    let expected: u64 = 0x0101010100000000;

    assert_eq_bitboard!(expected, utility::flip_diag_a8h1(board))
}

#[test]
fn test_enumerate_subsets() {
    let board: u64 = 0b00001101;
    let expected: Vec<u64> = vec![
        0b00001101, 0b00001000, 0b00001100, 0b00001001, 0b00000101, 0b00000001, 0b00000100,
        0b00000000,
    ];

    let expected: HashSet<u64> = HashSet::from_iter(expected.iter().cloned());
    let result: HashSet<u64> =
        HashSet::from_iter(utility::enumerate_subsets(board).iter().cloned());

    assert_eq!(expected, result);
}

#[test]
fn test_rook_rank_to_board() {
    let row_rank = 0b00011001;
    let col_rank = 0b11010010;
    let rook_index = 35;

    let result = utility::rook_rank_to_board(row_rank, col_rank, rook_index);
    let expected = 0b0000100000001000000000000001100100000000000000000000100000000000;

    assert_eq_bitboard!(expected, result);
}

#[test]
fn test_board_to_rook_ranks() {
    let rook_index = 45;
    let board = 0b0010000000000000101010110010000000000000001000000000000000100000;

    let col_expected = 0b10110101;
    let row_expected = 0b10101011;

    let (row, col) = board_to_rook_ranks(board, rook_index);

    assert_eq_u8!(row_expected, row);
    assert_eq_u8!(col_expected, col);
}
