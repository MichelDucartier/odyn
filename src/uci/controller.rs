use clap::{Parser, Subcommand};

use crate::{
    constants::{
        DEBUG_COMMAND, IS_READY_COMMAND, POSITION_COMMAND, READY_OK, SET_OPTION_COMMAND,
        UCINEWGAME_COMMAND,
    },
    engine::engine::ChessEngine,
};

pub struct Controller<T: ChessEngine> {
    chess_engine: T,
    debug: bool,
}

impl<T: ChessEngine> Controller<T> {
    pub fn new(chess_engine: T) -> Self {
        Controller {
            chess_engine: chess_engine,
            debug: false,
        }
    }

    fn handle_position_command(&mut self, command: String) -> Option<String> {
        None
    }

    fn handle_debug(&mut self) -> Option<String> {
        self.debug = !self.debug;
        None
    }

    fn handle_is_ready(&self) -> Option<String> {
        Some(READY_OK.to_string())
    }

    fn handle_set_option(&self) -> Option<String> {
        None
    }
}
