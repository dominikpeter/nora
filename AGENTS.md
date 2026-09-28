# Working on Nora

## Goal

Build an experimental AI-native representation of Rust: fewer model tokens,
full Rust expressiveness, and deterministic expansion without changing meaning.
Treat compact syntax coverage, Rust passthrough compatibility, and model success
as separate measurements. Full compact Rust coverage is a goal, not current status.

For writing or repairing Nora programs, read docs/ai-primer.md (`just ai`).
For unsupported syntax or repair details, read docs/ai-reference.md.

## Before changing code

Read README.md for supported syntax and limitations. The compiler is a small,
dependency-free Rust crate; keep changes incremental and preserve existing syntax.
For benchmark changes or model trials, read benchmarks/programming-rust/README.md.
For releases, read docs/release-0.1.0.md and inspect Cargo.toml packaging entries.

## Compiler changes

- Add a failing behavioral test before implementing new syntax or fixing a bug.
- Verify generated Rust with rustc and execute assertions against its behavior.
- Preserve precedence, types, ownership, borrowing, and evaluation order as
  constructs are added. Prefer explicit errors over guessing an interpretation.
- Keep verbatim Rust byte-preserving and document any compact/Rust boundary change.
- Measure complete programs using a named tokenizer before claiming savings.
  A symbol's isolated token count does not predict its cost in context.
- Keep profiles stable and versioned; tune on training tasks, evaluate on separate
  tasks. Keep ordinary Rust available when a feature has no compact spelling.

## Verification

From the repository root, `just check` runs the checks below. Use `just` to list
other recipes. Without just:

```sh
cargo test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
python3 -m unittest discover -s scripts -p 'test_*.py'
python3 scripts/bench_smoke.py
```

For syntax measurements, run `python3 scripts/count_tokens.py` with
`tiktoken==0.13.0` installed. Python/tokenizer tooling is optional for compiler users.

## Benchmark integrity

Score only fixed evaluator checks. Verify broken candidates fail. Keep source
provenance and licenses for imported examples. Label passthrough fixtures as
compatibility tests and report compact coverage separately. Source-token counts
exclude prompts, reasoning, and repair cost; report these limits.

The runner executes native code and is not a sandbox. Use a disposable isolated
environment for untrusted model submissions. Public development tests are not
hidden holdouts. Record model/version/settings and actual usage for model trials;
fixture results must never be presented as AI-generated results.

## Release hygiene

Keep generated reports, build artifacts, local agent state, and credentials out
of commits. Include this file in source packages. Before release, run verification,
`git diff --check`, and `cargo package --list`; verify the built package. Describe
current limitations in release notes. GitHub releases and crates.io publication
are separate actions; follow the user's requested destination.
