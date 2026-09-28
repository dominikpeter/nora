# Nora benchmark v1

Goal: full Rust expressiveness with a cheaper representation for AI agents,
without changing program meaning. Compiler preservation and model correctness
are separate requirements. No model is guaranteed to generate correct code.

## Run now

From the repository root (Rust and Python 3 required):

```sh
python3 scripts/bench_smoke.py
python3 -m unittest discover -s scripts -p 'test_benchmark.py'
```

The smoke runner builds Nora and scores five Rust references and five Nora
fixtures. Four fixtures use verbatim Rust; only transform uses compact Nora.
These are compatibility checks, NOT model results or evidence of savings on
queues and traits. JSON is saved to `target/benchmarks/smoke.json`.

Install tiktoken 0.13.0 to add source-token counts; otherwise counts are null.
Counts are source-only. Reference files include upstream comments and tests,
so their counts must not be used as a fair generated-code token baseline.

## Cases

| Task | Behavior | Compact Nora today |
| --- | --- | --- |
| gcd | Positive u64 GCD, symmetry, zero rejection, boundary values | Unsupported; Rust section required |
| queue | FIFO, Unicode, empty/reuse, split representation | Unsupported; Rust section required |
| generic_queue | FIFO with owned non-Clone values | Unsupported; Rust section required |
| interval | Generic partial ordering, equality, overlap, touching boundaries | Unsupported; Rust section required |
| transform | Integer prices and decimal string conversion | Supported |

Each task has a model-facing specification under `prompts/`. Tests under
`checks/` are independently written and appended by the scorer. The scorer
runs only those tests, verifies expected test discovery, and rejects compilation
errors, missing submissions, failed assertions, and timeouts. Runner tests
verify that deliberately wrong GCD, FIFO ordering, interval ordering, and
arithmetic implementations fail, as do disabled tests and infinite loops.

## Score model submissions

Save model outputs as `gcd.rs`, `queue.rs`, `interval.rs`, `generic_queue.rs`,
and `transform.rs` in a candidate directory, or use `.nora` for Nora trials.
Submit source only, without Markdown fences. Preserve the specified Rust API.

```sh
cargo build
python3 scripts/benchmark.py --candidates /path/to/rust-outputs --language rs --model model-name --report target/benchmarks/model-rust.json
python3 scripts/benchmark.py --candidates /path/to/nora-outputs --language nora --model model-name --report target/benchmarks/model-nora.json
```

`--model` is a report label; this command does not call a model. Each candidate
runs in a new temporary directory with a ten-second timeout per subprocess.
This executes native code locally; temporary directories are not a security
sandbox. Use a disposable container for untrusted submissions. The checks are
not designed to resist deliberately malicious programs.

These are public development tests, not hidden holdouts. For honest model
trials, give the model only task prompts plus the frozen Nora syntax guide,
withhold reference implementations and evaluator files, and score outside its
workspace. Run Rust and Nora with the same model version, effort, tools,
repair budget, and repeated trials. Record API input/output usage, language
instructions, repair cost, and latency separately; source counts alone do not
measure agent cost. Do not compare different prompts or include upstream test
code in one side's token count.

## Source provenance

Four Rust reference files are copied unchanged from
https://github.com/ProgrammingRust/examples at commit
`ce1eb55d7ecf69450202587b11ad3ca171b875e3`:

- gcd/src/main.rs → reference/gcd.rs
- queue/src/lib.rs → reference/queue.rs
- generic-queue/src/lib.rs → reference/generic_queue.rs
- interval/src/lib.rs → reference/interval.rs

Authors: Jim Blandy, Jason Orendorff, and Leonora Tindall. The upstream MIT
license is retained in LICENSE-MIT. The transform task and checks are original
to this project. Larger benchmarks should be added with pinned provenance,
clear licensing, independent behavioral checks, and deterministic execution.

The rust-examples GitHub topic was reviewed for discovery. No additional topic
repository was imported: the first suite uses the already selected, licensed
Programming Rust examples. Star ranking is not a quality metric for tests.

## Syntax search protocol

1. Freeze behavior and represent programs independently of spelling.
2. Enumerate supported syntax variants; measure complete programs with a named,
   versioned tokenizer. Use greedy search or beam search over grammar choices.
3. Reject ambiguity and any candidate that changes the generated Rust meaning.
4. Choose one stable profile using a training corpus; freeze it before scoring.
5. Evaluate held-out programs and repeated model trials. Include the cost of
   explaining the profile. Track Rust-escape usage separately.

Do not specialize a syntax profile to each test answer. A profile wins only
when token savings persist without reducing correctness. A tokenizer-only
minimum is not necessarily easiest for a model to produce.
