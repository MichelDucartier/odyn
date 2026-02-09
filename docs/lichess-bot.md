# Odyn + lichess-bot

This repo provides a UCI wrapper binary you can use with lichess-bot.

## 1) Build the UCI engine

From this repo root:

```bash
cargo build --release --bin odyn_uci
```

The engine binary will be at:

```text
target/release/odyn_uci
```

Quick sanity check:

```bash
printf "uci\nisready\nposition startpos\ngo movetime 50\nquit\n" | target/release/odyn_uci
```

You should see `uciok`, `readyok`, then a `bestmove ...` line.

## 2) Install lichess-bot

Clone lichess-bot (outside this repo) and follow its README for Python setup:

```bash
git clone https://github.com/lichess-bot-devs/lichess-bot.git
cd lichess-bot
```

## 3) Configure lichess-bot to run Odyn

Copy the example config in this repo:

- `docs/lichess-bot-config.example.yml`

into lichess-bot's `config.yml`, then edit:

- `token`: your Lichess bot token (keep it secret)
- `engine.dir`: absolute path to Odyn's `target/release` directory
- `engine.name`: keep as `odyn_uci`

Example (paths are just examples):

```yaml
engine:
  # Binary name (must match the file in engine.dir)
  name: "odyn_uci"
  protocol: "uci"
  dir: "/home/michael/Documents/Programmation/Rust/odyn/target/release"
```

## Notes

- The current engine implementation is `MockEngine` (it will always play a fixed move). Replace it with your real engine inside `src/bin/odyn_uci.rs`.
- UCI output must be clean (no prompts, no extra logs on stdout). If you want debug logs, write them to stderr.
