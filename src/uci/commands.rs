use clap::{Arg, Command};

use crate::constants;

/// Builds the `position` subcommand used by the CLI mode.
pub fn init_position_command() -> Command {
    Command::new(constants::POSITION_COMMAND)
        .about("Initialize new position")
        .arg(Arg::new("fen_keyword").required(true).value_parser(["fen"]))
        .arg(Arg::new("fenstring").required(true))
        .arg(
            Arg::new("moves_keyword")
                .required(false)
                .value_parser(["moves"]),
        )
        .arg(Arg::new("moves").required(false))
}

/// Builds the `quit` subcommand used by the CLI mode.
pub fn init_quit_command() -> Command {
    Command::new(constants::QUIT_COMMAND).about("Quit Odyn CLI")
}
