#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    Syntax,
    Runtime,
    Parse,
    Type,
}

#[derive(Debug, Clone)]
pub struct Position {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

impl Position {
    pub fn new(line: usize, column: usize, offset: usize) -> Self {
        Self {
            line,
            column,
            offset,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: String,
    pub position: Position,
    pub help: Option<String>,
}

impl Error {
    fn new(kind: ErrorKind, message: impl Into<String>, position: Position) -> Self {
        Self {
            kind,
            message: message.into(),
            position,
            help: None,
        }
    }
    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }
    pub fn syntax(message: impl Into<String>, position: Position) -> Self {
        Self::new(ErrorKind::Syntax, message, position)
    }

    pub fn runtime(message: impl Into<String>, position: Position) -> Self {
        Self::new(ErrorKind::Runtime, message, position)
    }

    pub fn parse(message: impl Into<String>, position: Position) -> Self {
        Self::new(ErrorKind::Parse, message, position)
    }

    pub fn type_error(message: impl Into<String>, position: Position) -> Self {
        Self::new(ErrorKind::Type, message, position)
    }
    pub fn unexpected_token(expected: &str, found: &str, position: Position) -> Self {
        Self::syntax(
            format!("Expected '{}', found '{}'", expected, found),
            position,
        )
        .with_help(format!("Try using '{}' instead", expected))
    }
    pub fn undefined_variable(name: &str, position: Position) -> Self {
        Self::runtime(format!("Undefined variable '{}'", name), position)
            .with_help("Make sure the variable is declared before use")
    }
    pub fn division_by_zero(position: Position) -> Self {
        Self::runtime("Division by zero", position).with_help("Ensure the denominator is not zero")
    }
    pub fn invalid_operand_types(op: &str, left: &str, right: &str, position: Position) -> Self {
        Self::runtime(
            format!("Invalid operand types for {}: {} and {}", op, left, right),
            position,
        )
        .with_help(&format!("The {} operator requires compatible types", op))
    }
}
