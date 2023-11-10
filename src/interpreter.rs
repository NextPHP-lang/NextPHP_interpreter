use std::collections::hash_map::Values;
use std::collections::HashMap;
use std::fmt::format;
use crate::ast::Expr;
use crate::error::ScrapError;
use crate::error::ScrapError::{EvaluatorError, ScannerError};
use crate::object::obj;
use crate::object::obj::{null, num};
use crate::token::Token;
use crate::tokentype::TType;
use crate::tokentype::TType::True;

pub struct Interpreter {
    pub expressions: Vec<Expr>,
    pub variables: HashMap<String, Expr>,
    index: usize,
}

impl Interpreter {
    pub fn new(expressions: Vec<Expr>) -> Interpreter {
        Interpreter {
            expressions,
            variables: HashMap::new(),
            index: 0,
        }
    }
    pub fn evaluate(&mut self) {
        let expr = self.current();
        match expr {
            Expr::VarAssign {identifier, value} => {
                self.eval_var_declaration(identifier.literal, value);
            },
            Expr::Binary {left,operator,right} => {
                self.eval_binary_num(left,operator, right, false);
            },
            // Expr::VarRef {identifier} => { //revisit this later for when you need it in like binary things
            //     let val = self.variables.get(&*identifier.literal);
            //     if val.is_some() {
            //         val
            //     } else {
            //         ScrapError::error(EvaluatorError, format!("{:?} is undefined", val).as_str(), identifier.line, file!())
            //     }
            // },
            _ => println!("unimplemented")
        }
    }
    fn eval_var_declaration(&mut self, name: String, value: Box<Expr>) {
        self.variables.insert(name, *value);
    }

    fn eval_binary_num(&mut self, left: Box<Expr>, operator: Token, right: Box<Expr>, in_group: bool) -> Option<f64> {
        let left: Option<f64> = match *left {
            Expr::Literal(obj::num(number)) => {
                Some(number)
            },
            Expr::Literal(obj::null) => {
                None
            },
            Expr::Literal(obj::str(..)) => {
                self.eval_binary_string(&left, operator.clone(), &right);
                None
            },
            Expr::Literal(obj::bool(bool)) => {
                ScrapError::error(EvaluatorError, "can't perform binary operations on booleans", 0000, file!());
                None
            },
            Expr::Grouping(expr) => {
                match *expr {
                    Expr::Binary { left, operator, right } => {
                        let val: Option<f64> = self.eval_binary_num(left, operator.clone(), right, true);
                        Some(val.unwrap())
                    }
                    _ => None
                }
            },
            Expr::Unary { operator, right } => {
                match operator.ttype {
                    TType::Minus => {
                        match *right {
                            Expr::Literal(obj::num(number_right)) => {
                                Some(-number_right)
                            },
                            Expr::Literal(obj::null) => {
                                None
                            },
                            _ => None
                        }
                    },
                    _ => {
                        None
                    }
                }
            }
                    // Expr::VarRef {identifier} => { //revisit this later for when you need it in like binary things
                    //     let val = self.variables.get(&*identifier.literal);
                    //     if val.is_some() {
                    //
                    //         Some(621.69) //test value for now, until i figure out how to get the value
                    //     } else {
                    //         ScrapError::error(EvaluatorError, format!("{:?} is undefined", val).as_str(), identifier.line, file!());
                    //         None
                    //     }
                    // },
            _ => {
                ScrapError::error(EvaluatorError, "invalid left", 0000, file!());
                None
            }
        };
        let right: Option<f64> = match *right {
            Expr::Literal(obj::num(number_right)) => {
                Some(number_right)
            },
            Expr::Literal(obj::null) => {
                None
            },
            Expr::Literal(obj::str(..)) => {
                ScrapError::error(EvaluatorError, "can't add string to a number", 0000, file!());
                None
            },
            Expr::Literal(obj::bool(bool)) => {
                ScrapError::error(EvaluatorError, "can't perform binary operations on booleans", 0000, file!());
                None
            },
            Expr::Grouping(expr) => {
                match *expr {
                    Expr::Binary { left, operator, right } => {
                        let val: Option<f64> = self.eval_binary_num(left, operator.clone(), right, true);
                        Some(val.unwrap())
                    }
                    _ => None
                }
            },
            Expr::Unary { operator, right } => {
                match operator.ttype {
                    TType::Minus => {
                        match *right {
                            Expr::Literal(obj::num(number_right)) => {
                                Some(-number_right)
                            },
                            Expr::Literal(obj::null) => {
                                None
                            },
                            _ => None
                        }
                    },
                    _ => {
                        None
                    }
                }
            }
                    // Expr::VarRef {identifier} => { //revisit this later for when you need it in like binary things
                    //     let val = self.variables.get(&*identifier.literal);
                    //     if val.is_some() {
                    //
                    //         Some(68.31) //test value for now, until i figure out how to get the value
                    //     } else {
                    //         ScrapError::error(EvaluatorError, format!("{:?} is undefined", val).as_str(), identifier.line, file!());
                    //         None
                    //     }
                    // },
            _ => {
                ScrapError::error(EvaluatorError, "invalid left", 0000, file!());
                None
            }
        };
        if left.is_some() && right.is_some() {
            return match operator.ttype {
                TType::Plus => {
                    let val: f64 = left.unwrap() + right.unwrap();
                    if !in_group {
                        println!("left: {}", left.unwrap());
                        println!("right: {}", right.unwrap());
                        println!("{}", val);
                    }

                    Some(val)
                },
                TType::Minus => {
                    let val: f64 = left.unwrap() - right.unwrap();
                    if !in_group {
                        println!("left: {}", left.unwrap());
                        println!("right: {}", right.unwrap());
                        println!("{}", val);
                    }
                    Some(val)
                },
                TType::Star => {
                    let val: f64 = left.unwrap() * right.unwrap();
                    if !in_group {
                        println!("left: {}", left.unwrap());
                        println!("right: {}", right.unwrap());

                        println!("{}", val);
                    }
                    Some(val)
                },
                TType::Slash => {
                    let val: f64 = left.unwrap() / right.unwrap();
                    if !in_group {
                        println!("left: {}", left.unwrap());
                        println!("right: {}", right.unwrap());
                        println!("{}", val);
                    }
                    Some(val)
                },
                _ => {
                    ScrapError::error(EvaluatorError, "Not a correct operator", 0, file!());
                    None
                }
            };
        } else {
            ScrapError::error(EvaluatorError, format!("undefined {:?} or {:?}", left, right).as_str(), operator.clone().line, file!());
            return None
        }
    }






    fn eval_binary_string(&mut self, left: &Box<Expr>, operator: Token, right: &Box<Expr>) {

    }

    fn advance(&mut self) {
        self.index += 1
    }

    fn current(&mut self) -> Expr {
        self.expressions[self.index].clone()
    }
}