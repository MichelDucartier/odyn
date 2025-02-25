use odyn::assert_eq_u8;
use odyn::constants::{
    BISHOP_ID, BLACK_ID, KING_ID, KNIGHT_ID, PAWN_ID, QUEEN_ID, ROOK_ID, WHITE_ID,
};
use odyn::game::chess_move::{self};
use odyn::game::mailbox::MailboxBoard;
use odyn::{constants::START_FEN, game::mailbox};
use std::collections::HashMap;

fn fen_to_map(fen: &str) -> HashMap<u32, (u8, u8)> {
    let mut board_map = HashMap::new();
    let piece_map = [
        ('p', (PAWN_ID, BLACK_ID)),
        ('n', (KNIGHT_ID, BLACK_ID)),
        ('b', (BISHOP_ID, BLACK_ID)),
        ('r', (ROOK_ID, BLACK_ID)),
        ('q', (QUEEN_ID, BLACK_ID)),
        ('k', (KING_ID, BLACK_ID)),
        ('P', (PAWN_ID, WHITE_ID)),
        ('N', (KNIGHT_ID, WHITE_ID)),
        ('B', (BISHOP_ID, WHITE_ID)),
        ('R', (ROOK_ID, WHITE_ID)),
        ('Q', (QUEEN_ID, WHITE_ID)),
        ('K', (KING_ID, WHITE_ID)),
    ];
    let piece_map: HashMap<char, (u8, u8)> = piece_map.into_iter().collect();

    let rows: Vec<&str> = fen.split_whitespace().next().unwrap().split('/').collect();
    for (rank, row) in rows.iter().enumerate() {
        let mut file = 0;
        for ch in row.chars() {
            if ch.is_ascii_digit() {
                file += ch.to_digit(10).unwrap();
            } else if let Some(&(piece_id, color_id)) = piece_map.get(&ch) {
                let index = ((rank * 8) as u32) + file;
                board_map.insert(index, (piece_id, color_id));
                file += 1;
            }
        }
    }

    board_map
}

fn test_mailbox_equal_fen(fen: &str, mboard: MailboxBoard) {
    let board_map = fen_to_map(fen);
    for (index, (expected_piece_id, expected_color_id)) in board_map {
        let (piece_id, color_id) = mboard.get_piece(index);

        assert_eq!(
            expected_piece_id, piece_id,
            "Testing piece ID equal for index {}",
            index
        );
        assert_eq!(
            expected_color_id, color_id,
            "Testing color ID equal for index {}",
            index
        );
    }
}

#[test]
fn test_to_from_identity_start_fen() {
    let mboard = mailbox::MailboxBoard::from_fen(START_FEN, " ");
    test_mailbox_equal_fen(START_FEN, mboard)
}

#[test]
fn test_to_from_random_fen() {
    const FEN1: &str = "2b5/p2NBp1p/1bp1nPPr/3P4/2pRnr1P/1k1B1Ppp/1P1P1pQP/Rq1N3K b - - 0 1";
    let mboard = mailbox::MailboxBoard::from_fen(FEN1, " ");
    test_mailbox_equal_fen(FEN1, mboard)
}

#[test]
fn test_mailbox_correct_after_pawn_move() {
    let mut mboard = mailbox::MailboxBoard::from_fen(START_FEN, " ");

    // Make move
    let move_ = chess_move::Move::new_no_promotion(51, 35);
    let flags = mboard.move_piece(&move_);

    const FEN_AFTER_MOVE: &str = "rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 1";
    test_mailbox_equal_fen(FEN_AFTER_MOVE, mboard);

    const EXPECTED_FLAGS: u8 =
        (PAWN_ID << chess_move::PIECE_INDEX) | (WHITE_ID << chess_move::COLOR_INDEX);
    assert_eq!(EXPECTED_FLAGS as u16, flags);
}

#[test]
fn test_mailbox_correct_after_black_move() {
    const FEN: &str = "rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 1";
    let mut mboard = mailbox::MailboxBoard::from_fen(FEN, " ");

    // Make move
    let move_ = chess_move::Move::new_no_promotion(12, 28);
    let flags = mboard.move_piece(&move_);

    const FEN_AFTER_MOVE: &str = "rnbqkbnr/pppp1ppp/8/4p3/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 1";
    test_mailbox_equal_fen(FEN_AFTER_MOVE, mboard);

    const EXPECTED_FLAGS: u8 =
        (PAWN_ID << chess_move::PIECE_INDEX) | (BLACK_ID << chess_move::COLOR_INDEX);
    assert_eq!(EXPECTED_FLAGS as u16, flags);
}

