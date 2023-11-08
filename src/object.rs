use std::fmt;
use std::ptr::write;
#[derive(Debug, Clone)]
pub enum obj {
    str(String),
    num(f64),
    bool(bool),
    null
}

impl fmt::Display for obj {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            obj::num(x) => write!(f, "{x}"),
            obj::str(x) => write!(f, "{x}"),
            _ => panic!("should not print this")
        }
    }
}