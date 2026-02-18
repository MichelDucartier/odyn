use super::{chess_move::Move, chessboard::Chessboard};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PerftMismatch {
    pub move_: Move,
    pub expected: Option<u64>,
    pub actual: Option<u64>,
}

/// Counts legal leaf nodes reachable from `board` at `depth`.
///
/// This follows the classic perft routine described on chessprogramming.org,
/// including the `depth == 1` bulk-counting fast path.
pub fn perft(board: &Chessboard, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }

    let legal_moves = board.legal_moves(board.current_turn());
    if depth == 1 {
        return legal_moves.len() as u64;
    }

    legal_moves
        .into_iter()
        .map(|move_| {
            let mut next = board.clone();
            next.make_move_unchecked(move_);
            perft(&next, depth - 1)
        })
        .sum()
}

/// Returns per-root-move node counts (`divide` output).
pub fn perft_divide(board: &Chessboard, depth: u8) -> Vec<(Move, u64)> {
    if depth == 0 {
        return Vec::new();
    }

    board
        .legal_moves(board.current_turn())
        .into_iter()
        .map(|move_| {
            let mut next = board.clone();
            next.make_move_unchecked(move_);

            let nodes = if depth == 1 {
                1
            } else {
                perft(&next, depth - 1)
            };
            (move_, nodes)
        })
        .collect()
}
