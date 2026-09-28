# Nora

**An experiment in a new AI-native coding language: compact source that translates directly to Rust.**

Nora explores whether AI coding agents can read, write, and edit programs with
fewer tokens by using a small, predictable syntax. The compiler is written in
Rust and emits ordinary Rust source. Rust's compiler handles native compilation.

```text
add(a,b:i)=a+b
text(x:i)=$x
price(q,p,discount:i)=q*p-discount
```

This is an early prototype, not a production-ready language. Source-token savings
are measurable on the examples below; better AI coding cost and correctness are
still hypotheses to test.

## Intention

The goal is to give AI agents a concise way to express Rust programs while keeping
the translation deterministic and inspectable:

- Remove repeated boilerplate and type annotations where the meaning is clear.
- Choose syntax using actual tokenizer measurements, not character counts.
- Preserve Rust behavior as supported constructs expand into Rust.
- Keep access to ordinary Rust for features the compact language cannot express yet.
- Measure the entire coding loop: instructions, generation, errors, and repairs.

Nora does not need an AI model to compile. The AI writes Nora; a normal compiler
translates it.

```text
Nora source → parsing and type checking → Rust source → rustc → native code
```

## Nora vs. Rust

These examples work today. The Rust column shows equivalent code with ordinary
names for readability.

| Nora | Equivalent Rust |
| --- | --- |
| `add(a,b:i)=a+b` | `fn add(a: i64, b: i64) -> i64 { a + b }` |
| `text(x:i)=$x` | `fn text(x: i64) -> String { x.to_string() }` |
| `scale(x:i)=x*100` | `fn scale(x: i64) -> i64 { x * 100 }` |
| `identity(x:s)=x` | `fn identity(x: String) -> String { x }` |
| `price(q,p,discount:i)=q*p-discount` | `fn price(q: i64, p: i64, discount: i64) -> i64 { q * p - discount }` |

Explicit signatures are also supported:

```text
text(x:i)->s=str(x)
add(a:i,b:i)->i=a+b
```

The compiler actually emits public functions with `nora_` names and `v_`
parameter names to avoid Rust keyword collisions. For `text(x:i)=$x`:

```rust
pub fn nora_text(v_x: i64) -> String {
    (v_x).to_string()
}
```

## Install from source

