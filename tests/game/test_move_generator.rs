use crate::common::formatting::assert_eq_bitboard;
use odyn::game::bitboard::Bitboard;

#[test]
fn test_king_moves() {
    let start_fen = "8/8/8/8/3K4/8/8/8 w - - 0 1";
    let bitboard = Bitboard::from_fen(start_fen, " ");

    let expected_moves =
        1 << 26 | 1 << 27 | 1 << 28 | 1 << 34 | 1 << 36 | 1 << 42 | 1 << 43 | 1 << 44;

    assert_eq_bitboard!(
        expected_moves,
        bitboard.generate_king_moves() & !bitboard.king_board
    );
}

#[test]
fn test_king_moves_in_corner() {
    let start_fen = "8/8/8/8/8/8/8/k7 w - - 0 1";
    let bitboard = Bitboard::from_fen(start_fen, " ");

    let expected_moves = 1 << 48 | 1 << 49 | 1 << 57;

    assert_eq_bitboard!(
        expected_moves,
        bitboard.generate_king_moves() & !bitboard.king_board
    );
}

#[test]
fn test_knight_moves() {
    let start_fen = "8/8/3N4/8/8/8/8/8 w - - 0 1";
    let bitboard = Bitboard::from_fen(start_fen, " ");

    let expected_moves = 1 << 2 | 1 << 4 | 1 << 13 | 1 << 29 | 1 << 36 | 1 << 34 | 1 << 25 | 1 << 9;

    assert_eq_bitboard!(
        expected_moves,
        bitboard.generate_knight_moves() & !bitboard.knight_board
    );
}

#[test]
fn test_knight_moves_in_corner() {
    let start_fen = "8/8/8/8/8/8/8/1N6 w - - 0 1";
    let bitboard = Bitboard::from_fen(start_fen, " ");

    let expected_moves = 1 << 40 | 1 << 42 | 1 << 51;

    assert_eq_bitboard!(
        expected_moves,
        bitboard.generate_knight_moves() & !bitboard.knight_board
    );
}
