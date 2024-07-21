use crate::common::fen;
use odyn::{constants::START_FEN, game::chessboard};

#[test]
fn dummy_test() {
    assert_eq!(0, 0);
}

#[test]
fn test_to_from_identity_random_fen() {
    let mut cboard = chessboard::Chessboard::from_fen(fen::FEN1, " ");
    assert_eq!(fen::FEN1, cboard.to_fen(" "));
}

#[test]
fn test_to_from_identity_start_fen() {
    let mut cboard = chessboard::Chessboard::from_fen(START_FEN, " ");
    assert_eq!(START_FEN, cboard.to_fen(" "));
}

#[test]
fn test_to_from_identity_en_passant_white_fen() {
    let mut cboard = chessboard::Chessboard::from_fen(fen::FEN2, " ");
    assert_eq!(fen::FEN2, cboard.to_fen(" "));
}

#[test]
fn test_to_from_identity_en_passant_black_fen() {
    let mut cboard = chessboard::Chessboard::from_fen(fen::FEN3, " ");
    assert_eq!(fen::FEN3, cboard.to_fen(" "));
}
