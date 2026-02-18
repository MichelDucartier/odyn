use std::collections::HashSet;

use crate::constants::{
    self, ALL_PIECES_ID, BISHOP_ID, KING_ID, KNIGHT_ID, PAWN_ID, QUEEN_ID, RANK_1_INDEX,
    RANK_8_INDEX, ROOK_ID,
};

use super::{bitboard::Bitboard, chess_move, chess_move::Move, chessboard::Chessboard, utility};

/// Counts legal leaf nodes reachable from `board` at `depth`.
pub fn perft(board: &Chessboard, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut nodes = 0;
    let legal_moves = board.legal_moves(board.current_turn());

    for move_ in legal_moves {
        let mut next = board.clone();
        next.make_move_unchecked(move_);

        if depth == 1 {
            nodes += 1;
        } else {
            nodes += perft(&next, depth - 1);
        }
    }

    nodes
}
