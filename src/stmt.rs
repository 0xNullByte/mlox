use std::io::Error;

use crate::{expr::Expr, token::Token};
#[derive(Debug)]
pub enum Stmt {
    ExprStmt(Result<Box<Expr>, Error>),
    PrintStmt(Result<Box<Expr>, Error>),
    VarStmt(Token, Option<Result<Box<Expr>, Error>>),
    BlockStmt(Vec<Box<Stmt>>),
    IfStmt(Result<Box<Expr>, Error>, Box<Stmt>, Option<Box<Stmt>>),
    WhileStmt(Result<Box<Expr>, Error>, Box<Stmt>),
}
