use odyn::constants::QUIT_COMMAND;
use odyn::uci::{self, init_cli};
use odyn::{engine::implementations::mock::mock_engine::MockEngine, uci::controller::Controller};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::io::{self};

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();
    let mock_engine = MockEngine {};
    let controller = Controller::new(mock_engine);

    let mut rl = DefaultEditor::new()?;

    loop {
        let readline = rl.readline(">> ");

        let cli = init_cli()?;

        match readline {
            Ok(line) => {
                let args = cli.get_matches_from(line.split(" "));

                match args.subcommand() {
                    Some((cmd, _)) => {
                        println!("{}", cmd);
                    }
                    None => {
                        println!("None");
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }

            Err(err) => {
                println!("Error: {:?}", err);
            }
        }
    }

    Ok(())
}
