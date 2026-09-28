# List available commands.
default:
    @just --list

# Build the Nora compiler.
build:
    cargo build

# Install Nora from this checkout.
install:
    cargo install --path .

# Run compiler and benchmark-runner tests.
test:
    cargo test
    python3 -m unittest discover -s scripts -p 'test_benchmark.py'

# Check Rust formatting and lint warnings.
lint:
    cargo fmt --check
    cargo clippy --all-targets -- -D warnings

# Format Rust source.
fmt:
    cargo fmt

# Run all five Rust/Nora benchmark fixtures; save a JSON report.
bench:
    python3 scripts/bench_smoke.py

# Run tests, lint, and benchmark fixtures.
check: test lint bench

# Compile and execute the mixed Nora/Rust example (Unix shell).
demo:
    mkdir -p target/demo
    cargo run --quiet -- examples/compact.nora > target/demo/compact.rs
    rustc --edition=2024 target/demo/compact.rs -o target/demo/compact
    ./target/demo/compact

# Print generated Rust for a Nora source file.
emit file="examples/basic.nora":
    cargo run --quiet -- "{{file}}"

# Install the optional tokenizer in an isolated environment.
setup-tokens:
    python3 -m venv target/token-env
    target/token-env/bin/python -m pip install -r requirements-bench.txt

# Measure syntax tokens; run just setup-tokens first.
tokens:
    target/token-env/bin/python scripts/count_tokens.py

# Build and verify the source package from a clean checkout.
package:
    cargo package