#[test]
fn test_mailbox_correct_after_bishop_capture() {
    const FEN: &str = "rnbqkbnr/pppp2pp/5p2/4p3/2B1P3/8/PPPP1PPP/RNBQK1NR w KQkq - 0 1";
    let mut mboard = mailbox::MailboxBoard::from_fen(FEN, " ");

    // Make move
    let move_ = chess_move::Move::new_no_promotion(34, 6);
    let flags = mboard.move_piece(&move_);

    const FEN_AFTER_MOVE: &str = "rnbqkbBr/pppp2pp/5p2/4p3/4P3/8/PPPP1PPP/RNBQK1NR w KQkq - 0 1";
    test_mailbox_equal_fen(FEN_AFTER_MOVE, mboard);

    const EXPECTED_FLAGS: u8 = (BISHOP_ID << chess_move::PIECE_INDEX)
        | (WHITE_ID << chess_move::COLOR_INDEX)
        | (KNIGHT_ID << chess_move::CAPTURED_PIECE_INDEX);

    assert_eq_u8!(EXPECTED_FLAGS as u16, flags);
}

#[test]
fn test_mailbox_correct_after_white_short_castle() {
    const FEN: &str = "r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 1";
    let mut mboard = mailbox::MailboxBoard::from_fen(FEN, " ");
    let move_ = chess_move::Move::new_no_promotion(60, 62);

    let flags = mboard.move_piece(&move_);

    const FEN_AFTER_MOVE: &str =
        "r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/5N2/PPPP1PPP/RNBQ1RK1 w Qkq - 0 1";
    test_mailbox_equal_fen(FEN_AFTER_MOVE, mboard);

    const EXPECTED_FLAGS: u8 = (KING_ID << chess_move::PIECE_INDEX)
        | (WHITE_ID << chess_move::COLOR_INDEX)
        | (1 << chess_move::CASTLE_INDEX);

    assert_eq_u8!(EXPECTED_FLAGS as u16, flags);
}

#[test]
fn test_mailbox_correct_after_black_short_castle() {
    const FEN: &str = "r1bqk2r/pppp1ppp/2n2n2/2b1p3/2B1P3/2N2N2/PPPP1PPP/R1BQ1RK1 w HQkq - 0 1";
    let mut mboard = mailbox::MailboxBoard::from_fen(FEN, " ");
    let move_ = chess_move::Move::new_no_promotion(4, 6);

    let flags = mboard.move_piece(&move_);

    const FEN_AFTER_MOVE: &str =
        "r1bq1rk1/pppp1ppp/2n2n2/2b1p3/2B1P3/2N2N2/PPPP1PPP/R1BQ1RK1 w Qq - 0 1";
    test_mailbox_equal_fen(FEN_AFTER_MOVE, mboard);

    const EXPECTED_FLAGS: u8 = (KING_ID << chess_move::PIECE_INDEX)
        | (BLACK_ID << chess_move::COLOR_INDEX)
        | (1 << chess_move::CASTLE_INDEX);

    assert_eq_u8!(EXPECTED_FLAGS as u16, flags);
}

#[test]
fn test_mailbox_correct_after_white_long_castle() {
    const FEN: &str = "r1bqk2r/ppp1bppp/2np1n2/4p1B1/4P3/2NP4/PPPQ1PPP/R3KBNR w KQkq - 0 1";
    let mut mboard = mailbox::MailboxBoard::from_fen(FEN, " ");
    let move_ = chess_move::Move::new_no_promotion(60, 58);

    let flags = mboard.move_piece(&move_);

    const FEN_AFTER_MOVE: &str =
        "r1bqk2r/ppp1bppp/2np1n2/4p1B1/4P3/2NP4/PPPQ1PPP/2KR1BNR w Kkq - 0 1";
    test_mailbox_equal_fen(FEN_AFTER_MOVE, mboard);

    const EXPECTED_FLAGS: u8 = (KING_ID << chess_move::PIECE_INDEX)
        | (WHITE_ID << chess_move::COLOR_INDEX)
        | (1 << chess_move::CASTLE_INDEX);

    assert_eq_u8!(EXPECTED_FLAGS as u16, flags);
}

#[test]
fn test_mailbox_correct_after_black_long_castle() {
    const FEN: &str = "r3kbnr/ppp2ppp/2np1q2/4p3/2B1P1b1/2NP1N2/PPP2PPP/R1BQ1RK1 w HQkq - 0 1";
    let mut mboard = mailbox::MailboxBoard::from_fen(FEN, " ");
    let move_ = chess_move::Move::new_no_promotion(4, 2);

    let flags = mboard.move_piece(&move_);

    const FEN_AFTER_MOVE: &str =
        "2kr1bnr/ppp2ppp/2np1q2/4p3/2B1P1b1/2NP1N2/PPP2PPP/R1BQ1RK1 w Qk - 0 1";
    test_mailbox_equal_fen(FEN_AFTER_MOVE, mboard);

    const EXPECTED_FLAGS: u8 = (KING_ID << chess_move::PIECE_INDEX)
        | (BLACK_ID << chess_move::COLOR_INDEX)
        | (1 << chess_move::CASTLE_INDEX);

    assert_eq_u8!(EXPECTED_FLAGS as u16, flags);
}
