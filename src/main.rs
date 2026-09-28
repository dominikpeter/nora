use std::{
    env, fs,
    io::{self, Write},
    process::ExitCode,
};

const AI_REFERENCE: &str = include_str!("../docs/ai-primer.md");
const AI_REFERENCE_FULL: &str = include_str!("../docs/ai-reference.md");
const USAGE: &str =
    "usage: nora <file.nora|file.rs> | --ai-reference | --ai-reference-full | --prompt <task.txt>";

fn run() -> Result<(), String> {
    let args: Vec<_> = env::args_os().skip(1).collect();
    let output = match args.as_slice() {
        [flag] if flag == "--ai-reference" => AI_REFERENCE.to_owned(),
        [flag] if flag == "--ai-reference-full" => AI_REFERENCE_FULL.to_owned(),
        [flag, path] if flag == "--prompt" => {
            let task =
                fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.to_string_lossy()))?;
            format!("{AI_REFERENCE}\n# Task\n\n{task}\n")
        }
        [path] if !path.to_string_lossy().starts_with("--") => {
            let source =
                fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.to_string_lossy()))?;
            if std::path::Path::new(path)
                .extension()
                .is_some_and(|ext| ext == "rs")
            {
                source
            } else {
                nora::compile(&source).map_err(|e| format!("{}: {e}", path.to_string_lossy()))?
            }
        }
        _ => return Err(USAGE.into()),
    };
    io::stdout()
        .lock()
        .write_all(output.as_bytes())
        .map_err(|e| e.to_string())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
