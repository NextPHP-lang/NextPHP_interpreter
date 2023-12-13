



use crate::token::*;
use crate::object::*;
#[derive(Debug, Clone, PartialEq)]
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
    Assign {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>
    }

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
    WhileStmt {
        expr: Box<Expr>,
        block: Box<Stmt>
    },
    Block(Vec<Stmt>),
    VariableAssign {
        identifier: String,
        value: Box<Expr>
    },
    VariableCall {
        identifier: Box<Expr>
    }
}




