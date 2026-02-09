use crate::game::chess_move;

pub trait ChessEngine {
    fn position(&self, fen: &str, moves: Vec<chess_move::Move>);
    fn current_best_move(&self) -> chess_move::Move;
}
