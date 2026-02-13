use crate::constants::{self, START_FEN, UCI_OK};
use crate::engine::engine::ChessEngine;
use crate::format_chessboard;
use crate::game::chess_move::Move;
use crate::game::utility;
use anyhow::{anyhow, Result};
use std::io::Write;

#[derive(Debug, Clone)]
pub struct PositionState {
    pub fen: String,
    pub moves: Vec<Move>,
}

impl Default for PositionState {
    fn default() -> Self {
        Self {
            fen: START_FEN.to_string(),
            moves: Vec::new(),
        }
    }
}

pub struct UciWrapper<T: ChessEngine> {
    engine: T,
    position: PositionState,
}

impl<T: ChessEngine> UciWrapper<T> {
    pub fn new(engine: T) -> Self {
        Self {
            engine,
            position: PositionState::default(),
        }
    }

    /// Handle one UCI input line.
    ///
    /// Returns `Ok(true)` when the caller should quit.
    pub fn handle_line(&mut self, line: &str, out: &mut dyn Write) -> Result<bool> {
        let line = line.trim();
        if line.is_empty() {
            return Ok(false);
        }

        let mut it = line.split_whitespace();
        let Some(cmd) = it.next() else {
            return Ok(false);
        };

        match cmd {
            "uci" => {
                // Minimal identification is required by most GUIs/bots.
                writeln!(out, "id name Odyn")?;
                writeln!(out, "id author MichelDucartier")?;
                writeln!(out, "{}", UCI_OK)?;
            }
            constants::IS_READY_COMMAND => {
                writeln!(out, "{}", constants::READY_OK)?;
            }
            constants::UCINEWGAME_COMMAND => {
                self.position = PositionState::default();
                // No output required by the UCI spec.
            }
            constants::POSITION_COMMAND => {
                let rest: Vec<&str> = it.collect();
                self.handle_position(&rest)?;
            }
            constants::GO_COMMAND => {
                // For now: compute a best move synchronously.
                // Lichess-bot is fine with a single `bestmove` line.
                self.engine
                    .position(&self.position.fen, self.position.moves.clone());

                let requested = self.engine.current_best_move();

                if let Some((mv, _val)) = requested {
                    writeln!(out, "bestmove {}", move_to_uci(mv))?;
                } else {
                    writeln!(out, "bestmove 0000")?;
                }
            }
            constants::STOP_COMMAND => {
                // No-op until we have async search.
            }
            constants::SET_OPTION_COMMAND => {
                // Accept and ignore for now.
                // UCI option wiring can be added once the engine supports it.
            }
            constants::DEBUG_COMMAND => {
                // Accept and ignore.
            }
            constants::PONDERHIT_COMMAND => {
                // Accept and ignore.
            }
            constants::REGISTER_COMMAND => {
                // Accept and ignore.
            }
            constants::QUIT_COMMAND => {
                return Ok(true);
            }
            _ => {
                // Unknown/unsupported command: ignore (don't break GUIs).
            }
        }

        out.flush()?;
        Ok(false)
    }

    fn handle_position(&mut self, tokens: &[&str]) -> Result<()> {
        if tokens.is_empty() {
            return Err(anyhow!("position: missing arguments"));
        }

        // UCI:
        // position startpos [moves ...]
        // position fen <fen...> [moves ...]
        let mut idx = 0;
        let mode = tokens[idx];
        idx += 1;

        let fen = if mode == "startpos" {
            START_FEN.to_string()
        } else if mode == "fen" {
            // Consume tokens until "moves" or end.
            let start = idx;
            while idx < tokens.len() && tokens[idx] != "moves" {
                idx += 1;
            }
            if idx == start {
                return Err(anyhow!("position fen: missing fen string"));
            }
            tokens[start..idx].join(" ")
        } else {
            return Err(anyhow!("position: expected 'startpos' or 'fen'"));
        };

        let mut moves: Vec<Move> = Vec::new();
        if idx < tokens.len() {
            if tokens[idx] != "moves" {
                return Err(anyhow!("position: expected 'moves' keyword"));
            }
            idx += 1;

            while idx < tokens.len() {
                let mv = parse_uci_move(tokens[idx])?;
                moves.push(mv);
                idx += 1;
            }
        }

        self.position.fen = fen;
        self.position.moves = moves;
        Ok(())
    }
}

pub fn parse_uci_move(s: &str) -> Result<Move> {
    // UCI move format:
    // - e2e4
    // - e7e8q (promotion)
    let s = s.trim();
    if s.len() < 4 {
        return Err(anyhow!("invalid UCI move: {}", s));
    }

    let from = s[0..2].to_ascii_lowercase();
    let to = s[2..4].to_ascii_lowercase();
    let promotion = s.get(4..5);

    let (from_row, from_col) = utility::string_to_square(&from)
        .ok_or_else(|| anyhow!("invalid from-square in move: {}", s))?;
    let (to_row, to_col) = utility::string_to_square(&to)
        .ok_or_else(|| anyhow!("invalid to-square in move: {}", s))?;

    let start_index = utility::square_to_index(from_row, from_col);
    let end_index = utility::square_to_index(to_row, to_col);

    let promotion_piece = match promotion.map(|c| c.chars().next().unwrap().to_ascii_lowercase()) {
        None => 0,
        Some('q') => constants::QUEEN_ID,
        Some('r') => constants::ROOK_ID,
        Some('b') => constants::BISHOP_ID,
        Some('n') => constants::KNIGHT_ID,
        Some(other) => {
            return Err(anyhow!(
                "invalid promotion piece '{}' in move: {}",
                other,
                s
            ));
        }
    };

    Ok(Move::new(start_index, end_index, promotion_piece))
}

pub fn move_to_uci(mv: Move) -> String {
    let from = utility::index_to_string(mv.start_index);
    let to = utility::index_to_string(mv.end_index);
    let promo = match mv.promotion_piece {
        0 => "".to_string(),
        constants::QUEEN_ID => "q".to_string(),
        constants::ROOK_ID => "r".to_string(),
        constants::BISHOP_ID => "b".to_string(),
        constants::KNIGHT_ID => "n".to_string(),
        _ => "".to_string(),
    };

    format!("{}{}{}", from, to, promo)
}
