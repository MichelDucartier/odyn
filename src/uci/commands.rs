use clap::{Arg, Command};

use crate::constants;

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

pub fn init_quit_command() -> Command {
    Command::new(constants::QUIT_COMMAND).about("Quit Odyn CLI")
}
