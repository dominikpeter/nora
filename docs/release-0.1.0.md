# Nora 0.1.0 experimental release

Intent: an AI-native, token-efficient representation of Rust with deterministic
expansion and full Rust available through source passthrough.

Included: compact i64/String functions, arithmetic, grouped parameters, inferred
returns, string conversion, source diagnostics, Rust passthrough, examples,
installation guide, token counter, and Programming Rust benchmark runner.

Known limits: compact syntax does not cover full Rust; there is no general Rust
parser/AST, Cargo integration, model runner, diagnostic source mapping, or
measured end-to-end AI cost advantage. Rust sections are copied, not validated
by Nora. Public benchmark checks are development tests, not secret holdouts.

Release checks:

```sh
cargo test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
python3 -m unittest discover -s scripts -p 'test_benchmark.py'
python3 scripts/bench_smoke.py
cargo package --allow-dirty
```

These commands verify and package the release locally. Version 0.1.0 is
distributed through GitHub Releases, not crates.io. The project license must
be selected before a future crates.io publication. Vendored benchmark references
retain their upstream MIT license.
