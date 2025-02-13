use odyn::constants::START_FEN;
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

    let move_ = chess_move::Move {
        start_index: 52,
        end_index: 36,
    };
    cboard.make_move_unchecked(move_);

    const FEN_AFTER_MOVE: &str = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1";

    assert_eq!(FEN_AFTER_MOVE, cboard.to_fen(" "))
}

#[test]
fn test_correct_after_bishop_move() {
    const FEN: &str = "rnbqkbnr/pppp1ppp/8/4p3/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 1";
    let mut cboard = chessboard::Chessboard::from_fen(FEN, " ");

    let move_ = chess_move::Move {
        start_index: 5,
        end_index: 26,
    };
    cboard.make_move_unchecked(move_);

    const FEN_AFTER_MOVE: &str = "rnbqk1nr/pppp1ppp/8/2b1p3/4P3/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 1";

    assert_eq!(FEN_AFTER_MOVE, cboard.to_fen(" "))
}

#[test]
fn test_correct_for_en_passant() {
    const FEN: &str = "r1bqk2r/pppp1ppp/3b1n2/4p2P/2BnP3/5N2/PPPP1PP1/RNBQK2R b KQkq - 0 1";
    let mut cboard = chessboard::Chessboard::from_fen(FEN, " ");

    let move_ = chess_move::Move {
        start_index: 14,
        end_index: 30,
    };
    cboard.make_move_unchecked(move_);

    const FEN_AFTER_MOVE: &str =
        "r1bqk2r/pppp1p1p/3b1n2/4p1pP/2BnP3/5N2/PPPP1PP1/RNBQK2R w KQkq g6 0 1";

    assert_eq!(FEN_AFTER_MOVE, cboard.to_fen(" "));

    let en_passant_move = chess_move::Move {
        start_index: 31,
        end_index: 22,
    };
    cboard.make_move_unchecked(en_passant_move);

    const FEN_AFTER_EN_PASSANT: &str =
        "r1bqk2r/pppp1p1p/3b1nP1/4p3/2BnP3/5N2/PPPP1PP1/RNBQK2R b KQkq - 0 1";
    assert_eq!(FEN_AFTER_EN_PASSANT, cboard.to_fen(" "));
}

#[test]
fn test_correct_for_en_passant_without_taking() {
    const FEN: &str = "r1bqk2r/pppp1ppp/3b1n2/4p2P/2BnP3/5N2/PPPP1PP1/RNBQK2R b KQkq - 0 1";
    let mut cboard = chessboard::Chessboard::from_fen(FEN, " ");

    let move_ = chess_move::Move {
        start_index: 14,
        end_index: 30,
    };
    cboard.make_move_unchecked(move_);

    const FEN_AFTER_MOVE: &str =
        "r1bqk2r/pppp1p1p/3b1n2/4p1pP/2BnP3/5N2/PPPP1PP1/RNBQK2R w KQkq g6 0 1";

    assert_eq!(FEN_AFTER_MOVE, cboard.to_fen(" "));

    let not_en_passant = chess_move::Move {
        start_index: 57,
        end_index: 42,
    };
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
    let move_ = chess_move::Move {
        start_index: 60,
        end_index: 62,
    };
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
    let move_ = chess_move::Move {
        start_index: 60,
        end_index: 58,
    };
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
    let move_ = chess_move::Move {
        start_index: 4,
        end_index: 6,
    };
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
    let move_ = chess_move::Move {
        start_index: 4,
        end_index: 2,
    };
    cboard.make_move_unchecked(move_);

    const FEN_AFTER_MOVE: &str =
        "2kr3r/ppp1qppp/2npbn2/2b1p3/4P3/2NP1Q1N/PPPBBPPP/R3K2R w KQ - 0 1";

    assert_eq!(FEN_AFTER_MOVE, cboard.to_fen(" "));
}
