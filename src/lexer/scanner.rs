use crate::error::{Error, ErrorKind, Result};
use crate::token::{Token, TokenKind};
use lazy_static::lazy_static;
use std::collections::HashMap;

// static code initialized at runtime
lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenKind> = {
        let mut m = HashMap::new();
        m.insert("and", TokenKind::And);
        m.insert("class", TokenKind::Class);
        m.insert("else", TokenKind::Else);
        m.insert("false", TokenKind::False);
        m.insert("for", TokenKind::For);
        m.insert("fun", TokenKind::Fun);
        m.insert("if", TokenKind::If);
        m.insert("nil", TokenKind::Nil);
        m.insert("or", TokenKind::Or);
        m.insert("print", TokenKind::Print);
        m.insert("return", TokenKind::Return);
        m.insert("super", TokenKind::Super);
        m.insert("true", TokenKind::True);
        m.insert("var", TokenKind::Var);
        m.insert("while", TokenKind::While);
        m
    };
}
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
    pub fn get_tokens(&mut self) -> Result<Vec<Token>> {
        while !self.is_at_the_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => {}
                Err(e) => return Err(e),
            };
        }
        // push EOF token to the vector
        self.tokens.push(Token::new(
            String::from(""),
            TokenKind::Eof,
            self.line,
            self.column,
        ));
        Ok(self.tokens.clone())
    }

    fn add_token(&mut self, kind: TokenKind, value: Option<String>) {
        let lexeme = match value {
            Some(value) => value,
            None => self.source[self.start..self.current].to_string(),
        };
        self.tokens
            .push(Token::new(lexeme, kind, self.line, self.column));
    }

    fn scan_token(&mut self) -> Result<()> {
        match self.advance() {
            '+' => self.add_token(TokenKind::Plus, None),
            '-' => self.add_token(TokenKind::Minus, None),
            '*' => self.add_token(TokenKind::Star, None),
            '/' => match self.peek_match('/') {
                true => {
                    //  A comment goes until the end of the line
                    while self.peek() != '\n' && !self.is_at_the_end() {
                        self.advance();
                    }
                }
                false => self.add_token(TokenKind::Slash, None),
            },
            '(' => self.add_token(TokenKind::LeftParen, None),
            ')' => self.add_token(TokenKind::RightParen, None),
            '{' => self.add_token(TokenKind::LeftBrace, None),
            '}' => self.add_token(TokenKind::RightBrace, None),
            ',' => self.add_token(TokenKind::Comma, None),
            '.' => self.add_token(TokenKind::Dot, None),
            '!' => match self.peek_match('=') {
                true => self.add_token(TokenKind::BangEqual, None),
                false => self.add_token(TokenKind::Bang, None),
            },
            '>' => match self.peek_match('=') {
                true => self.add_token(TokenKind::GreaterEqual, None),
                false => self.add_token(TokenKind::Greater, None),
            },
            '<' => match self.peek_match('=') {
                true => self.add_token(TokenKind::LessEqual, None),
                false => self.add_token(TokenKind::Less, None),
            },
            '=' => match self.peek_match('=') {
                true => self.add_token(TokenKind::EqualEqual, None),
                false => self.add_token(TokenKind::Equal, None),
            },
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => self.line += 1,               // move line
            '"' => self.handle_string_literal()?, // return early error
            'o' => {
                if self.peek_match('r') {
                    self.add_token(TokenKind::Or, None);
                }
            }
            _ => {
                if self.peek().is_ascii_digit() {
                    self.handle_number_literal();
                } else if self.peek().is_ascii_alphabetic() {
                    self.handle_identifier();
                }
                Error::new(
                    ErrorKind::Parse,
                    "Unexpected character.",
                    self.line,
                    self.column,
                );
            }
        }
        Ok(())
    }

    /// this method will consume the next character of the source by incrementing the position by one
    fn advance(&mut self) -> char {
        let c = self.source[self.current..].chars().next().unwrap();
        self.current += 1;
        c
    }

    /// this method will peek the next character but NOT consume the toke => Lookahead
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source[self.current + 1..].chars().next().unwrap()
    }

    /// this method will peek the current char but NOT consume the token => Lookahead.
    fn peek(&self) -> char {
        if self.is_at_the_end() {
            '\0';
        }
        self.source[self.current..].chars().next().unwrap()
    }

    /// peek match will check if the given `char` is the same as the next one then return true and update the position, otherwise false
    fn peek_match(&mut self, next: char) -> bool {
        if self.is_at_the_end() || self.peek() != next {
            false;
        }
        self.current += 1;
        true
    }

    /// this method will iterate through the lexeme, then it will parse the lexeme to find a number-token
    fn handle_number_literal(&mut self) {
        while self.peek().is_ascii_digit() && !self.is_at_the_end() {
            self.advance();
        }
        // decimal part
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
        }
        let value = self.source[self.start..self.current]
            .to_string()
            .parse::<f64>()
            .unwrap();

        self.add_token(TokenKind::Number, Some(value.to_string()));
    }

    /// this method will iterate through the lexeme, then it will parse the lexeme to find a string-token
    fn handle_string_literal(&mut self) -> Result<()> {
        while self.peek() != '"' && !self.is_at_the_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_the_end() {
            Error::new(
                ErrorKind::Syntax,
                "Unterminated string literal",
                self.line,
                self.column,
            );
        }
        self.advance(); // the closing " of the string literal
        // Trim the surrounding quotes
        let value = self.source[(self.start + 1)..(self.current + 1)].to_string();
        self.add_token(TokenKind::String, Some(value));
        Ok(())
    }

    /// this method will be used to handle the type-identifier token
    fn handle_identifier(&mut self) {
        while self.peek().is_alphanumeric() && !self.is_at_the_end() {
            self.advance();
        }

        let text = self.source[self.start..self.current].trim();
        let token_kind = KEYWORDS.get(text).cloned().unwrap_or(TokenKind::Identifier);

        self.add_token(token_kind, None);
    }
    fn is_alphanumeric(c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }
    fn is_alphabetic(c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }
}
