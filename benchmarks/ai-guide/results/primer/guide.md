Nora v0.1 → Rust. Emit source only.
One function/line: name(a,b:i,x:s)=expr. i=i64; s=String.
Return type inferred; optional ->i or ->s before =.
Expressions: parameters, i64 literals, (), unary -, + - * / %.
Rust arithmetic precedence. $(expr) converts to String. Strings move.
add(a,b:i)=a+b
label(a,b:i)=$(a+b)
identity(x:s)=x
Generated names: pub fn nora_NAME; parameters prefixed v_.
Other features: line %%rust then exact Rust to EOF; use required Rust names.
No compact user calls, branches, loops, collections, borrowing, or string literals.
