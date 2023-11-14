use std::fmt;
use std::fmt::{Formatter, write};
use crate::scanner::*;
use crate::token::*;
use crate::object::*;
#[derive(Debug, Clone)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>
    },
    Grouping(Box<Expr>),
    Literal(obj),
    Unary {
        operator: Token,
        right: Box<Expr>
    },
    VarRef {
        identifier: Token
    },
    VarAssign {
        identifier: Token,
        value: Box<Expr>
    },
    Call {
        caller: Box<Expr>,
        c_par: Token,
        args: Vec<Expr>
    },
    Comp {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>
    },
    Eol {
        semicolon: Token
    }
}

#[derive(Debug)]
pub enum Stmt {
    Print {
        value: Box<Expr>
    }
}



