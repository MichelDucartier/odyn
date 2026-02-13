use std::f32::INFINITY;

use crate::{
    constants::START_FEN,
    engine::{engine::ChessEngine, evaluator::ChessEvaluator},
    format_chessboard,
    game::{chess_move, chessboard::Chessboard},
    uci::protocol::move_to_uci,
};

pub struct OdynEngine<E: ChessEvaluator> {
    chessboard: Chessboard,
    evaluator: E,
}

impl<E: ChessEvaluator> OdynEngine<E> {
    pub fn new(evaluator: E) -> Self {
        OdynEngine {
            chessboard: Chessboard::from_fen(START_FEN, " "),
            evaluator: evaluator,
        }
    }
}

impl<E: ChessEvaluator> ChessEngine for OdynEngine<E> {
    fn position(&mut self, fen: &str, moves: Vec<crate::game::chess_move::Move>) {
        self.chessboard = Chessboard::from_moves(fen, moves);
        println!("{}", format_chessboard(&self.chessboard))
    }

    fn current_best_move(&self) -> Option<(chess_move::Move, f32)> {
        let current_color = self.chessboard.current_turn();
        let mut max_value = -INFINITY;
        let mut best_move = None;

        for current_move in self.chessboard.compute_legal_moves() {
            println!("Move {}", move_to_uci(current_move));
            let mut cboard = self.chessboard.clone();
            cboard.make_move_unchecked(current_move);
            let value = self.evaluator.evaluate(&cboard, current_color);

            if value > max_value {
                max_value = value;
                best_move = Some(current_move);
            }
        }

        best_move.map(|m| (m, max_value))
    }
}
