use crate::game::bitboard;

use super::bitboard::Bitboard;

#[derive(Default, Debug)]
pub struct Chessboard {
    bitboard: bitboard::Bitboard,
    white_moves: u32,
    black_moves: u32,
}

impl Chessboard {
    pub fn from_fen(fen: &str, separator: &str) -> Chessboard {
        let fen_parts: Vec<&str> = fen.split(separator).collect();

        let [_s_board, _s_turn, _s_castle, _s_enpassant, s_bmoves, s_wmoves] = &fen_parts[..]
        else {
            panic!("Invalid fen, invalid number of parts")
        };

        let mut chessboard = Chessboard::default();

        // Load bitboard
        chessboard.bitboard = Bitboard::from_fen(fen, separator);

        // Load number of moves
        chessboard.white_moves = s_wmoves.parse().unwrap();
        chessboard.black_moves = s_bmoves.parse().unwrap();

        return chessboard;
    }

    pub fn to_fen(&mut self, separator: &str) -> String {
        let mut bitboard_fen = self.bitboard.to_fen();
        let move_counts = format!("{} {}", self.black_moves, self.white_moves);
        bitboard_fen.push(move_counts);

        bitboard_fen.join(separator)
    }
}
