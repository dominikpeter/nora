# Can an AI use the Nora reference?

A small live pilot compares three conditions using fresh Codex CLI sessions:

1. Rust, no Nora guide.
2. Nora v0.1, no guide.
3. Nora v0.1 with docs/ai-reference.md.

Each condition gets the same five-function task. The guide is the intervention.
Outputs are compiled and run against five external behavioral tests covering
precedence, negative arithmetic, string conversion, and ownership. No repairs
are allowed. Tool-using responses are excluded. Raw responses are scored as-is;
Markdown fences or invented syntax cause failure instead of being silently fixed.

```sh
just setup-tokens
just bench-ai
```

Requires an authenticated Codex CLI. This invokes three real model calls and
executes generated native code locally. Use an isolated disposable environment
for untrusted model output. The temporary working directory is not a security
boundary. Prompts instruct the model to avoid tools, and events are audited;
this is a cooperative experiment, not an adversarially isolated evaluation.

Results, prompts, raw model events, and source are saved to target/ai-guide.
The report records CLI version, prompt/guide hashes, provider-reported usage,
source counts, local prompt counts, latency, and correctness. User configuration
is ignored; the CLI's default model is used. Model identity is not pinned in this
initial pilot, so results are descriptive and not a reproducible model ranking.

Guide tokens count as input overhead. CLI usage also includes its system context,
so standalone prompt counts differ from provider input counts. Source counts
exclude reasoning; provider usage is reported separately, without converting to
money or adding cached counts a second time.

These deliberately small tasks overlap concepts demonstrated in the guide. They
measure basic onboarding, not generalization to substantial Rust programs. One
trial per condition is a smoke experiment, not a statistical conclusion. They are
public tests, not permanently hidden holdouts. For stronger claims, freeze the
guide, pin the model, run repeated trials, and add unseen realistic tasks.

Offline evaluator checks are part of `just test`: correct Rust passes, wrong
precedence fails, invalid source fails, and disabled evaluator tests fail.

## Primer iteration

The initial full guide and its exact inputs/outputs are under results/initial.
A fourth live trial used docs/ai-primer.md, saved under results/primer. The guide
shrunk from 505 to 151 o200k_base tokens; all five checks still passed. The CLI now
embeds this primer by default. This is another single trial, not a controlled
multi-seed comparison. The new script accepts an explicit guide for experiments:

```sh
python3 scripts/bench_ai_guide.py --live --condition nora_with_guide --guide docs/ai-primer.md --report-dir target/ai-primer
```

The initial report remains frozen even when the default guide changes.
