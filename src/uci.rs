pub mod commands;
pub mod controller;
pub mod protocol;
pub mod utils;

use anyhow::Result;

pub fn init_cli() -> Result<clap::Command> {
    let cmd = clap::Command::new("Odyn chess engine")
        .no_binary_name(true)
        .about("Odyn is a (badly programmed) chess engine")
        .author("MichelDucartier")
        .subcommand(commands::init_position_command())
        .subcommand(commands::init_quit_command());

    Ok(cmd)
}
