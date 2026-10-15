#!/usr/bin/env bash
# Runs the real gcd binary against the feature map. Prints VERIFY: PASS or VERIFY: FAIL.
set -u
cd "$(git rev-parse --show-toplevel)"
cargo build --quiet || { echo "VERIFY: FAIL (build)"; exit 1; }
BIN=target/debug/gcd
fail=0
check() { # expected_stdout expected_exit args...
  local want_out="$1" want_code="$2"; shift 2
  local out code
  out="$("$BIN" "$@" 2>/dev/null)"; code=$?
  if [[ "$code" == "$want_code" && ( "$want_out" == "*" || "$out" == "$want_out" ) ]]; then
    echo "ok   gcd $* -> '${out}' exit=$code"
  else
    echo "FAIL gcd $* -> '${out}' exit=$code (want '${want_out}' exit=$want_code)"; fail=1
  fi
}
check 6 0 48 18
check 1 0 17 5
check 5 0 0 5
check 0 0 0 0
check 6 0 -- -48 18
check 9223372036854775808 0 -- -9223372036854775808 0
check "" 2 5
check "" 2 5 x
check "*" 0 --help
[[ $fail == 0 ]] && echo "VERIFY: PASS" || { echo "VERIFY: FAIL"; exit 1; }
