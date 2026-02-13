use odyn::{
    assert_eq_bitboard,
    constants::{START_FEN, WHITE_ID},
    game::{bitboard::Bitboard, move_generator},
};

fn rook_attacks_naive(rook_index: u32, occupancy: u64) -> u64 {
    let row = (rook_index / 8) as i32;
    let col = (rook_index % 8) as i32;
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut attacks = 0;

    for (dr, dc) in directions {
        let mut r = row + dr;
        let mut c = col + dc;

        while (0..8).contains(&r) && (0..8).contains(&c) {
            let index = (r * 8 + c) as u32;
            attacks |= 1u64 << index;

            if (occupancy & (1u64 << index)) != 0 {
                break;
            }

            r += dr;
            c += dc;
        }
    }

    attacks
}

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
fn test_black_rook_moves_with_occupancy() {
    let start_fen = "8/8/3p4/8/1P1r1n2/8/3B4/8 b - - 0 1";
    let bitboard = Bitboard::from_fen(start_fen, " ");

    let rook_board = bitboard.rook_board & bitboard.black_board;
    let occupancy = bitboard.white_board | bitboard.black_board;
    let expected_moves = (1u64 << 19)
        | (1u64 << 27)
        | (1u64 << 33)
        | (1u64 << 34)
        | (1u64 << 36)
        | (1u64 << 37)
        | (1u64 << 43)
        | (1u64 << 51);
    let generated_moves = move_generator::generate_rook_moves(rook_board, occupancy) & !rook_board;

    assert_eq_bitboard!(expected_moves, generated_moves);

    // Defensive check: compare against a naive ray-walk generator from d4 (index 35).
    assert_eq_bitboard!(rook_attacks_naive(35, occupancy), generated_moves);
}

#[test]
fn test_black_rook_moves_in_a8_corner_with_occupancy() {
    let start_fen = "r1B5/8/p7/8/8/8/8/8 b - - 0 1";
    let bitboard = Bitboard::from_fen(start_fen, " ");

    let rook_board = bitboard.rook_board & bitboard.black_board;
    let occupancy = bitboard.white_board | bitboard.black_board;
    let expected_moves = (1u64 << 1) | (1u64 << 2) | (1u64 << 8) | (1u64 << 16);
    let generated_moves = move_generator::generate_rook_moves(rook_board, occupancy) & !rook_board;

    assert_eq_bitboard!(expected_moves, generated_moves);

    assert_eq_bitboard!(rook_attacks_naive(0, occupancy), generated_moves);
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

#[test]
fn test_pawn_moves_on_starting_square() {
    let bitboard = Bitboard::from_fen(START_FEN, " ");
    let expected_moves = 0b0000000000000000111111111111111100000000000000000000000000000000;

    let pawn_board = bitboard.pawn_board & bitboard.white_board;
    let occupancy = bitboard.white_board | bitboard.black_board;
    let color = WHITE_ID;

    assert_eq_bitboard!(
        expected_moves,
        move_generator::generate_pawn_moves(pawn_board, occupancy, color)
    )
}

#[test]
fn test_pawn_attacks_on_a_file() {
    let bitboard = Bitboard::from_fen(
        "rnbqkbnr/ppppppp1/7p/8/P7/8/1PPPPPPP/RNBQKBNR w KQkq - 0 2",
        " ",
    );
    let expected_attacks = 0b0000000000000000111111110000000000000010000000000000000000000000;

    let pawn_board = bitboard.pawn_board & bitboard.white_board;
    let color = WHITE_ID;

    assert_eq_bitboard!(
        expected_attacks,
        move_generator::generate_pawn_attacks(pawn_board, color, 0)
    )
}
