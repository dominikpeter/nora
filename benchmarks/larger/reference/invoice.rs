pub fn nora_gross(q:i64,unit:i64)->i64{q*unit}
pub fn nora_discount(amount:i64,bps:i64)->i64{amount*bps/10000}
pub fn nora_net(amount:i64,bps:i64)->i64{amount-amount*bps/10000}
pub fn nora_tax(amount:i64,bps:i64)->i64{amount*bps/10000}
pub fn nora_total(amount:i64,tax:i64)->i64{amount+tax}
pub fn nora_due(total:i64,paid:i64)->i64{total-paid}
pub fn nora_shipping(weight:i64,rate:i64,base:i64)->i64{weight*rate+base}
pub fn nora_margin(sale:i64,cost:i64)->i64{sale-cost}
pub fn nora_percent(part:i64,whole:i64)->i64{part*10000/whole}
pub fn nora_round_cent(mills:i64)->i64{(mills+5)/10}
pub fn nora_label(amount:i64)->String{amount.to_string()}
pub fn nora_identity(value:String)->String{value}
pub fn nora_invoice(lines:&[(i64,i64,i64)],tax_bps:i64,shipping:i64,paid:i64)->String{
let subtotal:i64=lines.iter().map(|&(q,p,d)|nora_net(nora_gross(q,p),d)).sum();
let tax=nora_tax(subtotal,tax_bps);let total=subtotal+tax+shipping;
format!("subtotal={subtotal};tax={tax};shipping={shipping};total={total};due={}",total-paid)
}
