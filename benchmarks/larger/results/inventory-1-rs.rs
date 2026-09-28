#[derive(Debug, PartialEq, Eq)]
pub struct Stock {
    pub sku: String,
    pub quantity: i64,
    pub unit_cents: i64,
}

pub fn nora_parse(input: &str) -> Result<Vec<Stock>, String> {
    let mut items = std::collections::BTreeMap::<String, Stock>::new();

    for (index, line) in input.lines().enumerate() {
        let number = index + 1;
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let fields: Vec<&str> = line.split(',').map(str::trim).collect();
        if fields.len() != 3 {
            return Err(format!("line {number}: fields"));
        }

        let sku = fields[0];
        if sku.is_empty()
            || !sku
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
        {
            return Err(format!("line {number}: sku"));
        }

        let quantity = fields[1]
            .parse::<i64>()
            .ok()
            .filter(|&value| value >= 0)
            .ok_or_else(|| format!("line {number}: quantity"))?;

        let unit_cents = fields[2]
            .parse::<i64>()
            .ok()
            .filter(|&value| value >= 0)
            .ok_or_else(|| format!("line {number}: price"))?;

        if let Some(existing) = items.get_mut(sku) {
            if existing.unit_cents != unit_cents {
                return Err(format!("line {number}: price conflict"));
            }
            existing.quantity = existing
                .quantity
                .checked_add(quantity)
                .ok_or_else(|| format!("line {number}: quantity overflow"))?;
        } else {
            items.insert(
                sku.to_owned(),
                Stock {
                    sku: sku.to_owned(),
                    quantity,
                    unit_cents,
                },
            );
        }
    }

    Ok(items.into_values().collect())
}

pub fn nora_report(items: &[Stock], threshold: i64) -> String {
    let mut sorted: Vec<&Stock> = items.iter().collect();
    sorted.sort_unstable_by(|a, b| a.sku.cmp(&b.sku));

    let mut rows = Vec::new();
    let mut total = 0_i64;

    for item in sorted {
        let value = item.quantity * item.unit_cents;
        total += value;
        let state = if item.quantity < threshold {
            "low"
        } else {
            "ok"
        };
        rows.push(format!(
            "{}:{}:{}:{}",
            item.sku, item.quantity, value, state
        ));
    }

    rows.push(format!("total={total}"));
    rows.join("\n")
}