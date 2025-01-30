use std::{io::Error, rc::Rc};

use crate::{
    environ::Environment,
    eval::Evaluate,
    token::{Object, Token, TokenType},
    Expr, Stmt,
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = vec![];
        while !self.is_at_end() {
            stmts.push(self.declaration());
        }
        stmts
    }

    fn statement(&mut self) -> Stmt {
        if self.match_until(&[TokenType::FOR]) {
            return self.for_statement();
        }
        if self.match_until(&[TokenType::IF]) {
            return self.if_statement();
        }
        if self.match_until(&[TokenType::PRINT]) {
            return self.print_statement();
        }

        if self.match_until(&[TokenType::WHILE]) {
            return self.while_statement();
        }

        if self.match_until(&[TokenType::LeftBrace]) {
            return self.block();
        }
        self.expr_statement()
    }

    fn expression(&mut self) -> Result<Box<Expr>, Error> {
        self.assignment()
        // self.equality()
    }

    fn equality(&mut self) -> Result<Box<Expr>, Error> {
        let mut expr = self.comparison();
        while self.match_until(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let op = self.previous().clone();
            let xr = self.comparison();
            expr = Ok(Box::new(Expr::Binary(expr, op, xr)));
        }
        expr
    }

    fn comparison(&mut self) -> Result<Box<Expr>, Error> {
        let mut expr = self.term();
        while self.match_until(&[
            TokenType::GREATER,
            TokenType::GreaterEqual,
            TokenType::LESS,
            TokenType::LessEqual,
        ]) {
            let op = self.previous().clone();
            let xr = self.term();
            expr = Ok(Box::new(Expr::Binary(expr, op, xr)));
        }

        expr
    }

    fn match_until(&mut self, token_types: &[TokenType]) -> bool {
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
        while self.match_until(&[TokenType::PLUS, TokenType::MINUS]) {
            let op = self.previous().clone();
            let xr = self.factor();
            expr = Ok(Box::new(Expr::Binary(expr, op, xr)));
        }
        expr
    }

    fn factor(&mut self) -> Result<Box<Expr>, Error> {
        let mut expr = self.unary();
        while self.match_until(&[TokenType::SLASH, TokenType::STAR]) {
            let op = self.previous().clone();
            let xr = self.unary();
            expr = Ok(Box::new(Expr::Binary(expr, op, xr)));
        }
        expr
    }

    fn unary(&mut self) -> Result<Box<Expr>, Error> {
        if self.match_until(&[TokenType::BANG, TokenType::MINUS]) {
            let op = self.previous().clone();
            let xr = self.unary();
            return Ok(Box::new(Expr::Unary(op, xr)));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Box<Expr>, Error> {
        if self.match_until(&[TokenType::FALSE]) {
            return Ok(Box::new(Expr::Literal(Object::Bool(false))));
        }
        if self.match_until(&[TokenType::TRUE]) {
            return Ok(Box::new(Expr::Literal(Object::Bool(true))));
        }
        if self.match_until(&[TokenType::Null]) {
            return Ok(Box::new(Expr::Literal(Object::Null)));
        }

        if self.match_until(&[TokenType::NUMBER, TokenType::STRING]) {
            let tok = self.previous().clone();
            return Ok(Box::new(Expr::Literal(tok.literal)));
        }

        if self.match_until(&[TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.");
            return Ok(Box::new(Expr::Grouping(expr)));
        }

        if self.match_until(&[TokenType::IDENTIFIER]) {
            let token = self.previous().clone();
            return Ok(Box::new(Expr::Variable(token)));
        }

        Err(Error::other(format!(
            "{:?} {}",
            self.peek(),
            "Expect expression.",
        )))
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) -> &Token {
        if self.check(&token_type) {
            return self.advance();
        };
        panic!("{:?} {}", self.peek(), msg);
    }

    fn synchronize(&mut self) -> Stmt {
        self.advance();
        while !self.is_at_end() {
            if self.previous().token_type == TokenType::SEMICOLON {
                break;
            }
            match self.peek().token_type {
                TokenType::CLASS => {}
                TokenType::FUN => {}
                TokenType::VAR => {}
                TokenType::FOR => {}
                TokenType::IF => {}
                TokenType::WHILE => {}
                TokenType::PRINT => {}
                TokenType::RETURN => break,
                _ => {}
            }
        }
        self.advance();
        self.print_statement() // remove me
    }

    fn print_statement(&mut self) -> Stmt {
        let expr = self.expression();
        self.consume(TokenType::SEMICOLON, "Expect ';' after value.");
        Stmt::PrintStmt(expr)
    }

    fn expr_statement(&mut self) -> Stmt {
        let expr = self.expression();
        self.consume(TokenType::SEMICOLON, "Expect ';' after value.");
        Stmt::ExprStmt(expr)
    }

    fn declaration(&mut self) -> Stmt {
        if self.match_until(&[TokenType::VAR]) {
            return self.var_declaration();
        } else {
            return self.statement();
        }
        // self.synchronize()
    }

    fn var_declaration(&mut self) -> Stmt {
        let name = self
            .consume(TokenType::IDENTIFIER, "Expect varibale name.")
            .clone();

        let mut init = None;
        if self.match_until(&[TokenType::EQUAL]) {
            init = Some(self.expression());
        }
        self.consume(
            TokenType::SEMICOLON,
            "Expect ';' after variable declaration.",
        );
        return Stmt::VarStmt(name, init);
    }

    fn assignment(&mut self) -> Result<Box<Expr>, Error> {
        let expr = self.logic_or();
        if self.match_until(&[TokenType::EQUAL]) {
            let eq = self.previous().clone();
            let value = self.assignment();
            match *expr.unwrap() {
                Expr::Variable(t) => return Ok(Box::new(Expr::Assign(t, value))),
                _ => todo!(),
            };
        }
        expr
    }

    fn block(&mut self) -> Stmt {
        let mut stmts = vec![];
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            stmts.push(Box::new(self.declaration()));
        }
        self.consume(TokenType::RightBrace, "Expect '}' after block.");
        return Stmt::BlockStmt(stmts);
    }

    fn if_statement(&mut self) -> Stmt {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.");
        let condition = self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after 'if condition' .");
        let then_branch = self.statement();
        let mut else_branch = None;
        if self.match_until(&[TokenType::ELSE]) {
            else_branch = Some(Box::new(self.statement()));
        }
        Stmt::IfStmt(condition, Box::new(then_branch), else_branch)
    }

    fn logic_or(&mut self) -> Result<Box<Expr>, Error> {
        let mut expr = self.logic_and();
        while self.match_until(&[TokenType::OR]) {
            let op = self.previous().clone();
            let right = self.logic_and();
            expr = Ok(Box::new(Expr::Logical(expr, op, right)));
        }
        expr
    }

    fn logic_and(&mut self) -> Result<Box<Expr>, Error> {
        let mut expr = self.equality();
        while self.match_until(&[TokenType::AND]) {
            let op = self.previous().clone();
            let right = self.equality();
            expr = Ok(Box::new(Expr::Logical(expr, op, right)));
        }
        expr
    }

    fn while_statement(&mut self) -> Stmt {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.");
        let cond = self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after 'while condition'.");
        let body = self.statement();
        Stmt::WhileStmt(cond, Box::new(body))
    }

    fn for_statement(&mut self) -> Stmt {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.");

        let mut _init = None;
        if self.match_until(&[TokenType::SEMICOLON]) {
            _init = None
        } else if self.match_until(&[TokenType::VAR]) {
            _init = Some(self.var_declaration());
        } else {
            _init = Some(self.expr_statement());
        }

        let mut cond = None;
        if !self.check(&TokenType::SEMICOLON) {
            cond = Some(self.expression());
        }
        self.consume(TokenType::SEMICOLON, "Expect ';' after loop condition.");

        let mut increment = None;
        if !self.check(&TokenType::RightParen) {
            increment = Some(self.expression());
        }
        self.consume(TokenType::RightParen, "Expect ';' after loop condition.");

        let mut body = Some(self.statement());
        if increment.is_some() {
            body = Some(Stmt::BlockStmt(vec![
                Box::new(body.unwrap()),
                Box::new(Stmt::ExprStmt(increment.unwrap())),
            ]));
        };
        if cond.is_none() {
            cond = Some(Ok(Box::new(Expr::Literal(Object::Bool(true)))));
        }
        body = Some(Stmt::WhileStmt(cond.unwrap(), Box::new(body.unwrap())));

        if _init.is_some() {
            body = Some(Stmt::BlockStmt(vec![
                Box::new(_init.unwrap()),
                Box::new(body.unwrap()),
            ]));
        }
        body.unwrap()
    }
}
