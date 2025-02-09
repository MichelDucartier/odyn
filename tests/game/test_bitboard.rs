use odyn::constants::START_FEN;
use odyn::game::chessboard;

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
