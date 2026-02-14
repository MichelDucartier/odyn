//! Chess domain modules: board structures, pieces, move generation and helpers.

/// Bitboard representation and low-level board operations.
pub mod bitboard;
/// Move representation and packed move flags.
pub mod chess_move;
/// High-level board API built on top of bitboards and mailbox storage.
pub mod chessboard;
/// Precomputed magic/lookup helpers for sliding pieces.
pub mod magic;
/// Mailbox board representation for piece lookup by square.
pub mod mailbox;
/// Pseudo-legal move generation for each piece type.
pub mod move_generator;
/// Player related helpers and types.
pub mod player;
/// Conversion and bit manipulation utility functions.
pub mod utility;
