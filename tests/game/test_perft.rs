use odyn::{
    constants::START_FEN,
    game::{chessboard::Chessboard, perft},
};

#[test]
fn test_start_position_perft_depth_0_to_2() {
    let board = Chessboard::from_fen(START_FEN, " ");

    assert_eq!(1, perft::perft(&board, 0));
    assert_eq!(20, perft::perft(&board, 1));
    assert_eq!(400, perft::perft(&board, 2));
}
