use crate::game::chess_move;

pub trait ChessEngine {
    fn uci(&self) -> String;
    fn is_ready(&self) -> bool;
    fn uci_new_game(&self);
    fn position(&self, fen: &str, moves: Vec<chess_move::Move>);
    fn stop(&self);
    fn ponder_hit(&self);
    fn quit(&self);
}