You need Git and a recent stable Rust toolchain with Cargo and edition 2024
support. If Rust is not installed, follow [the Rust installation guide](https://www.rust-lang.org/tools/install).
The Nora compiler has no third-party Rust dependencies.

```sh
git clone https://github.com/dominikpeter/nora.git
cd nora
cargo install --path .
```

This installs the `nora` executable in Cargo's binary directory, normally
`~/.cargo/bin`. Make sure that directory is on your `PATH`.

If you already have this repository checked out, run `cargo install --path .`
from its root. You can also try everything without installing the executable
by replacing `nora` with `cargo run --quiet --`.

## Try it

From the repository root, compile and run the included example:

```sh
mkdir -p target/demo
nora examples/compact.nora > target/demo/compact.rs
rustc --edition=2024 target/demo/compact.rs -o target/demo/compact
./target/demo/compact
```

Expected output:

```text
6
```

The commands above use a Unix-style shell. On Windows, run the generated
`compact.exe` executable instead.

The example combines compact Nora functions with a Rust entry point:

```text
text(x:i)=$x
add(a,b:i)=a+b
%%rust
fn main() {
    let values = [1, 2, 3];
    let total: i64 = values.iter().sum();
    assert_eq!(nora_add(total, 4), 10);
    assert_eq!(nora_text(total), "6");
    println!("{}", nora_text(total));
}
```

To inspect generated Rust without saving or compiling it:

```sh
nora examples/basic.nora
```

A file containing only compact functions produces library source. Compile it
as a library, or add a Rust `main` as in the example above:

```sh
nora examples/basic.nora > target/demo/basic.rs
rustc --edition=2024 --crate-type lib target/demo/basic.rs -o target/demo/libbasic.rlib
```

## Syntax today

One compact function per line:

```text
name(parameter:type,...)->type=expression
```

| Construct | Meaning |
| --- | --- |
| `i` | Rust `i64` |
| `s` | Owned Rust `String` |
| `a,b:i` | Two integer parameters |
| `->i`, `->s` | Explicit return type; optional when Nora can infer it |
| `=expression` | Function body with an implicit return |
| `$x` or `str(x)` | Convert an integer or string to a string |
| `$(a+b)` | Convert the result of an expression to a string |
| `+ - * / %` | Integer arithmetic |
| `-x`, `(expression)` | Negation and grouping |

Blank lines and whitespace are accepted. Identifiers use ASCII letters,
digits, and underscores, and cannot begin with a digit.

Multiplication, division, and remainder bind more tightly than addition and
subtraction. Binary operators associate left to right. `$` binds at unary
precedence: `$x+1` is a type error; use `$(x+1)` instead.

Integer division truncates toward zero. Overflow and division by zero follow
the generated Rust's behavior, including build-dependent overflow checks.
Rust may reject invalid constant arithmetic during native compilation.

Nora checks parameter references, arithmetic types, and declared return types.
Errors include one-based line and column numbers. On a Nora compilation error,
the CLI exits unsuccessfully and emits no Rust.

## Rust compatibility and current limits

A line containing `%%rust` ends compact parsing. Everything after that line is
copied verbatim, with no closing delimiter. Use it for Rust functions, structs,
traits, macros, modules, or a `main` function. Rust code can call generated
`nora_` functions. The CLI also copies `.rs` files unchanged.

This provides access to full Rust source; it does **not** mean Nora's compact
syntax already covers the whole Rust language. Cargo still manages crates,
dependencies, editions, and build scripts. Nora does not replace Cargo.

Current limitations:

- Compact functions do not yet support user function calls, string literals,
  booleans, lists, loops, comments, imports, borrowing, or multiline bodies.
- Compact expressions cannot yet call arbitrary Rust functions.
- Errors in verbatim Rust and generated-name conflicts are diagnosed by `rustc`.
  Mapping those errors back to Nora source is not implemented yet.
- Integer literal magnitudes range from 0 to 9223372036854775807. The direct
  minimum-i64 literal is not supported because negation follows literal parsing.

## Token experiment

For the five examples in the comparison table, joined with newlines:

| Source form | Tokens |
| --- | ---: |
| Equivalent Rust shown above | 89 |
| Original Nora with explicit signatures | 49 |
| Current compact Nora | 41 |

Both `o200k_base` and `cl100k_base`, measured with `tiktoken==0.13.0`, give these
counts. That is **54% fewer source tokens than the Rust examples**, and **16%
fewer than the original Nora syntax**. Other tokenizers and programs may differ.

These counts exclude prompts, language instructions, reasoning, and repair
attempts. The Rust baseline uses ordinary names, not the compiler's longer
generated names. These are small syntax experiments, not evidence that Nora
reduces the total cost of solving real programming tasks.

To reproduce the measurements with Python installed:

```sh
python3 -m venv /tmp/nora-token-env
/tmp/nora-token-env/bin/python -m pip install tiktoken==0.13.0
/tmp/nora-token-env/bin/python scripts/count_tokens.py
```

The tokenizer is only needed for this experiment, not for compiling Nora.

## Development

With [just](https://github.com/casey/just) installed, run `just` to list commands:

```sh
just check         # tests, formatting, lint, and benchmark fixtures
just demo          # compile and run the example
just emit          # inspect generated Rust
just bench         # benchmark fixtures and JSON report
just setup-tokens  # install optional tokenizer into target/token-env
just tokens        # compare syntax token counts
just package       # verify a source package from a clean checkout
```

`just build`, `just install`, `just test`, `just lint`, and `just fmt` are also
available. The demo and tokenizer-environment recipes use Unix paths. The
underlying commands below work without just.


```sh
cargo test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
```

Tests compile generated Rust with `rustc` and execute it. They also check compact
syntax equivalence, diagnostics, CLI failures, and Rust passthrough.

- `src/lib.rs`: tokenizer, recursive-descent parser, type checks, and Rust emission.
- `src/main.rs`: CLI file input and stdout output.
- `examples/`: small Nora programs to try.
- `scripts/count_tokens.py`: reproducible source-token comparison.

The Rust library API is `nora::compile(&str) -> Result<String, String>`.
There is no separate AST or optimization IR yet.

Future iterations should expand useful Rust coverage and compare Nora against
Rust on varied tasks with isolated runs, held-out tests, and repeated trials.
Syntax changes should earn their place through measurable savings and reliable
behavior.

## References

- [Build a Compiler from Scratch](https://blog.sylver.dev/build-a-compiler-from-scratch-part-0-introduction): iterative compiler construction.
- [Dan Luu's language and token experiments](https://danluu.com/pl-tokens/): why source brevity alone does not establish lower agent cost.
- [Cython](https://github.com/cython/cython): source-to-source compilation and ecosystem interoperability.
- [Pythran](https://pythran.readthedocs.io/en/latest/) and [mypyc](https://mypyc.readthedocs.io/en/latest/): typed compilation approaches.

These projects are references, not Nora dependencies.

## Behavioral benchmarks

Run the first benchmark suite:

```sh
python3 scripts/bench_smoke.py
```

It checks GCD, character queues, generic ownership, interval ordering, and a
compact data transformation against independent assertions. Rust and Nora
fixtures are scored separately. Four tasks currently require Rust passthrough;
they test compatibility, not compact syntax savings. No AI model is called.

See [benchmark instructions](benchmarks/programming-rust/README.md) for scoring
model outputs, source provenance, failure tests, and the syntax-search protocol.
See [release notes](docs/release-0.1.0.md) for scope and remaining limits.
