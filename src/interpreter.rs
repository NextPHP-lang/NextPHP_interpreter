use std::collections::HashMap;
use std::ops::{Add,Sub,Mul,Neg,Not,Div, Deref};
use std::str::Matches;
use std::thread::current;
use crate::ast::Expr;
use crate::error::ScrapError;
use crate::error::ScrapError::{EvaluatorError, InvalidSyntax};
use crate::object::obj;
use crate::tokentype::TType;

pub struct Interpreter {
    pub expressions: Vec<Expr>,
    pub variables: HashMap<String, obj>,
    index: usize,
}

impl Interpreter {
    pub fn new(expressions: Vec<Expr>) -> Interpreter {
        Interpreter {
            expressions,
            variables: HashMap::new(),
            index: 0
        }
    }
    pub fn start(self) {
        for expression in &self.expressions {
           Expr::evaluate(expression, Some(&self));
        }
    }
}
impl Expr {
    fn evaluate(&self, x: Option<&Interpreter>) -> obj {
        match self {
            Expr::Grouping(expr) => {
               return expr.evaluate(None)
            },
            Expr::Binary {left,operator,right} => {
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
                            TType::GreaterEqual => {
                                if n1 > n2 || n1 == n2 {
                                    obj::bool(true)
                                } else {
                                    obj::bool(false)
                                }
                            },
                            TType::LessEqual => {
                                if n1 < n2 || n1 == n2 {
                                    obj::bool(true)
                                } else {
                                    obj::bool(false)
                                }
                            },
                            TType::EqualEqual => {
                                if n1 == n2 {
                                    obj::bool(true)
                                } else {
                                    obj::bool(false)
                                }
                            },
                            TType::Greater => {
                                if n1 > n2 {
                                    obj::bool(true)
                                } else {
                                    obj::bool(false)
                                }
                            },
                            TType::Less => {
                                if n1 < n2 {
                                    obj::bool(true)
                                } else {
                                    obj::bool(false)
                                }
                            },
                            TType::BangEqual => {
                                if n1 == n2 {
                                    obj::bool(false)
                                } else {
                                    obj::bool(true)
                                }
                            }
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
                    },
                    (obj::bool(b1), obj::bool(b2)) => {
                        match operator.ttype {
                            TType::EqualEqual => {
                                if b1 == b2 {
                                    obj::bool(true)
                                } else {
                                    obj::bool(false)
                                }
                            },
                            TType::BangEqual => {
                                if b1 == b2 {
                                    obj::bool(false)
                                } else {
                                    obj::bool(true)
                                }
                            },
                            TType::And => {
                                if b1 && b2 {
                                    obj::bool(true)
                                } else {
                                    obj::bool(false)
                                }
                            },
                            TType::Or => {
                                if b1 || b2 {
                                    obj::bool(true)
                                } else {
                                    obj::bool(false)
                                }
                            }
                            _ => {
                                ScrapError::error(
                                    EvaluatorError,
                                    "unable to do this operation on boolean values",
                                    operator.line, file!()
                                );
                                obj::null
                            }
                        }
                    },
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
                return val.clone()
            },
            Expr::Unary {operator,right} => {
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
                                if b == true {
                                    obj::bool(false)
                                } else {
                                    obj::bool(true)
                                }

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
            Expr::Print(expression) => {
                let val = expression.evaluate(None);
                match val {
                    obj::num(n) => {
                        println!("echo: {n}");
                        obj::num(n)
                    }
                    obj::bool(b) => {
                        println!("echo: {b}");
                        obj::bool(b)
                    }
                    obj::str(s) => {
                        format!("echo: {s}");
                        obj::str(s.clone())
                    }
                    obj::null => {
                        println!("Null");
                        obj::null
                    }
                    obj::variable(n,v) => {
                        println!("{}", *v);
                        obj::variable(n.clone(), v.clone())
                    }
                }
            }
            // Expr::VarRef {identifier} => {
            //     println!("eval: variable reference");
            //     let id = &identifier.literal;
            //     let map = &x.unwrap().variables;
            //     let val = map.get(id);
            //     if val.is_some() {
            //         let value = val.unwrap();
            //         return value.clone()
            //     } else {
            //         ScrapError::error(
            //             InvalidSyntax, format!("undefined variable '{}'", id).as_str(),
            //             0, operator.file.clone()
            //         );
            //         return obj::null
            //     }
            // },
            _ => {
                ScrapError::error(InvalidSyntax, "unimplemented", 0, file!());
                obj::null
            }

        }

    }
}