#[cfg(test)]
mod nora_bench {
    use super::*;
    #[test]
    fn divisors_and_symmetry() {
        for (a, b, expected) in [(1, 1, 1), (48, 18, 6), (17, 13, 1), (81, 27, 27), (u64::MAX, u64::MAX, u64::MAX)] {
            assert_eq!(gcd(a, b), expected);
            assert_eq!(gcd(b, a), expected);
        }
        for a in 1..80u64 {
            for b in 1..80u64 {
                let expected = (1..=a.min(b)).rev().find(|d| a % d == 0 && b % d == 0).unwrap();
                assert_eq!(gcd(a, b), expected);
            }
        }
    }
    #[test]
    #[should_panic]
    fn rejects_zero_left() { gcd(0, 1); }
    #[test]
    #[should_panic]
    fn rejects_zero_right() { gcd(1, 0); }
}
