pub fn gcd(a: i64, b: i64) -> u64 {
    let mut x = a.unsigned_abs();
    let mut y = b.unsigned_abs();
    while y != 0 {
        let r = x % y;
        x = y;
        y = r;
    }
    x
}

#[cfg(test)]
mod tests {
    use super::gcd;

    #[test]
    fn twelve_and_eighteen() {
        assert_eq!(gcd(12, 18), 6);
    }

    #[test]
    fn negatives_after_dash_dash() {
        assert_eq!(gcd(-48, 18), 6);
    }

    #[test]
    fn coprime() {
        assert_eq!(gcd(17, 5), 1);
    }

    #[test]
    fn one_zero() {
        assert_eq!(gcd(0, 5), 5);
    }

    #[test]
    fn both_zero() {
        assert_eq!(gcd(0, 0), 0);
    }

    #[test]
    fn i64_min_with_zero() {
        assert_eq!(gcd(i64::MIN, 0), 9223372036854775808);
    }

    #[test]
    fn i64_min_with_itself() {
        assert_eq!(gcd(i64::MIN, i64::MIN), 9223372036854775808);
    }
}
