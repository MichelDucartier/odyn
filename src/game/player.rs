use crate::game::chessboard::Chessboard;

pub trait Player {
    fn next_move(&mut self, chessboard: Chessboard) -> u32;
}
