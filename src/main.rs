use std::env;
mod error;
mod mlox;
mod scanner;
mod token;
use mlox::*;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let mut mlox = Mlox::new(args);
    mlox.interpreter();
}
