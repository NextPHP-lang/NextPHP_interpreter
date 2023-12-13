use std::fmt;
use crate::ast::Expr;


#[derive(Debug, Clone, PartialEq)]
pub enum obj {
    Str(String),
    Num(f64),
    Bool(bool),
    Null,
    Identifier(String)
}

impl fmt::Display for obj {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            obj::Num(x) => write!(f, "{x}"),
            obj::Str(x) => write!(f, "{x}"),
            _ => panic!("should not print this")
        }
    }
}
