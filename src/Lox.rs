// Scanning Part one
use std::io;
use std::io::Write;
use std::path::Path;

// Core interpreter functionality
struct Lox {}

impl Lox {
    fn new() -> Self {
        Lox {}
    }
    /// Core execution method
    pub fn run(&mut self, source: &str) -> Result<(), String> {
        Ok(())
    }
}

/// run file which contains .lox source code
fn run_file<P: AsRef<Path>>(lox: &mut Lox, path: P) -> io::Result<()> {
    let content = std::fs::read_to_string(path);
    for line in content?.lines() {
        if let Err(e) = lox.run(&line) {
            eprintln!("Error {}", e);
        }
    }
    Ok(())
}
/// run a single prompt from the interactive shell
fn run_prompt(lox: &mut Lox) -> io::Result<()> {
    let stdin = io::stdin();
    let mut buffer = String::new();

    loop {
        print!("> ");
        std::io::stdout().flush()?;

        buffer.clear();
        stdin.read_line(&mut buffer)?;
        if buffer.trim().is_empty() {
            break;
        }

        if let Err(e) = lox.run(&buffer) {
            eprintln!("Error {}", e);
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let mut lox = Lox::new();
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        1 => run_prompt(&mut lox)?,
        2 => run_file(&mut lox, &args[1])?,
        _ => {
            eprintln!("Usage: lox [path]");
            std::process::exit(64);
        }
    }
    Ok(())
}
