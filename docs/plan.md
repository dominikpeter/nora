# Nora first compiler

Approved scope: start small; compile compact typed functions to Rust.

Design: a dependency-free Rust library and CLI. Parse one function per line,
check expression types, emit a Rust library on stdout. Types: i (i64), s
(String). Expressions: parameters, integer literals, parentheses, unary minus,
+ - * / %, and str(expression). Integer arithmetic follows Rust semantics.
No user function calls, lists, borrowing, strings literals, or inference yet.
Generated identifiers use nora_ for functions and v_ for parameters to avoid
Rust keyword collisions. Errors include source line and column.

Implementation:
1. Add execution tests: arithmetic precedence, conversion, string identity,
   multiple functions; negative tests for syntax, names, and types.
2. Implement lexer, recursive descent parser with type checking, Rust emitter.
3. Add CLI, example and usage guide; test CLI failures and output.
4. Run cargo test, cargo fmt --check, and cargo clippy with warnings denied.

Token efficiency and an improvement loop remain future experiments.
