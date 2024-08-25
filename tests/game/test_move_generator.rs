use odyn::assert_eq_bitboard;
use odyn::game::bitboard::Bitboard;
use odyn::game::move_generator;

#[test]
fn test_king_moves() {
    let start_fen = "8/8/8/8/3K4/8/8/8 w - - 0 1";
    let bitboard = Bitboard::from_fen(start_fen, " ");

    let expected_moves =
        1 << 26 | 1 << 27 | 1 << 28 | 1 << 34 | 1 << 36 | 1 << 42 | 1 << 43 | 1 << 44;

    odyn::assert_eq_bitboard!(
        expected_moves,
        move_generator::generate_king_moves(bitboard.king_board) & !bitboard.king_board
    );
}

#[test]
fn test_king_moves_in_corner() {
    let start_fen = "8/8/8/8/8/8/8/k7 w - - 0 1";
    let bitboard = Bitboard::from_fen(start_fen, " ");

    let expected_moves = 1 << 48 | 1 << 49 | 1 << 57;

    assert_eq_bitboard!(
        expected_moves,
        move_generator::generate_king_moves(bitboard.king_board) & !bitboard.king_board
    );
}

#[test]
fn test_knight_moves() {
    let start_fen = "8/8/3N4/8/8/8/8/8 w - - 0 1";
    let bitboard = Bitboard::from_fen(start_fen, " ");

    let expected_moves = 1 << 2 | 1 << 4 | 1 << 13 | 1 << 29 | 1 << 36 | 1 << 34 | 1 << 25 | 1 << 9;

    assert_eq_bitboard!(
        expected_moves,
        move_generator::generate_knight_moves(bitboard.knight_board) & !bitboard.knight_board
    );
}

#[test]
fn test_knight_moves_in_corner() {
    let start_fen = "8/8/8/8/8/8/8/1N6 w - - 0 1";
    let bitboard = Bitboard::from_fen(start_fen, " ");

    let expected_moves = 1 << 40 | 1 << 42 | 1 << 51;

    assert_eq_bitboard!(
        expected_moves,
        move_generator::generate_knight_moves(bitboard.knight_board) & !bitboard.knight_board
    );
}

#[test]
fn test_rook_moves_empty_board() {
    let start_fen = "8/8/3R4/8/5r2/8/8/8 w - - 0 1";
    let bitboard = Bitboard::from_fen(start_fen, " ");

    let expected_moves = 0b0010100000101000001010001101111100101000111101110010100000101000;

    assert_eq_bitboard!(
        expected_moves,
        move_generator::generate_rook_moves(
            bitboard.rook_board,
            bitboard.white_board | bitboard.black_board
        ) & !bitboard.rook_board
    )
}

#[test]
fn test_rook_moves_single_rook() {
    let start_fen = "8/8/8/2R5/8/8/8/8 w - - 0 1";
    let bitboard = Bitboard::from_fen(start_fen, " ");

    let expected_moves = 0b0000010000000100000001000000010011111011000001000000010000000100;
    assert_eq_bitboard!(
        expected_moves,
        move_generator::generate_rook_moves(
            bitboard.rook_board,
            bitboard.white_board | bitboard.black_board
        ) & !bitboard.rook_board
    )
}

#[test]
fn test_rook_moves_with_blocking() {
    let start_fen = "3N4/8/3R2r1/8/8/8/3n4/8 w - - 0 1";
    let bitboard = Bitboard::from_fen(start_fen, " ");

    let expected_moves = 0b0100000001001000010010000100100001001000111111110100100001001000;

    assert_eq_bitboard!(
        expected_moves & !bitboard.rook_board,
        move_generator::generate_rook_moves(
            bitboard.rook_board,
            bitboard.white_board | bitboard.black_board
        ) & !bitboard.rook_board
    );
}

#[test]
fn test_bishop_moves_single_bishop() {
    let start_fen = "8/8/8/8/8/2B5/8/8 w - - 0 1";
    let bitboard = Bitboard::from_fen(start_fen, " ");

    let expected_moves = 0b0001000100001010000001000000101000010001001000000100000010000000;

    assert_eq_bitboard!(
        expected_moves & !bitboard.bishop_board,
        move_generator::generate_bishop_moves(
            bitboard.bishop_board,
            bitboard.white_board | bitboard.black_board
        ) & !bitboard.bishop_board
    )
}

#[test]
fn test_bishop_moves_with_blocking() {
    let start_fen = "7b/8/8/5P2/8/2n2B2/6K1/8 w - - 0 1";
    let bitboard = Bitboard::from_fen(start_fen, " ");

    let expected_moves = 0b0000100001010000001001000101100010011000001001000100001010000001;

    assert_eq_bitboard!(
        expected_moves & !bitboard.bishop_board,
        move_generator::generate_bishop_moves(
            bitboard.bishop_board,
            bitboard.white_board | bitboard.black_board
        ) & !bitboard.bishop_board
    )
}
