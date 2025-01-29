use std::{rc::Rc, sync::Mutex};

use crate::{
    environ::Environment,
    stmt::Stmt,
    token::{Object, TokenType},
    Expr,
};

pub struct Evaluate {
    env: Rc<Mutex<Environment>>,
    stmts: Rc<Vec<Stmt>>,
}

impl Evaluate {
    pub fn new(stmts: Rc<Vec<Stmt>>, env: Rc<Mutex<Environment>>) -> Self {
        Self { stmts, env }
    }

    pub fn eval(&mut self) {
        for stmt in self.stmts.clone().iter() {
            self.eval_stmt(stmt);
        }
    }

    pub fn eval_expr(&mut self, expr: &Expr) -> Object {
        let v = match expr {
            Expr::Literal(v) => v.clone(),
            Expr::Binary(xl, t, xr) => {
                let l_obj = self.eval_expr(xl.as_ref().unwrap());
                let r_obj = self.eval_expr(xr.as_ref().unwrap());

                match t.token_type {
                    TokenType::GREATER => match (l_obj, r_obj) {
                        (Object::Str(l), Object::Str(r)) => Object::Bool(l > r),
                        (Object::Num(l), Object::Num(r)) => Object::Bool(l > r),
                        _ => todo!(),
                    },
                    TokenType::GreaterEqual => match (l_obj, r_obj) {
                        (Object::Str(l), Object::Str(r)) => Object::Bool(l >= r),
                        (Object::Num(l), Object::Num(r)) => Object::Bool(l >= r),
                        _ => todo!(),
                    },
                    TokenType::LESS => match (l_obj, r_obj) {
                        (Object::Str(l), Object::Str(r)) => Object::Bool(l < r),
                        (Object::Num(l), Object::Num(r)) => Object::Bool(l < r),
                        _ => todo!(),
                    },
                    TokenType::LessEqual => match (l_obj, r_obj) {
                        (Object::Str(l), Object::Str(r)) => Object::Bool(l <= r),
                        (Object::Num(l), Object::Num(r)) => Object::Bool(l <= r),
                        _ => todo!(),
                    },
                    TokenType::BangEqual => match (l_obj, r_obj) {
                        (Object::Null, Object::Null) => Object::Bool(false),
                        (Object::Null, _) => Object::Bool(true),
                        (Object::Str(l), Object::Str(r)) => Object::Bool(l != r),
                        (Object::Num(l), Object::Num(r)) => Object::Bool(l != r),
                        _ => todo!(),
                    },

                    TokenType::EqualEqual => match (l_obj, r_obj) {
                        (Object::Null, Object::Null) => Object::Bool(true),
                        (Object::Null, _) => Object::Bool(false),
                        (Object::Str(l), Object::Str(r)) => Object::Bool(l == r),
                        (Object::Num(l), Object::Num(r)) => Object::Bool(l == r),
                        _ => todo!(),
                    },
                    TokenType::MINUS => match (l_obj, r_obj) {
                        (Object::Num(l), Object::Num(r)) => Object::Num(l - r),
                        _ => todo!(),
                    },

                    TokenType::PLUS => match (l_obj, r_obj) {
                        (Object::Num(l), Object::Num(r)) => Object::Num(l + r),
                        (Object::Str(l), Object::Str(r)) => Object::Str(format!("{l}{r}")),
                        _ => todo!(),
                    },
                    TokenType::STAR => match (l_obj, r_obj) {
                        (Object::Num(l), Object::Num(r)) => Object::Num(l * r),
                        (Object::Str(l), Object::Num(r)) => Object::Str(l.repeat(r as usize)),
                        (Object::Num(l), Object::Str(r)) => Object::Str(r.repeat(l as usize)),
                        _ => todo!(),
                    },

                    TokenType::SLASH => match (l_obj, r_obj) {
                        (Object::Num(l), Object::Num(r)) => Object::Num(l / r),
                        _ => todo!(),
                    },

                    _ => todo!(),
                }
            }

            Expr::Unary(t, x) => {
                let obj = self.eval_expr(x.as_ref().unwrap());
                match t.token_type {
                    TokenType::MINUS => match obj {
                        Object::Num(n) => Object::Num(-n),
                        _ => todo!(),
                    },
                    TokenType::BANG => match obj {
                        Object::Null => Object::Bool(true),
                        Object::Bool(b) => Object::Bool(!b),
                        Object::Str(s) => Object::Bool(!s.is_empty()),
                        Object::Num(n) => Object::Bool(n == 0.0),
                    },
                    _ => todo!(),
                }
            }
            Expr::Grouping(x) => self.eval_expr(x.as_ref().unwrap()),
            Expr::Variable(v) => self.env.lock().unwrap().get(v),
            Expr::Assign(t, x) => {
                let obj = self.eval_expr(x.as_ref().unwrap());
                self.env.lock().unwrap().assign(&t.lexeme, obj.clone());
                obj
            }
        };
        v
    }

    fn eval_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::PrintStmt(x) => {
                let value = self.eval_expr(x.as_ref().unwrap());
                println!("{}", value.to_string())
            }

            Stmt::ExprStmt(x) => {
                self.eval_expr(x.as_ref().unwrap());
            }
            Stmt::VarStmt(t, x) => {
                let obj = match x.as_ref() {
                    Some(v) => self.eval_expr(v.as_ref().unwrap()),
                    _ => Object::Null,
                };
                self.env.lock().unwrap().define(&t.lexeme, obj);
            }
            Stmt::BlockStmt(stmts) => {
                let prev_env = self.env.clone();
                self.env = Rc::new(Mutex::new(Environment::new()));
                for stmt in stmts {
                    self.eval_stmt(&stmt);
                }
                self.env = prev_env;
            }
        }
    }
}
