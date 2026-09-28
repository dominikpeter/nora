use std::{
    env, fs,
    io::{self, Write},
    process::ExitCode,
};

fn run() -> Result<(), String> {
    let mut args = env::args_os().skip(1);
    let path = args.next().ok_or("usage: nora <file.nora>")?;
    if args.next().is_some() {
        return Err("usage: nora <file.nora>".into());
    }
    let source =
        fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.to_string_lossy()))?;
    let rust = if std::path::Path::new(&path)
        .extension()
        .is_some_and(|ext| ext == "rs")
    {
        source
    } else {
        nora::compile(&source).map_err(|e| format!("{}: {e}", path.to_string_lossy()))?
    };
    io::stdout()
        .lock()
        .write_all(rust.as_bytes())
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
