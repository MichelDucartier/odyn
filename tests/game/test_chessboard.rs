use odyn::constants::{self, START_FEN};
use odyn::game::{chess_move, chessboard};

#[test]
fn dummy_test() {
    assert_eq!(0, 0);
}

#[test]
fn test_to_from_identity_random_fen() {
    const FEN1: &str = "2b5/p2NBp1p/1bp1nPPr/3P4/2pRnr1P/1k1B1Ppp/1P1P1pQP/Rq1N3K b - - 0 1";
    let cboard = chessboard::Chessboard::from_fen(FEN1, " ");
    assert_eq!(FEN1, cboard.to_fen(" "));
}

#[test]
fn test_to_from_identity_start_fen() {
    let cboard = chessboard::Chessboard::from_fen(START_FEN, " ");
    assert_eq!(START_FEN, cboard.to_fen(" "));
}

#[test]
fn test_to_from_identity_en_passant_white_fen() {
    const FEN2: &str = "rnbqkbnr/pp1ppppp/8/2p5/3PP3/8/PPP2PPP/RNBQKBNR b KQkq d3 0 2";
    let cboard = chessboard::Chessboard::from_fen(FEN2, " ");
    assert_eq!(FEN2, cboard.to_fen(" "));
}

#[test]
fn test_to_from_identity_en_passant_black_fen() {
    const FEN3: &str = "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2";

    let cboard = chessboard::Chessboard::from_fen(FEN3, " ");
    assert_eq!(FEN3, cboard.to_fen(" "));
}

#[test]
fn test_correct_after_pawn_move() {
    let mut cboard = chessboard::Chessboard::from_fen(START_FEN, " ");

    let move_ = chess_move::Move::new_no_promotion(52, 36);
    cboard.make_move_unchecked(move_);

    const FEN_AFTER_MOVE: &str = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1";

    assert_eq!(FEN_AFTER_MOVE, cboard.to_fen(" "))
}

#[test]
fn test_correct_after_bishop_move() {
    const FEN: &str = "rnbqkbnr/pppp1ppp/8/4p3/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 1";
    let mut cboard = chessboard::Chessboard::from_fen(FEN, " ");

    let move_ = chess_move::Move::new_no_promotion(5, 26);
    cboard.make_move_unchecked(move_);

    const FEN_AFTER_MOVE: &str = "rnbqk1nr/pppp1ppp/8/2b1p3/4P3/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 1";

    assert_eq!(FEN_AFTER_MOVE, cboard.to_fen(" "))
}

#[test]
fn test_correct_for_en_passant() {
    const FEN: &str = "r1bqk2r/pppp1ppp/3b1n2/4p2P/2BnP3/5N2/PPPP1PP1/RNBQK2R b KQkq - 0 1";
    let mut cboard = chessboard::Chessboard::from_fen(FEN, " ");

    let move_ = chess_move::Move::new_no_promotion(14, 30);
    cboard.make_move_unchecked(move_);

    const FEN_AFTER_MOVE: &str =
        "r1bqk2r/pppp1p1p/3b1n2/4p1pP/2BnP3/5N2/PPPP1PP1/RNBQK2R w KQkq g6 0 1";

    assert_eq!(FEN_AFTER_MOVE, cboard.to_fen(" "));

    let en_passant_move = chess_move::Move::new_no_promotion(31, 22);
    cboard.make_move_unchecked(en_passant_move);

    const FEN_AFTER_EN_PASSANT: &str =
        "r1bqk2r/pppp1p1p/3b1nP1/4p3/2BnP3/5N2/PPPP1PP1/RNBQK2R b KQkq - 0 1";
    assert_eq!(FEN_AFTER_EN_PASSANT, cboard.to_fen(" "));
}

#[test]
fn test_correct_for_en_passant_without_taking() {
    const FEN: &str = "r1bqk2r/pppp1ppp/3b1n2/4p2P/2BnP3/5N2/PPPP1PP1/RNBQK2R b KQkq - 0 1";
    let mut cboard = chessboard::Chessboard::from_fen(FEN, " ");

    let move_ = chess_move::Move::new_no_promotion(14, 30);
    cboard.make_move_unchecked(move_);

    const FEN_AFTER_MOVE: &str =
        "r1bqk2r/pppp1p1p/3b1n2/4p1pP/2BnP3/5N2/PPPP1PP1/RNBQK2R w KQkq g6 0 1";

    assert_eq!(FEN_AFTER_MOVE, cboard.to_fen(" "));

    let not_en_passant = chess_move::Move::new_no_promotion(57, 42);
    cboard.make_move_unchecked(not_en_passant);

    const FEN_AFTER_EN_PASSANT: &str =
        "r1bqk2r/pppp1p1p/3b1n2/4p1pP/2BnP3/2N2N2/PPPP1PP1/R1BQK2R b KQkq - 0 1";
    assert_eq!(FEN_AFTER_EN_PASSANT, cboard.to_fen(" "));
}

#[test]
fn test_correct_for_white_short_castle() {
    const FEN: &str = "r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 1";
    let mut cboard = chessboard::Chessboard::from_fen(FEN, " ");

    // Castle move
    let move_ = chess_move::Move::new_no_promotion(60, 62);
    cboard.make_move_unchecked(move_);

    const FEN_AFTER_MOVE: &str =
        "r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/5N2/PPPP1PPP/RNBQ1RK1 b kq - 0 1";

    assert_eq!(FEN_AFTER_MOVE, cboard.to_fen(" "));
}

