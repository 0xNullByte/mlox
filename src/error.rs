use std::io::Error;

use crate::token::{Token, TokenType};

#[derive(Default)]
pub struct ScannerError {
    pub had_err: bool,
}

impl ScannerError {
    pub fn send(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    pub fn report(&mut self, line: usize, where_: &str, message: &str) {
        println!("[line {}] Error {}: {}", line, where_, message);
        self.had_err = true;
    }
}

pub struct ParseError {
    pub had_err: bool,
}

impl ParseError {
    pub fn new(had_err: bool) -> Self {
        Self { had_err }
    }
    pub fn send(&mut self, token: &Token, message: &str) -> Result<(), Error> {
        if token.token_type == TokenType::EOF {
            return self.report(token.line, " at end", message);
        }
        self.report(token.line, &format!(" at '{}'", token.lexeme), message)
    }

    pub fn report(&mut self, line: usize, where_: &str, message: &str) -> Result<(), Error> {
        println!("[line {}] Error {}: {}", line, where_, message);
        self.had_err = true;
        Err(Error::other(format!(
            "[line {}] Error {}: {}",
            line, where_, message
        )))
    }
}
