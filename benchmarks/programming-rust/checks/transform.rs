#[cfg(test)]
mod nora_bench {
    use super::*;
    #[test]
    fn prices_and_labels() {
        for (q, p, d, expected) in [(0,10,0,0), (3,100,20,280), (2,-7,3,-17), (-2,-7,-3,17)] {
            assert_eq!(nora_price(q,p,d), expected);
        }
        for (x, expected) in [(0,"0"), (-42,"-42"), (i64::MIN,"-9223372036854775808"), (i64::MAX,"9223372036854775807")] {
            assert_eq!(nora_label(x), expected);
        }
    }
}