#[test]
fn test_correct_for_white_long_castle() {
    const FEN: &str = "r1b1k2r/ppppqppp/2n2n2/2b1p3/4P3/2NP1Q2/PPPB1PPP/R3KBNR w KQkq - 0 1";
    let mut cboard = chessboard::Chessboard::from_fen(FEN, " ");

    // Castle move
    let move_ = chess_move::Move::new_no_promotion(60, 58);
    cboard.make_move_unchecked(move_);

    const FEN_AFTER_MOVE: &str =
        "r1b1k2r/ppppqppp/2n2n2/2b1p3/4P3/2NP1Q2/PPPB1PPP/2KR1BNR b kq - 0 1";

    assert_eq!(FEN_AFTER_MOVE, cboard.to_fen(" "));
}

#[test]
fn test_correct_for_black_short_castle() {
    const FEN: &str = "r1bqk2r/pppp1ppp/2n2n2/2b1p3/2B1P3/2P2N2/PP1P1PPP/RNBQ1RK1 b kq - 0 1";
    let mut cboard = chessboard::Chessboard::from_fen(FEN, " ");

    // Castle move
    let move_ = chess_move::Move::new_no_promotion(4, 6);
    cboard.make_move_unchecked(move_);

    const FEN_AFTER_MOVE: &str =
        "r1bq1rk1/pppp1ppp/2n2n2/2b1p3/2B1P3/2P2N2/PP1P1PPP/RNBQ1RK1 w - - 0 1";

    assert_eq!(FEN_AFTER_MOVE, cboard.to_fen(" "));
}

#[test]
fn test_correct_for_black_long_castle() {
    const FEN: &str = "r3k2r/ppp1qppp/2npbn2/2b1p3/4P3/2NP1Q1N/PPPBBPPP/R3K2R b KQkq - 0 1";
    let mut cboard = chessboard::Chessboard::from_fen(FEN, " ");

    // Castle move
    let move_ = chess_move::Move::new_no_promotion(4, 2);
    cboard.make_move_unchecked(move_);

    const FEN_AFTER_MOVE: &str =
        "2kr3r/ppp1qppp/2npbn2/2b1p3/4P3/2NP1Q1N/PPPBBPPP/R3K2R w KQ - 0 1";

    assert_eq!(FEN_AFTER_MOVE, cboard.to_fen(" "));
}

#[test]
fn test_number_of_pseudo_legal_moves_start_pos() {
    let cboard = chessboard::Chessboard::from_fen(START_FEN, " ");

    let legal_moves = cboard.pseudo_legal_moves(constants::WHITE_ID);

    for move_ in legal_moves.iter() {
        println!("{}", move_);
    }

    assert_eq!(20, legal_moves.len());
}

#[test]
fn test_is_in_check_start_pos() {
    let cboard = chessboard::Chessboard::from_fen(START_FEN, " ");
    let is_in_check = cboard.is_in_check(constants::WHITE_ID);
    assert!(!is_in_check);
}

#[test]
fn test_is_in_check_white_in_check() {
    let cboard = chessboard::Chessboard::from_fen(
        "rnbqk1nr/pppp1ppp/8/4p3/1b2P3/3P1N2/PPP2PPP/RNBQKB1R w KQkq - 0 1",
        " ",
    );
    let is_in_check = cboard.is_in_check(constants::WHITE_ID);
    assert!(is_in_check);
}

#[test]
fn test_is_in_check_pawn_attacks() {
    let cboard = chessboard::Chessboard::from_fen(
        "rnbqk1nr/ppp2ppp/8/4p3/1b1pP3/3PKN2/PPP2PPP/RNBQ1B1R w kq - 0 1",
        " ",
    );
    let is_in_check = cboard.is_in_check(constants::WHITE_ID);
    assert!(is_in_check);
}

#[test]
fn test_checkmate_scholar_mate() {
    let cboard = chessboard::Chessboard::from_fen(
        "r1bqkb1r/pppp1Qpp/2n2n2/4p3/2B1P3/8/PPPP1PPP/RNB1K1NR b KQkq - 0 1",
        " ",
    );

    let is_checkmate = cboard.is_checkmate();
    assert!(is_checkmate);
}

#[test]
fn test_number_of_legal_moves_start_pos() {
    let cboard = chessboard::Chessboard::from_fen(START_FEN, " ");

    let legal_moves = cboard.compute_legal_moves();

    for move_ in legal_moves.iter() {
        println!("{}", move_);
    }

    assert_eq!(20, legal_moves.len());
}

#[test]
fn test_capture_when_same_type() {}

#[test]
fn test_promotion_white_no_capture() {
    const FEN: &str = "8/2k3P1/4n3/1n6/4K3/2p3N1/8/8 w - - 0 1";
    let mut cboard = chessboard::Chessboard::from_fen(FEN, " ");

    let move_ = chess_move::Move::new(14, 6, constants::QUEEN_ID);
    cboard.make_move_unchecked(move_);

    const FEN_AFTER_MOVE: &str = "6Q1/2k5/4n3/1n6/4K3/2p3N1/8/8 b - - 0 1";
    assert_eq!(FEN_AFTER_MOVE, cboard.to_fen(" "));
}

#[test]
fn test_promotion_black_capture() {
    const FEN: &str = "8/2k5/4n1P1/1n6/4K3/6N1/2p5/1B6 b - - 0 1";
    let mut cboard = chessboard::Chessboard::from_fen(FEN, " ");

    let move_ = chess_move::Move::new(50, 57, constants::KNIGHT_ID);
    cboard.make_move_unchecked(move_);

    const FEN_AFTER_MOVE: &str = "8/2k5/4n1P1/1n6/4K3/6N1/8/1n6 w - - 0 1";
    assert_eq!(FEN_AFTER_MOVE, cboard.to_fen(" "));
}
