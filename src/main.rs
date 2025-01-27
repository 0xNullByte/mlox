use std::env;
mod error;
mod eval;
mod expr;
mod mlox;
mod parser;
mod scanner;
mod token;
use expr::*;
use mlox::*;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let mut mlox = Mlox::new(args);
    mlox.interpreter();
    // let literal = Expr::Literal(Box::new(Object::Num(123.0)));
    // let unary = Expr::Unary(
    //     Token::new(token::TokenType::MINUS, "-".into(), token::Object::Null, 1),
    //     Box::new(literal),
    // );
    // let grp = Expr::Grouping(Box::new(Expr::Literal(Box::new(Object::Num(45.67)))));

    // let bin = Expr::Binary(
    //     Box::new(unary),
    //     Token::new(token::TokenType::STAR, "*".into(), Object::Null, 1),
    //     Box::new(grp),
    // );

    // bin.print_expr();
}
