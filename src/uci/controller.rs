use clap::Parser;

#[derive(Parser)]
pub struct EngineArgs {
    pub pattern: String,
    pub path: String,
}

pub struct Controller {
    is_ready: bool,
}

impl Controller {
    pub fn uci(&self) -> String {
        "uciok".to_string()
    }

    pub fn is_ready(&self) -> bool {
        self.is_ready
    }

    pub fn uci_new_game(&self) {}

    pub fn position(&self, fen: &str, moves: Vec<&str>) {}
    pub fn go(&self, args: EngineArgs) {}
    pub fn stop(&self) {}
    pub fn ponder_hit(&self) {}
    pub fn quit(&self) {}
}
