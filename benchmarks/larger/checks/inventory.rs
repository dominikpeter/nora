#[cfg(test)] mod guide_checks {
use super::*;
#[test] fn parse_merge_sort() {
assert_eq!(nora_parse(" # skip\r\n\n B_2 , 2 , 125 \r\nA-1,0,30\nB_2,3,125\na,1,0").unwrap(),vec![Stock{sku:"A-1".into(),quantity:0,unit_cents:30},Stock{sku:"B_2".into(),quantity:5,unit_cents:125},Stock{sku:"a".into(),quantity:1,unit_cents:0}]);
}
#[test] fn validation_order() {
for (input,expected) in [("x,1","line 1: fields"),("\n#x\n,1,2","line 3: sku"),("é,1,2","line 1: sku"),("a b,1,2","line 1: sku"),("a,-1,x","line 1: quantity"),("a,1,-1","line 1: price"),("a,9223372036854775808,1","line 1: quantity"),("a,1,9223372036854775808","line 1: price"),("a,1,2,3","line 1: fields")] {assert_eq!(nora_parse(input).unwrap_err(),expected);}
}
#[test] fn duplicates_and_overflow() {
assert_eq!(nora_parse("a,1,2\na,2,3").unwrap_err(),"line 2: price conflict");
assert_eq!(nora_parse("a,9223372036854775807,2\na,1,2").unwrap_err(),"line 2: quantity overflow");
assert_eq!(nora_parse("a,9223372036854775807,2\na,1,3").unwrap_err(),"line 2: price conflict");
assert_eq!(nora_parse("a,+1,+2").unwrap()[0].quantity,1);
}
#[test] fn report_sort_threshold_immutable() {
let items=vec![Stock{sku:"z".into(),quantity:2,unit_cents:5},Stock{sku:"A".into(),quantity:3,unit_cents:7},Stock{sku:"zero".into(),quantity:0,unit_cents:99}];
assert_eq!(nora_report(&items,3),"A:3:21:ok\nz:2:10:low\nzero:0:0:low\ntotal=31");
assert_eq!(items[0].sku,"z");
}
#[test] fn empty_and_negative_threshold() {
assert_eq!(nora_parse("\n #comment\n").unwrap(),vec![]);
assert_eq!(nora_report(&[],0),"total=0");
assert_eq!(nora_report(&[Stock{sku:"a".into(),quantity:0,unit_cents:3}],-1),"a:0:0:ok\ntotal=0");
}
}
