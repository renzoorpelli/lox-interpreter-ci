use crate::Token::{Token, TokenKind};
use std::process::exit;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,   // points to the first character of the lexeme => offset
    current: usize, // points at the character currently being considered => offset
    line: usize,    // track what source line current is on.
    column: usize,  // current column
}

impl Scanner {
    pub fn new(source: String, tokens: Vec<Token>, column: usize) -> Self {
        Scanner {
            source,
            tokens,
            start: 0,
            current: 0,
            line: 1,
            column,
        }
    }
    /// method used to check all the characters were consumed
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
            String::from(""),
            TokenKind::Eof,
            self.line,
            self.column,
        ));

        self.tokens.clone()
    }

    fn add_token(&mut self, kind: TokenKind) {
        let value = &self.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::new(value.to_string(), kind, self.line, self.column));
    }
    fn scan_token(&mut self) {
        match self.move_next() {
            '+' => self.add_token(TokenKind::Plus),
            '-' => self.add_token(TokenKind::Minus),
            '*' => self.add_token(TokenKind::Star),
            '/' => match self.peek_match('/') {
                true => {
                    //  A comment goes until the end of the line
                    while self.peek() != '\n' && !self.is_at_the_end() {
                        self.move_next();
                    }
                }
                false => self.add_token(TokenKind::Slash),
            },
            '(' => self.add_token(TokenKind::LeftParen),
            ')' => self.add_token(TokenKind::RightParen),
            '{' => self.add_token(TokenKind::LeftBrace),
            '}' => self.add_token(TokenKind::RightBrace),
            ',' => self.add_token(TokenKind::Comma),
            '.' => self.add_token(TokenKind::Dot),
            '!' => match self.peek_match('=') {
                true => self.add_token(TokenKind::BangEqual),
                false => self.add_token(TokenKind::Bang),
            },
            '>' => match self.peek_match('=') {
                true => self.add_token(TokenKind::GreaterEqual),
                false => self.add_token(TokenKind::Greater),
            },
            '<' => match self.peek_match('=') {
                true => self.add_token(TokenKind::LessEqual),
                false => self.add_token(TokenKind::Less),
            },
            '=' => match self.peek_match('=') {
                true => self.add_token(TokenKind::EqualEqual),
                false => self.add_token(TokenKind::Equal),
            },
            _ => self.print_error("unexpected character"),
        }
    }

    /// this method will consume the next character of the source by incrementing the position by one
    fn move_next(&mut self) -> char {
        let c = self.source[self.current..].chars().next().unwrap();
        self.current += 1;
        c
    }

    /// this method will peek the current char but NOT consume the token => Lookahead.
    fn peek(&self) -> char {
        if self.is_at_the_end() {
            '\0'
        }
        self.source[self.current..].chars().next().unwrap()
    }

    /// this method will get the current character of the source
    fn get_current(&self) -> char {
        self.source.chars().nth(self.current).unwrap()
    }

    /// peek match will check if the given `char` is the same as the next one then return true and update the position, otherwise false
    fn peek_match(&mut self, next: char) -> bool {
        if self.is_at_the_end() || self.get_current() != next {
            false;
        }
        self.current += 1;
        true
    }

    /// helper method to generate a string with a comprehensive log error message for the user
    fn error_details(&self, message: &str) -> String {
        let line_content = self.source.lines().nth(self.line - 1).unwrap_or("");
        let indicator = " ".repeat(self.column) + "^"; // Create a pointer to show the error pos

        format!(
            "Error on line {}, column {}:\n{}\n{}\n{}",
            self.line, self.column, line_content, indicator, message
        )
    }

    /// this method will print an error and ends the process
    fn print_error(&self, message: &str) {
        eprintln!("{}", self.error_details(message));
        exit(65)
    }
}
