# Larger-program paired benchmark

This experiment tests the shorter Nora instruction on two larger library modules:

- **invoice**: twelve numeric/string helpers plus invoice aggregation and rendering.
  Compact Nora can express helpers; aggregation requires the Rust escape section.
- **inventory**: text parsing, validation, error precedence, overflow checking,
  duplicate merging, deterministic sorting, and reporting. The current compact
  subset cannot express most of this; Rust passthrough is expected.

These are original, specified tasks, not copied benchmark solutions. They are
larger than the initial five-expression pilot but still small modules, not full
applications. The invoice task deliberately exercises Nora's supported subset;
the inventory task probes its coverage limit. Treat them separately.

## Run

```sh
just setup-tokens
just bench-larger
# Or with a Python environment containing tiktoken:
python3 scripts/bench_larger.py --live --trials 2
```

Requires authenticated Codex CLI access. This makes eight real model calls and
executes native output locally. Use a disposable sandbox for untrusted candidates;
the temporary directory and CLI read-only generation mode do not sandbox execution.

Settings: gpt-6-astra, medium effort, two fresh sessions per language per task,
zero repairs. The model alias is explicit, but the underlying model snapshot is
not immutable. User configuration is ignored. Order alternates Rust/Nora then
Nora/Rust within each task. Output must be source only; fences are not stripped.
Tool use is audited and excludes a trial. Do not discard failed trials.

All task requirements are identical across languages. Nora alone receives the
103-token guide.txt. Five external test functions per task are appended only
after generation; the model receives neither checks nor reference implementations.
Checks include multiple inputs per function. Expected-test discovery guards
against accidentally disabling tests. These tests are public development fixtures,
not permanently hidden holdouts. Model sessions are instructed not to use tools;
this is a cooperative prompt-only trial, not adversarial filesystem isolation.

Reference implementations validate the evaluator before live trials. Mutation
checks prove wrong payable amounts and wrong threshold comparisons fail.

## Measurements

`target/larger-benchmark/report.json` contains task/guide/check hashes, CLI and
model settings, status, latency, source counts, prompt+source counts, Rust escape
usage, and model-reported input/output/reasoning/cache usage. Exact prompts,
responses, and local event logs are saved beside it.

Compare correctness first. Local o200k_base counts are a common text metric,
not necessarily the selected model's exact tokenizer. They exclude reasoning.
Provider usage includes CLI system context and reasoning where reported. Cached
input and reasoning output are subsets: do not add them to totals again.
Sum input_tokens + output_tokens for a total-usage comparison; this is a token
count, not a monetary cost estimate. Pricing, caching, and token types can differ.

No repairs occur in this experiment, so repair cost is zero by protocol, not a
claim that failures need no repair. Repeated trials are descriptive; two trials
per condition are insufficient for a strong statistical claim.

## Recorded run

The committed `results/` directory contains all eight outputs and prompts, the
frozen guide, model usage, hashes, and aggregates. All trials passed. Invoice
mean local prompt+source tokens: Rust 878.5, Nora 790. Inventory: Rust 1049,
Nora 1159.5. Full reported input+output means: invoice 20142.5 vs 20085.5;
inventory 20378.5 vs 20428.5. Overall full-token usage was nearly identical.

Both invoice Nora outputs used twelve compact helper definitions plus a Rust
tail. Both inventory Nora outputs were entirely Rust tails. The result supports
expanding compact coverage before claiming a general agent-cost advantage.
No production compiler or default primer changes were made for this experiment.

Rebuild the chart and summary with `python3 scripts/plot_larger_benchmark.py`.
