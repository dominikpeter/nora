# Write Nora v0.1

Return source only. One compact function per line:
`name(a,b:i,x:s)->i=expression`. The return annotation is optional.
`i` = Rust i64; `s` = owned String. `a,b:i` gives both parameters type i.
Expressions: parameter names, decimal integer literals, parentheses, unary -, 
integer + - * / %, and `$expression` (string conversion at unary precedence).
Use `$(a+b)` to convert a whole expression. `str(a+b)` is equivalent.
Normal arithmetic precedence; integer division truncates toward zero.
Positive literal magnitudes must fit i64. Explicit return types are checked.
No implicit clones or numeric coercions. Returning a string parameter moves it.

```nora
add(a,b:i)=a+b
label(x:i)=$x
adjust(x,delta:i)=x+delta
identity(x:s)=x
text_sum(a,b:i)=$(a+b)
```

These generate public Rust functions named `nora_add`, `nora_label`, etc.
Parameters gain `v_` in generated Rust. Use ordinary parameter names in Nora.

Compact syntax has no calls to user functions, string literals, booleans, lists,
branches, loops, comments, imports, generics, borrowing, or multiline bodies.
For those features, write a line `%%rust` then ordinary Rust for the rest of the
file. There is no closing marker and no return to compact syntax. That tail is
copied exactly; use the required Rust names and visibility yourself. A whole
file may use this fallback. Put any Rust crate-level attributes before Rust
items (using an all-Rust tail if necessary).

```nora
double(x:i)=x*2
%%rust
fn main() { assert_eq!(nora_double(3), 6); }
```

Check: `nora file.nora > file.rs`, then `rustc --edition=2024 --crate-type lib
file.rs` for functions, or `rustc --edition=2024 file.rs` for a program with main.
Nora reports line/column errors; rustc checks emitted Rust. Fix the source and
repeat. `$x+1` mixes String and integer: write `$(x+1)` if conversion should be
last. Unknown parameters must match the signature. Unsupported syntax belongs
in the Rust tail; preserve requested behavior instead of inventing shorthand.
