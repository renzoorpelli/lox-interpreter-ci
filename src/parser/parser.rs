use crate::error::Position;
use crate::{
    error::Error,
    parser::expr::{Expr, Literal},
    token::{Token, TokenKind},
};
#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr, String> {
        self.expression()
    }
    fn expression(&mut self) -> Result<Expr, String> {
        self.equality()
    }
    fn comma(&mut self) -> Result<Expr, String> {
        let mut expr = self.ternary()?;

        if self.match_token(&[TokenKind::Comma]) {
            let operator = self.previous().clone();
            let right = self.ternary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        Ok(expr)
    }

    fn ternary(&mut self) -> Result<Expr, String> {
        let expr = self.equality()?;

        if self.match_token(&[TokenKind::Question]) {
            let then_expr = self.expression()?;

            if !self.match_token(&[TokenKind::Colon]) {
                return Err("Expected ':' after then expression in ternary operator".to_string());
            }

            let else_expr = self.ternary()?; // right associative

            return Ok(Expr::Ternary {
                condition: Box::new(expr),
                then_expr: Box::new(then_expr),
                else_expr: Box::new(else_expr),
            });
        }
        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;

        while self.match_token(&[TokenKind::BangEqual, TokenKind::Equal]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }
    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;

        while self.match_token(&[
            TokenKind::Greater,
            TokenKind::GreaterEqual,
            TokenKind::Less,
            TokenKind::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;

        while self.match_token(&[TokenKind::Plus, TokenKind::Minus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;

        while self.match_token(&[TokenKind::Slash, TokenKind::Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if self.match_token(&[TokenKind::Bang, TokenKind::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;

            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, String> {
        if self.match_token(&[TokenKind::False]) {
            return Ok(Expr::Literal(Literal::Bool(false)));
        }

        if self.match_token(&[TokenKind::True]) {
            return Ok(Expr::Literal(Literal::Bool(true)));
        }

        if self.match_token(&[TokenKind::Nil]) {
            return Ok(Expr::Literal(Literal::Nil));
        }

        if self.match_token(&[TokenKind::Number]) {
            return Ok(Expr::Literal(Literal::Number(
                self.previous().lexeme.parse::<f64>().unwrap(),
            )));
        }

        if self.match_token(&[TokenKind::String]) {
            return Ok(Expr::Literal(Literal::String(
                self.previous().lexeme.clone(),
            )));
        }

        if self.match_token(&[TokenKind::LeftParen]) {
            let expr = self.expression()?;

            return match self.consume(TokenKind::RightParen, "Expected ')' after expression.") {
                Ok(_token) => Ok(Expr::Grouping {
                    expr: Box::new(expr),
                }),
                Err(error) => Err(error.message),
            };
        }
        Err(format!("Unexpected token: {:?}", self.peek()))
    }

    /// Check if the current token has any of the given types
    fn match_token(&mut self, kinds: &[TokenKind]) -> bool {
        for &kind in kinds {
            if self.check(kind) {
                self.advance();
                return true;
            }
        }
        false
    }

    /// Check if the current token is of the given type
    fn check(&self, kind: TokenKind) -> bool {
        !self.is_at_end() && self.peek().kind == kind
    }

    /// Consumes the current token and returns it
    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    /// Check if we have any tokens
    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenKind::Eof
    }

    /// Get the current token without consuming it
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    /// Get the token that you have consumed
    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    /// Consume a token of the expected type or return an error
    fn consume(&mut self, kind: TokenKind, message: &str) -> Result<Token, Error> {
        if self.check(kind) {
            Ok(self.advance())
        } else {
            let err_token = self.peek();
            Err(Error::parse(
                message,
                Position::new(err_token.line, err_token.column, err_token.offset),
            ))
        }
    }
}
