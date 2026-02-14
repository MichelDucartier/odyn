use crate::game::chess_move;

/// Defines the minimal behavior required from a chess engine.
pub trait ChessEngine {
    /// Loads a position and optional move history into the engine.
    fn position(&mut self, fen: &str, moves: Vec<chess_move::Move>);
    /// Returns the currently selected best move and its score, if any.
    fn current_best_move(&self) -> Option<(chess_move::Move, f32)>;
}
