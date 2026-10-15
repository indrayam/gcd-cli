pub fn gcd(a: i64, b: i64) -> u64 {
    let mut x = a.unsigned_abs();
    let mut y = b.unsigned_abs();
    while y != 0 {
        (x, y) = (y, x % y);
    }
    x
}

#[cfg(test)]
mod tests {
    use super::gcd;

    #[test]
    fn basic() {
        assert_eq!(gcd(48, 18), 6);
    }

    #[test]
    fn coprime() {
        assert_eq!(gcd(17, 5), 1);
    }

    #[test]
    fn one_zero() {
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(gcd(5, 0), 5);
    }

    #[test]
    fn both_zero() {
        assert_eq!(gcd(0, 0), 0);
    }

    #[test]
    fn negatives() {
        assert_eq!(gcd(-48, 18), 6);
        assert_eq!(gcd(48, -18), 6);
        assert_eq!(gcd(-48, -18), 6);
    }

    #[test]
    fn i64_min() {
        assert_eq!(gcd(i64::MIN, 0), 1u64 << 63);
        assert_eq!(gcd(i64::MIN, i64::MIN), 1u64 << 63);
    }
}
