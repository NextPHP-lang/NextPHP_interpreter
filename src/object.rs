use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Not, Sub};
use std::ptr::{addr_of, write};
use crate::error::ScrapError;
use crate::error::ScrapError::InvalidSyntax;

#[derive(Debug, Clone)]
pub enum obj {
    str(String),
    num(f64),
    bool(bool),
    variable(String, Box<obj>),
    eol,
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
