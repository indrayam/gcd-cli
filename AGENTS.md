# AGENTS.md

`README.md` is the spec. If the code and the README disagree, the README wins unless the PR changes both on purpose.

## Commands
- Build: `cargo build`
- Test: `cargo test`
- Lint: `cargo clippy --all-targets -- -D warnings`
- Format check: `cargo fmt --check`
- Verify on the real binary: `bash .cursor/skills/verify-gcd/scripts/verify.sh` (it must print `VERIFY: PASS` and exit 0)

## Conventions
- Argument parsing: `clap` (derive).
- Pure logic, `gcd(a: i64, b: i64) -> u64`, lives in `src/lib.rs` with unit tests. `src/main.rs` only parses and prints.
- CLI tests go in `tests/cli.rs` (`assert_cmd` + `predicates` are fine as dev-dependencies).
- No comments that restate the code.

## Done means
`cargo fmt --check`, clippy, `cargo test` and `verify.sh` all pass. Paste the `verify.sh` output into the PR body as proof.
