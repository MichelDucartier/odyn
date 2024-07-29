use odyn::game::utility;
use rand::{rngs::StdRng, RngCore, SeedableRng};

use crate::common::formatting::assert_eq_bitboard;

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
fn test_extract_bit_with_0() {
    let mut rng = StdRng::seed_from_u64(42);
    let r = rng.next_u64();
    let res = utility::extract_bit(r & (0 << 61), 61);
    assert_eq!(0, res);
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
