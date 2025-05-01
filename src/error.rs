use std::process::exit;

#[derive(Debug)]
pub enum ErrorKind {
    Syntax,
    Runtime,
    Parse,
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl Error {
    // TODO: enhance API
    pub fn new(kind: ErrorKind, message: &str, line: usize, column: usize) -> Error {
        Error {
            kind,
            message: message.to_string(),
            line,
            column,
        }
    }
    fn report(&self, source_line: &str) -> String {
        let indicator = " ".repeat(self.column) + "^"; // Create a pointer to show the error pos
        format!(
            "{:?} Error on line {}, column {}:\n{}\n{}\n{}",
            self.kind, self.line, self.column, self.message, source_line, indicator
        )
    }
    pub fn print(&self, source_line: &str) {
        let err = self.report(source_line);
        eprintln!("{}", err);
        exit(65)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
