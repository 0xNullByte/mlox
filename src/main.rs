use std::env;
mod environ;
mod error;
mod eval;
mod expr;
mod mlox;
mod parser;
mod scanner;
mod stmt;
mod token;
use environ::Environment;
use expr::*;
use mlox::*;
use stmt::*;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let mut mlox = Mlox::new(args);
    mlox.interpreter();
}
