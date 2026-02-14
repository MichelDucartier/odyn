//! UCI protocol entry points and command configuration.

/// Clap command definitions used by the CLI wrapper.
pub mod commands;
/// UCI protocol parser and command dispatcher.
pub mod protocol;

use anyhow::Result;

/// Initializes the command-line interface command tree.
pub fn init_cli() -> Result<clap::Command> {
    let cmd = clap::Command::new("Odyn chess engine")
        .no_binary_name(true)
        .about("Odyn is a (badly programmed) chess engine")
        .author("MichelDucartier")
        .subcommand(commands::init_position_command())
        .subcommand(commands::init_quit_command());

    Ok(cmd)
}
