use super::{
    bitboard::Bitboard,
    chess_move::Move,
    mailbox::{self, MailboxBoard},
};
use crate::game::bitboard;

#[derive(Default, Debug)]
pub struct Chessboard {
    bitboard: bitboard::Bitboard,
    mailbox: mailbox::MailboxBoard,

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

        Chessboard {
            bitboard: Bitboard::from_fen(fen, separator),
            mailbox: MailboxBoard::from_fen(fen, separator),
            white_moves: s_wmoves.parse().unwrap(),
            black_moves: s_bmoves.parse().unwrap(),
        }
    }

    pub fn to_fen(&self, separator: &str) -> String {
        let mut bitboard_fen = self.bitboard.to_fen();
        let move_counts = format!("{} {}", self.black_moves, self.white_moves);
        bitboard_fen.push(move_counts);

        bitboard_fen.join(separator)
    }

    pub fn make_move_unchecked(&mut self, move_: Move) {
        let flags = self.mailbox.move_piece(&move_);
        self.bitboard.move_piece(&move_, flags);
    }
}
