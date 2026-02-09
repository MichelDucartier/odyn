use crate::game::chess_move::{self, Move};
use crate::game::utility;
use crate::{engine::engine::ChessEngine, game::chessboard::Chessboard};

pub struct MockEngine {}

impl ChessEngine for MockEngine {
    fn position(&self, fen: &str, moves: Vec<Move>) {}

    fn current_best_move(&self) -> crate::game::chess_move::Move {
        let (start_row, start_col) = utility::string_to_square("e2").unwrap();
        let (end_row, end_col) = utility::string_to_square("e4").unwrap();

        let start_index = utility::square_to_index(start_row, start_col);
        let end_index = utility::square_to_index(end_row, end_col);

        chess_move::Move::new_no_promotion(start_index, end_index)
    }
}
