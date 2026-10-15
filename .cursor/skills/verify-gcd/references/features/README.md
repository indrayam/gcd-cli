# Feature map: gcd

| Feature | How a user reaches it | Expected |
|---|---|---|
| basic gcd | `gcd 48 18` | stdout `6`, exit 0 |
| coprime | `gcd 17 5` | `1`, exit 0 |
| one zero | `gcd 0 5` | `5`, exit 0 |
| both zero | `gcd 0 0` | `0`, exit 0 |
| negative input | `gcd -- -48 18` | `6`, exit 0 |
| i64::MIN edge | `gcd -- -9223372036854775808 0` | `9223372036854775808`, exit 0 |
| missing arg | `gcd 5` | usage error on stderr, exit 2 |
| not a number | `gcd 5 x` | usage error on stderr, exit 2 |
| help | `gcd --help` | usage on stdout, exit 0 |

Sweep order: top to bottom.
