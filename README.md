# gcd

Print the greatest common divisor of two integers.

This README is written first and is the spec (README-driven development). The code must match it.

## Usage

```console
$ gcd 48 18
6
$ gcd -- -48 18
6
$ gcd 0 5
5
$ gcd 0 0
0
$ gcd --help
Print the greatest common divisor of two integers

Usage: gcd <A> <B>

Arguments:
  <A>
  <B>

Options:
  -h, --help  Print help

Examples:
  gcd 12 18
  gcd -- -48 18
$ gcd 12 18
6
$ gcd -- -48 18
6
```

- Two positional arguments `A` and `B`, parsed as signed 64-bit integers (`i64`). Negative numbers need `--` before them, as shown above.
- Output: the GCD as a non-negative decimal integer on stdout, followed by one newline. Nothing else goes to stdout.
- `gcd(a, 0) = |a|` and `gcd(0, 0) = 0`.
- The result for `i64::MIN` with `0` or with `i64::MIN` is `9223372036854775808`, which doesn't fit in `i64`. The result type is `u64`, so it prints correctly.

## Errors

| Situation | stderr | Exit code |
|---|---|---|
| Wrong number of arguments, or an argument that isn't an integer or is out of `i64` range | clap's usage error | 2 |
| Success | (nothing) | 0 |

## Install

```console
$ cargo install --path .
```

## Develop

See `AGENTS.md` for the build, test and verify commands.
