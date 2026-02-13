# AGENTS.md
Guidance for coding agents working in this repository.
This project is a Rust chess engine (`odyn`) with a UCI wrapper binary.

## Project Snapshot
- Language: Rust (edition 2021)
- Primary crate: `odyn`
- Main library entry: `src/lib.rs`
- CLI binary: `src/main.rs`
- UCI binary: `src/bin/odyn_uci.rs`
- Integration tests: `tests/game/main.rs` + `tests/game/test_*.rs`
- CI workflow: `.github/workflows/rust.yml`

## Source Layout
- `src/game/`: board representation, move generation, utilities
- `src/engine/`: `ChessEngine` trait + implementations
- `src/uci/`: UCI protocol parsing/handling
- `tests/game/`: integration tests grouped under one `game` test target
- `docs/`: operational docs (including lichess-bot integration)

## Build Commands
Run from repository root.
- Debug build: `cargo build`
- Verbose debug build (CI-like): `cargo build --verbose`
- Release build: `cargo build --release`
- Release build for UCI binary only: `cargo build --release --bin odyn_uci`
- Typecheck/compile check only: `cargo check`

## Test Commands
- Run all tests: `cargo test`
- Run all tests verbose: `cargo test --verbose`
- List tests before selecting one: `cargo test -- --list`
- Compile tests only (no run): `cargo test --no-run`

### Run a Single Test (important)
Use one of these patterns:
- By substring match: `cargo test test_king_moves`
- Exact match + stdout: `cargo test test_move_generator::test_king_moves -- --exact --nocapture`
- Single integration test in `tests/game/main.rs` target:
  - `cargo test --test game test_move_generator::test_king_moves -- --exact --nocapture`
- Single library unit test:
  - `cargo test --lib game::chessboard::tests::test_unpack_move -- --exact --nocapture`

## Lint / Formatting Commands
No `clippy.toml` or `rustfmt.toml` is present; defaults apply.
- Format code: `cargo fmt --all`
- Check formatting only: `cargo fmt --all -- --check`
- Run clippy for all targets/features: `cargo clippy --all-targets --all-features`

Current state note:
- `cargo fmt --all -- --check` reports formatting diffs in existing files.
- `cargo clippy --all-targets --all-features` reports warnings but completes.

## Run Commands
- Run interactive binary: `cargo run`
- Run UCI binary: `cargo run --bin odyn_uci`
- UCI smoke test:
  - `printf "uci\nisready\nposition startpos\ngo movetime 50\nquit\n" | target/release/odyn_uci`

## CI Behavior
From `.github/workflows/rust.yml`:
- Triggers: pushes and PRs to `master`
- Build step: `cargo build --verbose`
- Test step: `cargo test --verbose`
Keep local verification at least at CI level before finalizing major changes.

## Recommended Verification Flow
Before opening a PR or finalizing non-trivial edits, run:
1. `cargo build --verbose`
2. `cargo test --verbose`
3. `cargo fmt --all -- --check`
4. `cargo clippy --all-targets --all-features`

If only one module changed, run at least one focused test first (single-test pattern),
then run broader checks before shipping.

## Code Style Guidelines
Follow existing repository patterns over personal preference.

### Imports
- Prefer grouped `use` statements with nested paths when clearer.
- Keep ordering rustfmt-friendly and stable.
- Remove unused imports when touching files.
- In constant-heavy modules, explicit imports are common and acceptable.

### Formatting
- Use rustfmt defaults (`cargo fmt --all`).
- 4-space indentation; no tabs.
- Prefer readable line lengths; let rustfmt wrap.
- Avoid manual alignment that rustfmt will undo.

### Types and Data Modeling
- Use explicit integer widths for board logic (`u64`, `u32`, `u16`, `u8`).
- Use structs/enums for domain entities (`Move`, `Bitboard`, `MailboxBoard`).
- Derive common traits when useful (`Debug`, `Clone`, `Copy`, `Default`, `Eq`, `Hash`).
- Keep bit-mask/flag constants explicit and centralized.

### Naming Conventions
- Types/traits: `UpperCamelCase` (`Chessboard`, `ChessEngine`)
- Functions/modules/variables: `snake_case` (`generate_rook_moves`, `from_fen`)
- Constants: `SCREAMING_SNAKE_CASE` (`START_FEN`, `WHITE_ID`)
- Tests: descriptive `snake_case`, generally prefixed with `test_`

### Error Handling
- For UCI/input boundaries, return `anyhow::Result` with useful context.
- Core game internals currently use `panic!`/`unwrap()` for invariants.
- For new protocol/parsing paths, prefer recoverable errors over panics.
- Avoid adding new `unwrap()` in long-running command/protocol I/O paths.
- In UCI loops, malformed input should usually be handled and ignored, not crash the process.

### Control Flow and APIs
- Prefer early returns for guard conditions.
- Keep functions focused; split helpers when logic grows multi-purpose.
- Preserve public signatures unless a task explicitly requires API changes.
- Respect module boundaries (`game`, `engine`, `uci`).

### Testing Expectations
- Add or update tests with behavior changes.
- For move generation, use deterministic expected bitboards.
- For UCI behavior, keep outputs spec-compatible (`uciok`, `readyok`, `bestmove ...`).
- When fixing bugs, add a focused regression test using the single-test commands.

## Agent Operational Rules in This Repo
- Make minimal, targeted changes.
- Do not refactor unrelated code during a feature/bugfix.
- Preserve current file/module organization unless restructuring is requested.
- If you touch nearby low-risk lint/format issues, fixing them is okay.

## Practical Notes for Agents
- Prefer edits that preserve existing public APIs unless explicitly asked to change them.
- Keep UCI stdout clean; protocol noise should go to stderr.
- For command parsing or protocol work, favor deterministic behavior over randomness.
- Avoid broad renames/reformat-only commits unless the task explicitly asks for cleanup.
- When adding tests, keep them deterministic and independent of execution order.
- When introducing constants, colocate them with existing constant groups.

## Cursor / Copilot Rules
Searched locations:
- `.cursor/rules/`
- `.cursorrules`
- `.github/copilot-instructions.md`

Result at time of writing:
- No Cursor rule files found.
- No Copilot instruction file found.

If these files are added later, update this AGENTS.md and treat those instructions as authoritative.
