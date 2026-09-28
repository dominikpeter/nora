# Nora v0.1 language reference

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
