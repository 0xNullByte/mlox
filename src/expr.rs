use crate::token::{Object, Token};

#[derive(Debug)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Box<Object>),
    Unary(Token, Box<Expr>),
}

impl Expr {
    fn to_string(&self) -> String {
        let xs = match self {
            Self::Binary(xl, t, xr) => {
                format!("({} {} {})", t.lexeme, xl.to_string(), xr.to_string())
            }
            Self::Grouping(x) => format!("(group {})", x.to_string()),
            Self::Literal(obj) => obj.to_string(),
            Self::Unary(t, x) => format!("({} {})", t.lexeme, x.to_string()),
        };
        xs
    }
    pub fn print_expr(&self) {
        println!("{}", self.to_string());
    }
}
