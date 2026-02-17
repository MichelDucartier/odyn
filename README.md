# Odyn

Odyn is a Rust chess engine project with a UCI-compatible wrapper binary.

It includes:
- core board representation and move generation,
- a pluggable engine/evaluator architecture,
- integration tests for game logic,
- and an `odyn_uci` binary you can run from chess GUIs or bots (like lichess-bot).

## Current Status

Odyn is under active development.

The default engine implementation (`OdynEngine` + `OdynEvaluator`) currently scores positions with a simple material heuristic and selects the move with the best immediate evaluation.

## Repository Layout

```text
.
├── src/
│   ├── game/        # board state, bitboards, mailbox, move generation
│   ├── engine/      # ChessEngine / ChessEvaluator traits + implementations
│   ├── uci/         # UCI parsing and command handling
│   ├── lib.rs       # library entrypoint
│   ├── main.rs      # simple CLI binary
│   └── bin/
│       └── odyn_uci.rs
├── tests/game/      # integration tests (chessboard, move generator, utility, mailbox)
└── docs/
    └── lichess-bot.md
```

## Requirements

- Rust stable toolchain (edition 2021)
- Cargo

Install Rust via [rustup](https://rustup.rs/) if needed.

## Build

From the repository root:

```bash
cargo build
```

Release build:

```bash
cargo build --release
```

Build only the UCI binary:

```bash
cargo build --release --bin odyn_uci
```

## Run

Run the default binary:

```bash
cargo run
```

Run the UCI binary:

```bash
cargo run --bin odyn_uci
```

Quick UCI smoke test (after release build):

```bash
printf "uci\nisready\nposition startpos\ngo movetime 50\nquit\n" | target/release/odyn_uci
```

You should see UCI responses including:
- `uciok`
- `readyok`
- `bestmove ...`

## Testing

Run all tests:

```bash
cargo test
```

Run one focused test (example):

```bash
cargo test --test game test_move_generator::test_king_moves -- --exact --nocapture
```

## Format and Lint

Format:

```bash
cargo fmt --all
```

Check formatting only:

```bash
cargo fmt --all -- --check
```

Run clippy:

```bash
cargo clippy --all-targets --all-features
```

## UCI Integration Notes

Odyn's UCI wrapper accepts standard commands such as:
- `uci`
- `isready`
- `ucinewgame`
- `position`
- `go`
- `stop`
- `quit`

For lichess-bot setup, see:
- `docs/lichess-bot.md`

## Contributing

Contributions are welcome.

A good local verification flow before opening a PR is:
1. `cargo build --verbose`
2. `cargo test --verbose`
3. `cargo fmt --all -- --check`
4. `cargo clippy --all-targets --all-features`
