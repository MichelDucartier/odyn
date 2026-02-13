use core::f32;

use crate::game::chessboard::Chessboard;

pub trait ChessEvaluator {
    fn evaluate(&self, board: &Chessboard, color_id: u8) -> f32;
}
