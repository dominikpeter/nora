#[cfg(test)]
mod nora_bench {
    use super::*;
    use std::cmp::Ordering::*;
    #[test]
    fn ordering_and_overlap() {
        let a = Interval { lower: 10, upper: 20 };
        for (lower, upper, expected) in [(10,20,Some(Equal)), (20,30,Some(Less)), (0,10,Some(Greater)), (15,25,None), (12,18,None), (0,30,None)] {
            let b = Interval { lower, upper };
            assert_eq!(a.partial_cmp(&b), expected);
            assert_eq!(b.partial_cmp(&a), expected.map(|v| v.reverse()));
        }
        let x = Interval { lower: 0.5, upper: 1.5 };
        assert_eq!(x.partial_cmp(&Interval { lower: 1.5, upper: 2.5 }), Some(Less));
    }
}
