#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // single-character tokens
    LeftParen, RightParen, LeftBrace, RightBrace, Comma,
    Dot, Minus, Plus, Semicolon, Slash, Star,
    // one-two character tokens
    Bang, BangEqual, Equal, EqualEqual,
    Greater, GreaterEqual, Less, LessEqual,
    // literals
    Identifier, String, Number,
    // Keywords
    And, Class, Else, False, Fun, For, If, Print,
    Return, Or, Nil, Super, This, True, Var, While,
    Eof,
}
#[derive(Debug, Clone)]
pub struct Token {
    // size = 24 bytes (usize) + 1 byte (enum) +  variable size string
    value: String,   // lexeme
    kind: TokenKind, // type of the token
    line: usize,     // where token appears
    column: usize,   // column where token starts
    length: usize,   // size of the lexeme
}

impl Token {
    pub fn new(value: String, kind: TokenKind, line: usize, column: usize) -> Self {
        let length = value.len();
        Token {
            value,
            kind,
            line,
            column,
            length,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SlimToken {
    // 16 bytes (usize) + 1 byte (enum)
    kind: TokenKind,
    offset: usize,
    length: usize,
}
impl SlimToken {
    pub fn new(kind: TokenKind, offset: usize, length: usize) -> Self {
        SlimToken {
            kind,
            offset, // offset from the beginning of the source to the beginning of the lexeme
            length, // length of the lexeme
        }
    }
    /// Get the full lexeme to prevent space-allocation of the line, column, value;
    pub fn get_lexeme<'a>(&self, source: &'a str) -> &'a str {
        &source[self.offset..self.offset + self.length]
    }
}
