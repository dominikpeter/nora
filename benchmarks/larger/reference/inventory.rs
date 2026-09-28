use std::collections::BTreeMap;
#[derive(Debug, PartialEq, Eq)]
pub struct Stock { pub sku:String, pub quantity:i64, pub unit_cents:i64 }
pub fn nora_parse(input:&str)->Result<Vec<Stock>,String>{
let mut stock:BTreeMap<String,Stock>=BTreeMap::new();
for (index,line) in input.lines().enumerate(){
let line=line.trim();if line.is_empty() || line.starts_with('#'){continue;}
let err=|kind:&str|format!("line {}: {}",index+1,kind);
let f:Vec<_>=line.split(',').map(str::trim).collect();
if f.len()!=3{return Err(err("fields"));}
if f[0].is_empty() || !f[0].bytes().all(|c|c.is_ascii_alphanumeric() || c==b'-' || c==b'_'){return Err(err("sku"));}
let quantity=f[1].parse::<i64>().ok().filter(|n|*n>=0).ok_or_else(||err("quantity"))?;
let price=f[2].parse::<i64>().ok().filter(|n|*n>=0).ok_or_else(||err("price"))?;
if let Some(row)=stock.get_mut(f[0]){
if row.unit_cents!=price{return Err(err("price conflict"));}
row.quantity=row.quantity.checked_add(quantity).ok_or_else(||err("quantity overflow"))?;
}else{stock.insert(f[0].into(),Stock{sku:f[0].into(),quantity,unit_cents:price});}
}
Ok(stock.into_values().collect())
}
pub fn nora_report(items:&[Stock],threshold:i64)->String{
let mut sorted:Vec<_>=items.iter().collect();sorted.sort_by(|a,b|a.sku.cmp(&b.sku));
let mut lines=Vec::new();let mut total=0;
for row in sorted{let value=row.quantity*row.unit_cents;total+=value;
lines.push(format!("{}:{}:{}:{}",row.sku,row.quantity,value,if row.quantity<threshold{"low"}else{"ok"}));}
lines.push(format!("total={total}"));lines.join("\n")
}
