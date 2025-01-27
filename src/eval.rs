use crate::{
    token::{Object, TokenType},
    Expr,
};

pub struct Evaluate {
    expr: Box<Expr>,
}

impl Evaluate {
    pub fn new(expr: Box<Expr>) -> Self {
        Self { expr }
    }

    pub fn eval(&self) {
        let obj = self.eval_expr(&self.expr);
        println!("{:?}", obj.to_string());
    }

    pub fn eval_expr(&self, expr: &Expr) -> Object {
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
        };
        v
    }
}
