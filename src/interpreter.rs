use std::collections::HashMap;
use std::ops::{Add,Sub,Mul,Neg,Not,Div, Deref};
use std::str::Matches;
use crate::ast::Expr;
use crate::error::ScrapError;
use crate::error::ScrapError::{EvaluatorError, InvalidSyntax};
use crate::object::obj;
use crate::object::obj::eol;
use crate::tokentype::TType;

pub struct Evaluator {
    pub expressions: Vec<Expr>,
    pub variables: HashMap<String, obj>,
    index: usize,
}

impl Evaluator {
    pub fn new(expressions: Vec<Expr>) -> Evaluator {
        Evaluator {
            expressions,
            variables: HashMap::new(),
            index: 0
        }
    }
    pub fn start(self) {
        for expression in &self.expressions {
            println!("entered next expression");
            let mut result = Expr::evaluate(expression, Some(&self));
            match result {
                obj::num(n) => {
                    println!("number: {n}");
                }
                obj::bool(b) => {
                    println!("bool: {b}");
                }
                obj::str(s) => {
                    format!("string: {s}");
                }
                obj::null => {
                    println!("Null");
                }
                obj::variable(n,v) => {
                    println!("variable: name: {}, value: {}", n, *v);
                }
                obj::eol => {
                    println!("eol")
                }
            }
        }

    }
}
impl Expr {
    fn evaluate(&self, x: Option<&Evaluator>) -> obj {
        match self {
            Expr::Grouping(expr) => {
               println!("eval: group");
               return expr.evaluate(None)
            },
            Expr::Binary {left,operator,right} => {
                println!("eval: binary");
                let left = left.evaluate(None);
                let right = right.evaluate(None);
                match (left, right) {
                    (obj::num(n1), obj::num(n2)) => {
                        match operator.ttype {
                            TType::Plus => {
                                obj::num(n1 + n2)
                            },
                            TType::Minus => {
                                obj::num(n1 - n2)
                            },
                            TType::Slash => {
                                obj::num(n1 / n2)
                            },
                            TType::Star => {
                                obj::num(n1 * n2)
                            },
                            _ => {
                                ScrapError::error(
                                    EvaluatorError,
                                    "undefined binary operator",
                                    operator.line, file!()
                                );
                                obj::null
                            }
                        }
                    },
                    (obj::str(s1), obj::str(s2)) => {
                        match operator.ttype {
                            TType::Plus => {
                                obj::str(s1 + &*s2)
                            }
                            _ => {
                                ScrapError::error(
                                    InvalidSyntax,
                                    "unable to '-', '*' '/' a string ",
                                    operator.line, file!()
                                );
                                obj::null
                            }
                        }
                    }
                    (_, obj::num(n)) | (obj::num(n), _) => {
                        ScrapError::error(
                            InvalidSyntax,
                            "unable to '+', '-', '*' and '/' here",
                            operator.line, file!()
                        );
                        obj::null
                    }
                    _ => {
                        obj::null
                    }
                }

            },
            Expr::Literal(val) => {
                println!("eval: literal");
                return val.clone()
            },
            Expr::Unary {operator,right} => {
                println!("eval: unary");
                let right = right.evaluate(None);
                match right {
                    obj::num(n) => {
                        match operator.ttype {
                            TType::Minus => {
                                obj::num(-n)
                            },
                            _ => {
                                println!("not a unary operator");
                                obj::null
                            }
                        }
                    },
                    obj::bool(b) => {
                        match operator.ttype {
                            TType::Bang => {
                                obj::bool(!b)
                            },
                            _ => {
                                println!("not a unary operator");
                                obj::null
                            }
                        }
                    },
                    _ => {
                        ScrapError::error(
                            InvalidSyntax,
                            "unable to make unary",
                            operator.line, file!()
                        );
                        obj::null
                    }
                }
                },
            Expr::VarAssign {identifier,value} => {
                println!("eval: variable assign");
                let val = value.evaluate(None);
                let id = identifier.literal.clone();
                let mut map = x.unwrap().variables.clone();
                if !map.contains_key(&id) {
                    map.insert(id.clone(), val.clone());
                } else if map.contains_key(&id) {
                    map.remove(&id);
                    map.insert(id.clone(), val.clone());
                }

                return obj::variable(id.clone(), Box::new(val.clone()))
            },
            Expr::VarRef {identifier} => {
                println!("eval: variable reference");
                let id = &identifier.literal;
                let map = &x.unwrap().variables;
                let val = map.get(id);
                if val.is_some() {
                    let value = val.unwrap();
                    return value.clone()
                } else {
                    ScrapError::error(
                        InvalidSyntax, format!("undefined variable '{}'", id).as_str(),
                        0, file!()
                    );
                    return obj::null
                }
            },
            Expr::Eol {semicolon} => {
                println!("eol");
                obj::eol
            },
            _ => {
                ScrapError::error(InvalidSyntax, "unimplemented", 0, file!());
                obj::null
            }

        }

    }
}