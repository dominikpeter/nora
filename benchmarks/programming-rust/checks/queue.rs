#[cfg(test)]
mod nora_bench {
    use super::*;
    #[test]
    fn fifo_across_refills() {
        let mut q = Queue::new();
        let mut oracle = std::collections::VecDeque::new();
        for round in 0..40 {
            for c in ['a', '∞', '🦀', '\0'] { q.push(c); oracle.push_back(c); }
            for _ in 0..(round % 5) { assert_eq!(q.pop(), oracle.pop_front()); }
            assert_eq!(q.is_empty(), oracle.is_empty());
        }
        while let Some(c) = oracle.pop_front() { assert_eq!(q.pop(), Some(c)); }
        assert!(q.is_empty());
        assert_eq!(q.pop(), None);
        q.push('z');
        assert_eq!(q.pop(), Some('z'));
    }
    #[test]
    fn split_preserves_internal_order() {
        let mut q = Queue::new();
        for c in ['a', 'b', 'c'] { q.push(c); }
        assert_eq!(q.pop(), Some('a'));
        q.push('d');
        assert_eq!(q.split(), (vec!['c', 'b'], vec!['d']));
    }
}
