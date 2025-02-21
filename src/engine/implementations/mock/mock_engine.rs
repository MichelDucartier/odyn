use crate::{engine::engine::ChessEngine, game::chessboard::Chessboard};

struct MockEngine {
    chessboard: Chessboard,
}

// impl ChessEngine for MockEngine {
//     fn uci(&self) -> String {}
//     fn is_ready(&self) -> bool {}
//     fn uci_new_game(&self) {}
//     fn position(&self, fen: &str, moves: Vec<&str>) {}
//     fn stop(&self) {}
//     fn ponder_hit(&self) {}
//     fn quit(&self) {}
// }
