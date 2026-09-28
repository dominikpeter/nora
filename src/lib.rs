//! A small, dependency-free Nora-to-Rust compiler.
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, PartialEq)]
enum Type {
    Int,
    String,
}
impl Type {
    fn rust(self) -> &'static str {
        match self {
            Self::Int => "i64",
            Self::String => "String",
        }
    }
}

struct Token<'a> {
    text: &'a str,
    column: usize,
}
struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    pos: usize,
    line: usize,
    end: usize,
    params: HashMap<String, Type>,
}
struct Expr {
    code: String,
    ty: Type,
}

impl<'a> Parser<'a> {
    fn new(source: &'a str, line: usize) -> Result<Self, String> {
        let mut tokens = Vec::new();
        let mut chars = source.char_indices().peekable();
        while let Some((start, c)) = chars.next() {
            if c.is_whitespace() {
                continue;
            }
            let mut end = start + c.len_utf8();
            if c.is_ascii_alphabetic() || c == '_' {
                while let Some(&(index, next)) = chars.peek() {
                    if !(next.is_ascii_alphanumeric() || next == '_') {
                        break;
                    }
                    chars.next();
                    end = index + 1;
                }
            } else if c.is_ascii_digit() {
                while let Some(&(index, next)) = chars.peek() {
                    if !next.is_ascii_digit() {
                        break;
                    }
                    chars.next();
                    end = index + 1;
                }
            } else if c == '-' && chars.peek().is_some_and(|&(_, c)| c == '>') {
                chars.next();
                end += 1;
            } else if !"():,=+-*/%$".contains(c) {
                return Err(format!(
                    "line {line}, column {}: unexpected character {c:?}",
                    source[..start].chars().count() + 1
                ));
            }
            tokens.push(Token {
                text: &source[start..end],
                column: source[..start].chars().count() + 1,
            });
        }
        Ok(Self {
            tokens,
            pos: 0,
            line,
            end: source.chars().count() + 1,
            params: HashMap::new(),
        })
    }
    fn peek(&self) -> &str {
        self.tokens.get(self.pos).map_or("", |t| t.text)
    }
    fn error(&self, message: &str) -> String {
        format!(
            "line {}, column {}: {message}",
            self.line,
            self.tokens.get(self.pos).map_or(self.end, |t| t.column)
        )
    }
    fn take(&mut self, text: &str) -> bool {
        if self.peek() == text {
            self.pos += 1;
            true
        } else {
            false
        }
    }
    fn expect(&mut self, text: &str) -> Result<(), String> {
        if self.take(text) {
            Ok(())
        } else {
            Err(self.error(&format!("expected '{text}'")))
        }
    }
    fn name(&mut self) -> Result<String, String> {
        if !self
            .peek()
            .starts_with(|c: char| c.is_ascii_alphabetic() || c == '_')
        {
            return Err(self.error("expected identifier"));
        }
        let name = self.peek().to_owned();
        self.pos += 1;
        Ok(name)
    }
    fn ty(&mut self) -> Result<Type, String> {
        let ty = match self.peek() {
            "i" => Type::Int,
            "s" => Type::String,
            _ => return Err(self.error("unknown type; expected i or s")),
        };
        self.pos += 1;
        Ok(ty)
    }
    fn function(&mut self) -> Result<(String, String), String> {
        let name = self.name()?;
        self.expect("(")?;
        let mut params = Vec::new();
        if !self.take(")") {
            loop {
                let mut group = vec![self.name()?];
                while self.take(",") {
                    group.push(self.name()?);
                }
                self.expect(":")?;
                let ty = self.ty()?;
                for param in group {
                    if self.params.insert(param.clone(), ty).is_some() {
                        return Err(self.error("duplicate parameter"));
                    }
                    params.push(format!("v_{param}: {}", ty.rust()));
                }
                if self.take(")") {
                    break;
                }
                self.expect(",")?;
            }
        }
        let declared = if self.take("->") {
            Some(self.ty()?)
        } else {
            None
        };
        self.expect("=")?;
        let expression = self.expr(0)?;
        if self.pos != self.tokens.len() {
            return Err(self.error("unexpected token after expression"));
        }
        let ty = declared.unwrap_or(expression.ty);
        if ty != expression.ty {
            return Err(self.error("return type does not match expression"));
        }
        let code = format!(
            "pub fn nora_{name}({}) -> {} {{\n    {}\n}}\n",
            params.join(", "),
            ty.rust(),
            expression.code
        );
        Ok((name, code))
    }
    fn expr(&mut self, min: u8) -> Result<Expr, String> {
        let mut left = self.atom()?;
        loop {
            let precedence = match self.peek() {
                "+" | "-" => 1,
                "*" | "/" | "%" => 2,
                _ => break,
            };
            if precedence < min {
                break;
            }
            let op = self.peek().to_owned();
            self.pos += 1;
            let right = self.expr(precedence + 1)?;
            if left.ty != Type::Int || right.ty != Type::Int {
                return Err(self.error("arithmetic requires i operands"));
            }
            left = Expr {
                code: format!("({} {op} {})", left.code, right.code),
                ty: Type::Int,
            };
        }
        Ok(left)
    }
    fn atom(&mut self) -> Result<Expr, String> {
        if self.take("$") {
            let value = self.atom()?;
            return Ok(Expr {
                code: format!("({}).to_string()", value.code),
                ty: Type::String,
            });
        }
        if self.take("-") {
            let value = self.atom()?;
            if value.ty != Type::Int {
                return Err(self.error("negation requires i"));
            }
            return Ok(Expr {
                code: format!("(-{})", value.code),
                ty: Type::Int,
            });
        }
        if self.take("(") {
            let value = self.expr(0)?;
            self.expect(")")?;
            return Ok(value);
        }
        if self.peek().starts_with(|c: char| c.is_ascii_digit()) {
            let number = self
                .peek()
                .parse::<i64>()
                .map_err(|_| self.error("integer out of range"))?;
            self.pos += 1;
            return Ok(Expr {
                code: format!("{number}_i64"),
                ty: Type::Int,
            });
        }
        if self
            .peek()
            .starts_with(|c: char| c.is_ascii_alphabetic() || c == '_')
        {
            let name = self.name()?;
            if name == "str" && self.take("(") {
                let value = self.expr(0)?;
                self.expect(")")?;
                return Ok(Expr {
                    code: format!("({}).to_string()", value.code),
                    ty: Type::String,
                });
            }
            let ty = *self
                .params
                .get(&name)
                .ok_or_else(|| self.error(&format!("unknown parameter '{name}'")))?;
            return Ok(Expr {
                code: format!("v_{name}"),
                ty,
            });
        }
        Err(self.error("expected expression"))
    }
}

/// Compile newline-separated Nora functions into a Rust library source file.
/// Errors include one-based source line and column numbers.
pub fn compile(source: &str) -> Result<String, String> {
    let mut output = String::new();
    let mut names = HashSet::new();
    let mut offset = 0;
    for (index, line) in source.split_inclusive('\n').enumerate() {
        offset += line.len();
        if line.trim() == "%%rust" {
            output.push_str(&source[offset..]);
            break;
        }
        if line.trim().is_empty() {
            continue;
        }
        let mut parser = Parser::new(line, index + 1)?;
        let (name, code) = parser.function()?;
        if !names.insert(name) {
            return Err(parser.error("duplicate function"));
        }
        output.push_str(&code);
    }
    Ok(output)
}
