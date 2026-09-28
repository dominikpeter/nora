#[cfg(test)] mod guide_checks {
use super::*;
#[test] fn helpers() {
assert_eq!(nora_gross(7,125),875); assert_eq!(nora_discount(999,333),33);
assert_eq!(nora_net(999,333),966); assert_eq!(nora_tax(966,825),79);
assert_eq!(nora_total(966,79),1045); assert_eq!(nora_due(1045,1100),-55);
assert_eq!(nora_shipping(7,12,40),124); assert_eq!(nora_margin(10,20),-10);
assert_eq!(nora_percent(1,3),3333); assert_eq!(nora_round_cent(14),1);
assert_eq!(nora_round_cent(15),2); assert_eq!(nora_label(12345),"12345");
assert_eq!(nora_identity("🦀".to_owned()),"🦀");
}
#[test] fn per_line_rounding() {assert_eq!(nora_invoice(&[(1,99,5000),(1,99,5000)],1000,0,0),"subtotal=100;tax=10;shipping=0;total=110;due=110");}
#[test] fn tax_excludes_shipping() {assert_eq!(nora_invoice(&[(2,500,1000)],825,100,200),"subtotal=900;tax=74;shipping=100;total=1074;due=874");}
#[test] fn empty_and_overpaid() {assert_eq!(nora_invoice(&[],500,50,100),"subtotal=0;tax=0;shipping=50;total=50;due=-50");}
#[test] fn mixed_lines() {assert_eq!(nora_invoice(&[(3,199,1500),(2,105,500),(0,9999,1000)],725,35,0),"subtotal=708;tax=51;shipping=35;total=794;due=794");}
}
