use anyhow::Result;
use odyn::{
    engine::implementations::{odyn_engine::OdynEngine, odyn_evaluator::OdynEvaluator},
    uci::protocol::UciWrapper,
};
use std::io::{self, BufRead};

fn main() -> Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let engine = OdynEngine::new(OdynEvaluator);
    let mut uci = UciWrapper::new(engine);

    for line in stdin.lock().lines() {
        let line = line?;
        let quit = match uci.handle_line(&line, &mut stdout) {
            Ok(quit) => quit,
            Err(err) => {
                // Don't crash on malformed UCI input; lichess-bot/GUIs will keep going.
                eprintln!("UCI error: {err:#}");
                false
            }
        };
        if quit {
            break;
        }
    }

    Ok(())
}
