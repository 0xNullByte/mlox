use std::io::Error;

use crate::{
    token::{Object, Token, TokenType},
    Expr,
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) {
        let expr = self.expression();
        match expr {
            Ok(expr) => expr.print_expr(),
            Err(err) => println!("Error {err}",),
        };
    }

    fn expression(&mut self) -> Result<Box<Expr>, Error> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Box<Expr>, Error> {
        let mut expr = self.comparison();
        while self.match_next(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let op = self.previous().clone();
            let xr = self.comparison();
            expr = Ok(Box::new(Expr::Binary(expr, op, xr)));
        }
        expr
    }

    fn comparison(&mut self) -> Result<Box<Expr>, Error> {
        let mut expr = self.term();
        while self.match_next(&[
            TokenType::GREATER,
            TokenType::GREATER,
            TokenType::LESS,
            TokenType::LessEqual,
        ]) {
            let op = self.previous().clone();
            let xr = self.term();
            expr = Ok(Box::new(Expr::Binary(expr, op, xr)));
        }

        expr
    }

    fn match_next(&mut self, token_types: &[TokenType]) -> bool {
        for tokentype in token_types {
            if self.check(tokentype) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, tokentype: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == *tokentype
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn term(&mut self) -> Result<Box<Expr>, Error> {
        let mut expr = self.factor();
        while self.match_next(&[TokenType::MINUS, TokenType::PLUS]) {
            let op = self.previous().clone();
            let xr = self.factor();
            expr = Ok(Box::new(Expr::Binary(expr, op, xr)));
        }
        expr
    }

    fn factor(&mut self) -> Result<Box<Expr>, Error> {
        let mut expr = self.unary();
        while self.match_next(&[TokenType::SLASH, TokenType::STAR]) {
            let op = self.previous().clone();
            let xr = self.unary();
            expr = Ok(Box::new(Expr::Binary(expr, op, xr)));
        }
        expr
    }

    fn unary(&mut self) -> Result<Box<Expr>, Error> {
        let mut expr = self.primary();
        if self.match_next(&[TokenType::BANG, TokenType::MINUS]) {
            let op = self.previous().clone();
            let xr = self.unary();
            expr = Ok(Box::new(Expr::Unary(op, xr)));
        }
        expr
    }

    fn primary(&mut self) -> Result<Box<Expr>, Error> {
        if self.is_at_end() {
            return Ok(Box::new(Expr::Literal(Box::new(Object::Null)))); //remove me
        }
        if self.match_next(&[TokenType::FALSE]) {
            return Ok(Box::new(Expr::Literal(Box::new(Object::False))));
        }
        if self.match_next(&[TokenType::TRUE]) {
            return Ok(Box::new(Expr::Literal(Box::new(Object::True))));
        }
        if self.match_next(&[TokenType::Null]) {
            return Ok(Box::new(Expr::Literal(Box::new(Object::Null))));
        }

        if self.match_next(&[TokenType::NUMBER, TokenType::STRING]) {
            let tok = self.previous().clone();
            return Ok(Box::new(Expr::Literal(Box::new(tok.literal))));
        }

        if self.match_next(&[TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.");
            return Ok(Box::new(Expr::Grouping(expr)));
        }

        Err(Error::other(format!(
            "{:?} {}",
            self.peek(),
            "Expect expression."
        )))
        // return Ok(Box::new(Expr::Literal(Box::new(Object::Null))); //remove me
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) -> &Token {
        if self.check(&token_type) {
            return self.advance();
        };
        let t = self.peek().clone();
        panic!("{:?} {}", self.peek(), msg);
        self.advance() //remove me
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().token_type == TokenType::SEMICOLON {
                return;
            }
            match self.peek().token_type {
                TokenType::CLASS => {}
                TokenType::FUN => {}
                TokenType::VAR => {}
                TokenType::FOR => {}
                TokenType::IF => {}
                TokenType::WHILE => {}
                TokenType::PRINT => {}
                TokenType::RETURN => return,
                _ => {}
            }
        }
        self.advance();
    }
}
