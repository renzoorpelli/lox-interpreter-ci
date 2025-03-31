use crate::Token::{Token, TokenKind};
use std::io;
struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,   // start of the current lexeme
    current: usize, // current char position
    line: usize,    // current line
    column: usize,  // current column
}

impl Scanner {
    pub fn new(
        source: String,
        tokens: Vec<Token>,
        start: usize,
        current: usize,
        line: usize,
        column: usize,
    ) -> Self {
        Scanner {
            source,
            tokens,
            start,
            current,
            line,
            column,
        }
    }
    fn is_at_the_end(&self) -> bool {
        self.current >= self.source.len()
    }

    /// this method will scan the source code and return all the tokens
    pub fn get_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_the_end() {
            self.start = self.current;
            self.scan_token();
        }

        // push EOF token to the vector
        self.tokens.push(Token::new(
            String::from("EOF"),
            TokenKind::EOF,
            self.line,
            self.start,
            self.column,
        ));

        self.tokens.clone()
    }

    fn add_token(&mut self, kind: TokenKind) {
        let value = &self.source[self.start..self.current].to_string();
        self.tokens.push(Token::new(
            value.to_string(),
            kind,
            self.line,
            self.start,
            self.column,
        ));
    }
    fn scan_token(&mut self) {}
}
