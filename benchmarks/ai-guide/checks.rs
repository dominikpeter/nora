#[cfg(test)]
mod guide_checks {
    use super::*;
    #[test]
    fn offset() { for (x,d,want) in [(0,0,0),(-9,4,-5),(7,-10,-3)] { assert_eq!(nora_offset(x,d),want); } }
    #[test]
    fn total() { for (q,p,d,want) in [(3,100,20,280),(0,7,4,-4),(-2,-7,-3,17)] { assert_eq!(nora_total(q,p,d),want); } }
    #[test]
    fn label() { assert_eq!(nora_label(2,3),"5"); assert_eq!(nora_label(-10,4),"-6"); assert_eq!(nora_label(0,0),"0"); }
    #[test]
    fn identity() { for value in ["", "🦀 hello", "a\0b"] { assert_eq!(nora_identity(value.to_owned()),value); } }
    #[test]
    fn bucket() { for (x,w,want) in [(7,2,5),(-8,2,-2),(8,-3,-3),(0,10,0)] { assert_eq!(nora_bucket(x,w),want); } }
}
