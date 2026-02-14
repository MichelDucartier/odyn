//! Core library crate for the Odyn chess engine.
//!
//! This crate exposes game state/manipulation primitives, engine traits and
//! implementations, and UCI protocol helpers.

/// Shared constants used by the game and UCI layers.
pub mod constants;
/// Core board representation and move generation modules.
pub mod game;
/// Formats a [`game::chessboard::Chessboard`] for terminal display.
pub use game::chessboard::format_chessboard;
/// Formats a raw bitboard (`u64`) as an 8x8 board string.
pub use game::utility::format_bitboard;
mod assert;
/// Engine abstractions and built-in implementations.
pub mod engine;
/// UCI protocol commands and wrapper utilities.
pub mod uci;
