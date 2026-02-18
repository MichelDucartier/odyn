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

#[test]
fn test_start_position_perft_divide_depth_2() {
    let board = Chessboard::from_fen(START_FEN, " ");
    let divide = perft::perft_divide(&board, 2);

    assert_eq!(20, divide.len());
    assert!(divide.iter().all(|(_, nodes)| *nodes == 20));
    assert_eq!(400, divide.iter().map(|(_, nodes)| nodes).sum::<u64>());
}
