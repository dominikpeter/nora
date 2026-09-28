use std::process::Command;

#[test]
fn emits_rust_for_example() {
    let output = Command::new(env!("CARGO_BIN_EXE_nora"))
        .arg("examples/basic.nora")
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("pub fn nora_text"));
}

#[test]
fn reports_usage_and_io_errors() {
    for args in [vec![], vec!["does-not-exist.nora"], vec!["a", "b"]] {
        let output = Command::new(env!("CARGO_BIN_EXE_nora"))
            .args(args)
            .output()
            .unwrap();
        assert!(!output.status.success());
        assert!(output.stdout.is_empty());
        assert!(!output.stderr.is_empty());
    }
}

#[test]
fn invalid_source_produces_no_partial_output() {
    let path = std::env::temp_dir().join(format!("nora-invalid-{}.nora", std::process::id()));
    std::fs::write(&path, "ok()->i=1\nbad()->s=2").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_nora"))
        .arg(&path)
        .output()
        .unwrap();
    std::fs::remove_file(path).unwrap();
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains("line 2, column"));
}

#[test]
fn rust_files_pass_through_unchanged() {
    let path = std::env::temp_dir().join(format!("nora-pass-{}.rs", std::process::id()));
    let source = "fn main() { println!(\"hello\"); }\r\n";
    std::fs::write(&path, source).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_nora"))
        .arg(&path)
        .output()
        .unwrap();
    std::fs::remove_file(path).unwrap();
    assert!(output.status.success());
    assert_eq!(output.stdout, source.as_bytes());
}

#[test]
fn embedded_guide_and_task_prompt_work_outside_the_repository() {
    let dir = std::env::temp_dir().join(format!("nora-prompt-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let reference = Command::new(env!("CARGO_BIN_EXE_nora"))
        .arg("--ai-reference")
        .current_dir(&dir)
        .output()
        .unwrap();
    assert!(reference.status.success());
    assert_eq!(reference.stdout, include_bytes!("../docs/ai-primer.md"));
    let full = Command::new(env!("CARGO_BIN_EXE_nora"))
        .arg("--ai-reference-full")
        .current_dir(&dir)
        .output()
        .unwrap();
    assert!(full.status.success());
    assert_eq!(full.stdout, include_bytes!("../docs/ai-reference.md"));
    std::fs::write(dir.join("task.txt"), "Create integer addition.").unwrap();
    let prompt = Command::new(env!("CARGO_BIN_EXE_nora"))
        .args(["--prompt", "task.txt"])
        .current_dir(&dir)
        .output()
        .unwrap();
    assert!(prompt.status.success());
    let prompt = String::from_utf8(prompt.stdout).unwrap();
    assert!(prompt.starts_with(include_str!("../docs/ai-primer.md")));
    assert!(prompt.ends_with("Create integer addition.\n"));
    for args in [
        vec!["--prompt"],
        vec!["--prompt", "missing.txt"],
        vec!["--ai-reference", "extra"],
    ] {
        let result = Command::new(env!("CARGO_BIN_EXE_nora"))
            .args(args)
            .current_dir(&dir)
            .output()
            .unwrap();
        assert!(!result.status.success());
        assert!(result.stdout.is_empty());
    }
    std::fs::remove_dir_all(dir).unwrap();
}
