---
name: verify-gcd
description: Verify gcd-cli changes on the real binary. Use after any change to the gcd CLI, before you say it is done or open a PR.
---

# verify-gcd

Prove that the `gcd` binary behaves the way `README.md` says. Run the real binary; don't just reason about the code.

## Launch
`cargo build` (debug). The binary is `target/debug/gcd`.

## Drive and prove
Run `bash .cursor/skills/verify-gcd/scripts/verify.sh`. It builds the binary and runs every case in `references/features/README.md`. For each case it checks stdout and the exit code, prints one line per case, and ends with `VERIFY: PASS` or `VERIFY: FAIL`.

To check a single case: `./target/debug/gcd 48 18; echo "exit=$?"`

## Evidence
Paste the full `verify.sh` output into the PR body. A behavior change counts as verified only when it has a case in the feature map and `verify.sh` passes.

## Maintain
When behavior changes, update `README.md`, `references/features/README.md` and the `CASES` table in `scripts/verify.sh` in the same PR.
