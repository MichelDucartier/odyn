use core::f32;

use crate::game::chessboard::Chessboard;

/// Scores a board position from the perspective of a given color.
pub trait ChessEvaluator {
    /// Returns a higher value for positions better for `color_id`.
    fn evaluate(&self, board: &Chessboard, color_id: u8) -> f32;
}
