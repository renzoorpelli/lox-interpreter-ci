#[derive(Clone)]
pub enum TokenKind {
    String,     // strings
    Float,      // lox only uses floating point numbers
    Keyword,    //
    Operator,   // operators +=-()...
    Identifier, //
    EOF         // End of file
}
#[derive(Clone)]
pub struct Token {
    value: String,      // lexeme
    kind: TokenKind,    // type of the token
    line: usize,        // where token appears
    position: usize,    // absolute position in the source
    column: usize,      // column where token starts
}

impl Token {
    pub fn new(value: String, kind: TokenKind, line: usize, position: usize, column: usize) -> Self {
        Token {
            value,
            kind,
            line,
            position,
            column,
        }
    }
}
