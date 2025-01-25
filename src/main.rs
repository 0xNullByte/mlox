use std::env;
mod error;
mod expr;
mod mlox;
mod scanner;
mod token;
use expr::*;
use mlox::*;
use token::{Object, Token};

fn main() {
    // let args = env::args().collect::<Vec<_>>();
    // let mut mlox = Mlox::new(args);
    // mlox.interpreter();
    let literal = Expr::Literal(Box::new(Object::Num(123.0)));
    let unary = Expr::Unary(
        Token::new(token::TokenType::MINUS, "-".into(), token::Object::None, 1),
        Box::new(literal),
    );
    let grp = Expr::Grouping(Box::new(Expr::Literal(Box::new(Object::Num(45.67)))));

    let bin = Expr::Binary(
        Box::new(unary),
        Token::new(token::TokenType::STAR, "*".into(), Object::None, 1),
        Box::new(grp),
    );
    bin.print_expr();
}
