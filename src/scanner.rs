use crate::error::MloxError;
use crate::token::{Object, Token, TokenType};
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    static KEYWORDS: RefCell< HashMap<&'static str, TokenType> > =
        HashMap::from([
                      ("and", TokenType::AND),
                      ("class", TokenType::CLASS),
                      ("else", TokenType::ELSE),
                      ("false", TokenType::FALSE),
                      ("for", TokenType::FOR),
                      ("fun", TokenType::FUN),
                      ("if", TokenType::IF),
                      ("null", TokenType::NIL),
                      ("or", TokenType::OR),
                      ("print", TokenType::PRINT),
                      ("ret", TokenType::RETURN),
                      ("super", TokenType::SUPER),
                      ("this", TokenType::THIS),
                      ("true", TokenType::TRUE),
                      ("var", TokenType::VAR),
                      ("while", TokenType::WHILE),

        ]).into();

}

pub struct Scanner<'a> {
    /// Source code.
    source: &'a String,

    /// List of tokens.
    pub tokens: Vec<Token>,

    ///  Points at the first character in the lexeme being scanned.
    start: usize,

    /// Points at the character currently being considered.
    current: usize,

    ///  Field tracks what source line current is on.
    line: usize,

    /// Error handle.
    error: &'a mut MloxError,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a String, _error: &'a mut MloxError) -> Self {
        Self {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            error: _error,
        }
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.add_none_token(TokenType::EOF);
    }

    fn add_none_token(&mut self, token_type: TokenType) {
        self.tokens
            .push(Token::new(token_type, "".into(), Object::None, self.line));
    }

    fn add_token(&mut self, token_type: TokenType, literal: Object) {
        let text = &self.source[self.start..self.current];
        self.tokens
            .push(Token::new(token_type, text.to_string(), literal, self.line));
    }

    /// True if we consumed all the characters.
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    /// Scan single token.
    fn scan_token(&mut self) {
        let ch = self.advance();
        match ch {
            '(' => self.add_none_token(TokenType::LeftParen),
            ')' => self.add_none_token(TokenType::RightParen),
            '{' => self.add_none_token(TokenType::LeftBrace),
            '}' => self.add_none_token(TokenType::RightBrace),
            ',' => self.add_none_token(TokenType::COMMA),
            '.' => self.add_none_token(TokenType::DOT),
            '-' => self.add_none_token(TokenType::MINUS),
            '+' => self.add_none_token(TokenType::PLUS),
            ';' => self.add_none_token(TokenType::SEMICOLON),
            '*' => self.add_none_token(TokenType::STAR),
            '!' => {
                if self.match_next('=') {
                    self.add_none_token(TokenType::BangEqual)
                } else {
                    self.add_none_token(TokenType::BANG)
                }
            }
            '=' => {
                if self.match_next('=') {
                    self.add_none_token(TokenType::EqualEqual)
                } else {
                    self.add_none_token(TokenType::EQUAL)
                }
            }

            '<' => {
                if self.match_next('=') {
                    self.add_none_token(TokenType::LessEqual)
                } else {
                    self.add_none_token(TokenType::LESS)
                }
            }

            '>' => {
                if self.match_next('=') {
                    self.add_none_token(TokenType::GreaterEqual)
                } else {
                    self.add_none_token(TokenType::GREATER)
                }
            }

            '/' => {
                if self.match_next('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_none_token(TokenType::SLASH)
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,

            '"' | '\'' => self.string_check(),
            '0'..='9' => self.number_check(),
            'a'..='z' | 'A'..='Z' | '_' => self.identifier_check(),

            _ => {
                self.error.send(self.line, "Unexpected character.");
            }
        }
    }

    /// Consumes the next character in the source file and returns it.
    fn advance(&mut self) -> char {
        let ch = self.source.chars().nth(self.current).expect(&format!(
            "scanner.advance: canot read char at {}",
            self.current
        ));
        self.current += 1;
        ch
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self
            .source
            .chars()
            .nth(self.current)
            .expect("scanner.match_next: cannot reac char at {}")
            != expected
        {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        };
        self.source
            .chars()
            .nth(self.current)
            .expect("error: scanner.peek")
    }

    fn string_check(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            self.error.send(self.line, "Unterminated string!");
            return;
        }
        // The closing ".
        self.advance();
        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token(TokenType::STRING, Object::Str(value.to_owned()));
    }

    fn number_check(&mut self) {
        while self.peek().is_numeric() {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_numeric() {
            // consume the "."
            self.advance();
            while self.peek().is_numeric() {
                self.advance();
            }
        }
        let n = self.source[self.start..self.current]
            .parse::<f64>()
            .expect("cannot parse {} to floot number");
        self.add_token(TokenType::NUMBER, Object::Num(n));
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        };
        self.source
            .chars()
            .nth(self.current + 1)
            .expect("Error: scanner.peek_next: cannot read char at {}")
    }

    fn identifier_check(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }
        let text = &self.source[self.start..self.current];
        let token_type = KEYWORDS.with(|kw| {
            let keywords = kw.borrow();
            let tt = keywords
                .get(&text)
                .unwrap_or(&TokenType::IDENTIFIER)
                .clone();
            tt
        });
        self.add_none_token(token_type);
    }
}
