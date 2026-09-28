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
    python3 -m unittest discover -s scripts -p 'test_*.py'

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

# Print the compact reference to give an AI before it writes Nora.
ai:
    @cargo run --quiet -- --ai-reference

# Run three live Codex trials (uses model access; run setup-tokens first).
bench-ai:
    target/token-env/bin/python scripts/bench_ai_guide.py --live

# Regenerate the README graph from the committed live-pilot report.
graphs:
    python3 scripts/plot_benchmark.py
    python3 scripts/plot_larger_benchmark.py

# Print the embedded AI reference followed by a task file.
prompt task:
    @cargo run --quiet -- --prompt "{{task}}"

# Run paired larger-program trials (8 live model calls by default).
bench-larger trials="2":
    target/token-env/bin/python scripts/bench_larger.py --live --trials "{{trials}}"
