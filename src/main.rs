use clap::Parser;
use odyn::uci::controller::EngineArgs;

fn main() {
    let args = EngineArgs::parse();

    println!("{}", args.pattern);
}
