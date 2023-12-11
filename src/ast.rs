use std::collections::HashMap;
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
    Comp {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>
    },
    Print(Box<Expr>),

}
#[derive(Debug, Clone)]
pub enum Stmt {
    Print(Box<Stmt>),
    Expression(Box<Expr>),
    Ifstmt {
        expr: Box<Expr>,
        block: Box<Stmt>,
        elseblock: Option<Box<Stmt>>
    },
    Block(Vec<Stmt>),
    Variable_assign {
        identifier: String,
        value: Box<Expr>
    },
    Variable_call{
        identifier: String
    }
    // VarRef{
    //     name: String
    // }
}




