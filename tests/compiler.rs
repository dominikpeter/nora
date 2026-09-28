use std::{fs, process::Command};

#[test]
fn generated_rust_executes_transformations() {
    let source = "text(x:i)->s=str(x)\ncalc(a:i,b:i)->i=-(a+2)*b+17/2%3\nidentity(x:s)->s=x\ntype(match:i)->i=match\nconstant()->i=42\n";
    let rust = nora::compile(source).expect("valid Nora");
    let mixed = nora::compile(include_str!("../examples/compact.nora")).unwrap();
    let dir = std::env::temp_dir().join(format!("nora-test-{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("test.rs"), format!("{rust}\nfn main() {{ assert_eq!(nora_text(-42), \"-42\"); assert_eq!(nora_calc(3, 4), -18); assert_eq!(nora_identity(String::from(\"hello\")), \"hello\"); assert_eq!(nora_type(7), 7); assert_eq!(nora_constant(), 42); }}")).unwrap();
    let compiled = Command::new("rustc")
        .arg("--edition=2024")
        .arg(dir.join("test.rs"))
        .arg("-o")
        .arg(dir.join("test"))
        .output()
        .unwrap();
    assert!(
        compiled.status.success(),
        "{}",
        String::from_utf8_lossy(&compiled.stderr)
    );
    assert!(Command::new(dir.join("test")).status().unwrap().success());
    fs::write(dir.join("mixed.rs"), mixed).unwrap();
    let result = Command::new("rustc")
        .arg("--edition=2024")
        .arg(dir.join("mixed.rs"))
        .arg("-o")
        .arg(dir.join("mixed"))
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let result = Command::new(dir.join("mixed")).output().unwrap();
    assert!(result.status.success());
    assert_eq!(result.stdout, b"6\n");
    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn rejects_invalid_programs_at_source_locations() {
    for (source, message) in [
        ("f(x:i)->s=x", "return type"),
        ("f(x:s)->i=x+1", "requires i"),
        ("f(x:i)->i=y", "unknown parameter"),
        ("f(x:i,x:i)->i=x", "duplicate parameter"),
        ("f()->i=1\nf()->i=2", "duplicate function"),
        ("f(x:q)->i=1", "unknown type"),
        ("f()->i=1 garbage", "unexpected token"),
        ("f()->i=9223372036854775808", "out of range"),
        ("f()->i=1;", "unexpected character"),
        ("f()->i=(1+2", "expected ')'"),
        ("f()->i=", "expected expression"),
    ] {
        let error = nora::compile(source).expect_err(source);
        assert!(error.contains(message), "{source}: {error}");
        assert!(
            error.starts_with("line ") && error.contains(", column "),
            "{error}"
        );
    }
}

#[test]
fn accepts_blank_lines_and_crlf() {
    assert!(nora::compile("\r\n f( x : i ) -> s = str( x + 1 )\r\n\n").is_ok());
    assert_eq!(nora::compile("").unwrap(), "");
}

#[test]
fn compact_syntax_preserves_existing_rust_expansion() {
    for (compact, original) in [
        ("text(x:i)=$x", "text(x:i)->s=str(x)"),
        ("add(a,b:i)=a+b", "add(a:i,b:i)->i=a+b"),
        ("f(a,b:i,c:s)=$(a+b)", "f(a:i,b:i,c:s)->s=str(a+b)"),
        ("f(x:s)=$x", "f(x:s)->s=str(x)"),
        ("f()=42", "f()->i=42"),
    ] {
        assert_eq!(
            nora::compile(compact).unwrap(),
            nora::compile(original).unwrap()
        );
    }
    assert!(
        nora::compile("f(a,a:i)=a")
            .unwrap_err()
            .contains("duplicate parameter")
    );
    assert!(
        nora::compile("f(x:i)=$x+1")
            .unwrap_err()
            .contains("requires i")
    );
}

#[test]
fn rust_tail_is_preserved_byte_for_byte() {
    let rust = "// arbitrary Rust, including the section marker in a string\r\nfn main() { println!(\"%%rust\"); }\r\n";
    assert_eq!(nora::compile(&format!("%%rust\n{rust}")).unwrap(), rust);
    assert_eq!(
        nora::compile(&format!("f()=1\n%%rust\n{rust}")).unwrap(),
        format!("{}{rust}", nora::compile("f()->i=1").unwrap())
    );
}
