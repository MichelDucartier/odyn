use crate::{
    constants::{KING_ID, PIECE_VALUES},
    engine::evaluator::ChessEvaluator,
};

pub struct OdynEvaluator;

impl ChessEvaluator for OdynEvaluator {
    fn evaluate(&self, board: &crate::game::chessboard::Chessboard, color_id: u8) -> f32 {
        // Simple evaluator that maximizes the number of points you have in pieces value
        // It's a good starting point for a simple evaluator
        // You can use any other heuristic you want
        let mut points = 0.0;

        for (_, (piece_id, piece_color)) in board.get_iterator_on_pieces() {
            if piece_id == KING_ID {
                continue;
            }

            let piece_value = PIECE_VALUES.get(piece_id as usize);

            if piece_value.is_none() {
                continue;
            }

            let sign = if color_id == piece_color { 1 } else { -1 } as f32;
            points += sign * piece_value.unwrap();
        }

        points
    }
}
