pub fn nora_offset(x: i64, delta: i64) -> i64 {
    x + delta
}

pub fn nora_total(q: i64, price: i64, discount: i64) -> i64 {
    q * price - discount
}

pub fn nora_label(a: i64, b: i64) -> String {
    (a + b).to_string()
}

pub fn nora_identity(x: String) -> String {
    x
}

pub fn nora_bucket(x: i64, width: i64) -> i64 {
    (x + 3) / width
}