"""Run with Python + tiktoken 0.13.0; counts source only, not agent cost."""
import tiktoken

CASES = [
    ("add(a:i,b:i)->i=a+b", "add(a,b:i)=a+b", "fn add(a: i64, b: i64) -> i64 { a + b }"),
    ("text(x:i)->s=str(x)", "text(x:i)=$x", "fn text(x: i64) -> String { x.to_string() }"),
    ("scale(x:i)->i=x*100", "scale(x:i)=x*100", "fn scale(x: i64) -> i64 { x * 100 }"),
    ("identity(x:s)->s=x", "identity(x:s)=x", "fn identity(x: String) -> String { x }"),
    ("price(q:i,p:i,discount:i)->i=q*p-discount", "price(q,p,discount:i)=q*p-discount", "fn price(q: i64, p: i64, discount: i64) -> i64 { q * p - discount }"),
]

for encoding in ("o200k_base", "cl100k_base"):
    tokenizer = tiktoken.get_encoding(encoding)
    print(f"\n{encoding}: old Nora / compact Nora / equivalent Rust")
    for row in CASES:
        print([len(tokenizer.encode(code)) for code in row], row[1])
    totals = [len(tokenizer.encode("\n".join(row[col] for row in CASES) + "\n")) for col in range(3)]
    print("Combined (including newlines):", totals)
    print(f"Compact reduction vs old Nora: {1-totals[1]/totals[0]:.1%}; vs Rust: {1-totals[1]/totals[2]:.1%}")
