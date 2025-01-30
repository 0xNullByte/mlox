use std::io::Error;

use crate::token::{Object, Token, TokenType};

#[derive(Debug)]
pub enum Expr {
    Assign(Token, Result<Box<Expr>, Error>),
    Binary(Result<Box<Expr>, Error>, Token, Result<Box<Expr>, Error>),
    Grouping(Result<Box<Expr>, Error>),
    Literal(Object),
    Logical(Result<Box<Expr>, Error>, Token, Result<Box<Expr>, Error>),
    Unary(Token, Result<Box<Expr>, Error>),
    Variable(Token),
}

impl Expr {
    fn to_string(&self) -> String {
        let xs = match self {
            Self::Binary(xl, t, xr) => {
                let op = match t.token_type {
                    TokenType::STAR => "*",
                    TokenType::SLASH => "/",
                    TokenType::PLUS => "+",
                    TokenType::MINUS => "-",
                    _ => "??",
                };
                if xl.is_err() {
                    return xl.as_ref().err().unwrap().to_string();
                }
                if xr.is_err() {
                    return xr.as_ref().err().unwrap().to_string();
                }

                format!(
                    "({} {} {})",
                    op,
                    xl.as_ref().unwrap().to_string(),
                    xr.as_ref().unwrap().to_string()
                )
            }
            Self::Grouping(x) => {
                if x.is_err() {
                    return x.as_ref().err().unwrap().to_string();
                }
                format!("(group {})", x.as_ref().unwrap().to_string())
            }
            Self::Literal(obj) => obj.to_string(),
            Self::Unary(t, x) => {
                if x.is_err() {
                    return x.as_ref().err().unwrap().to_string();
                }
                format!("({} {})", t.lexeme, x.as_ref().unwrap().to_string())
            }
            _ => todo!(),
        };
        xs
    }
    pub fn print_expr(&self) {
        println!("{}", self.to_string());
    }
}
