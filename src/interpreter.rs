use std::collections::HashMap;
use std::ops::{Add,Sub,Mul,Neg,Not,Div, Deref};
use std::process::id;
use std::str::Matches;
use std::thread::current;
use crate::ast::{Expr, Stmt};
use crate::error::ScrapError;
use crate::error::ScrapError::{EvaluatorError, InvalidSyntax};
use crate::object::obj;
use crate::tokentype::TType;

pub struct Interpreter {
    pub expressions: Vec<Expr>,
    pub variables: HashMap<String, obj>,
    pub  statements: Vec<Stmt>,
    index: usize,
}

impl Interpreter {
    pub fn new(expressions: Vec<Expr>, statements: Vec<Stmt>) -> Interpreter {
        Interpreter {
            expressions,
            statements,
            variables: HashMap::new(),
            index: 0
        }
    }
    fn getvar(&mut self, name: String) -> &obj {
        if self.variables.contains_key(name.as_str()) {
           let val = self.variables.get(name.as_str()).unwrap();
            println!("{}", val);
           return val
        } else {
            ScrapError::error(
                InvalidSyntax,
                &*("undefined variable: ".to_owned() + &*name),
                line!() as usize,
                file!()
            );
            &obj::null
        }


    }
    fn setvar(&mut self, name: String, value: obj) {
        if self.variables.contains_key(name.as_str()) {
            self.variables.remove(name.as_str());
        }
        self.variables.insert(name, value);
    }
    pub fn start(&mut self) {
        while self.index < self.statements.len() {
            Stmt::run_stmt(self.statements[self.index].clone(),  self);
            self.index += 1;
        }
    }
}
impl Expr {
    fn evaluate(&self, mut interpreter: Option<Interpreter>) -> obj {
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
                                let s1 = s1.replace('"', "");
                                let s2 = s2.replace('"', "");
                                let mut str = s1;
                                str.push_str(&*s2);
                                obj::str(str)
                            }
                            TType::Minus => {
                                let s1 = s1.replace('"', "");
                                let s2 = s2.replace('"', "");
                                let mut str = s1.replace(&s2, "");
                                obj::str(str)
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
            _ => {
                ScrapError::error(InvalidSyntax, "unimplemented", 0, file!());
                obj::null
            }

        }

    }
}

impl Stmt {
    pub fn run_stmt(stmt: Stmt, mut interpreter: &mut Interpreter) {
        match stmt {
            Stmt::Print(statement) => {
                match *statement {
                    Stmt::Expression(expression) => {
                        let val = expression.evaluate(None);
                        match val {
                            obj::num(n) => {
                                println!("{n}");
                            }
                            obj::bool(b) => {
                                println!("{b}");

                            }
                            obj::str(s) => {
                                println!("{}", s);

                            }
                            obj::null => {
                                println!("Null");

                            }
                        }
                    },
                    Stmt::Variable_call {identifier} => {
                        let var = interpreter.variables.get(&*identifier).unwrap();
                        match var {
                            obj::num(n) => {
                                println!("{n}");
                            }
                            obj::bool(b) => {
                                println!("{b}");

                            }
                            obj::str(s) => {
                                println!("{}", s);

                            }
                            obj::null => {
                                println!("Null");

                            }
                        }

                    }
                    _ => {
                        ScrapError::error(
                            InvalidSyntax,
                            "unable to print statement",
                                line!() as usize,
                            file!()
                        )
                    }
                }

            },
            Stmt::Variable_assign {identifier, value} => {
                let val = value.evaluate(None);
                if interpreter.variables.contains_key(&*identifier) {
                    interpreter.variables.remove(&*identifier);
                    interpreter.variables.insert(identifier,val);
                } else {
                    interpreter.variables.insert(identifier,val);
                }
            }
            Stmt::Variable_call {identifier} => {
                let var = interpreter.variables.get(&*identifier).unwrap();
            }
            Stmt::Expression(expression) => {
                expression.evaluate(None);
            },
            Stmt::Ifstmt {expr, block, elseblock} => {
                let expression = expr.evaluate(None);
                match expression {
                    obj::bool(b) => {
                        if b == true {
                            match *block {
                                Stmt::Block(stmts) => {
                                    for stmt in stmts {
                                        Stmt::run_stmt(stmt,interpreter);
                                    }
                                }
                                _ => {
                                    ScrapError::error(
                                        InvalidSyntax,
                                        "exected block",
                                        line!() as usize,
                                        file!()
                                    )
                                }
                            }

                        } else if elseblock.is_some() && b == false {
                            match *elseblock.unwrap() {
                                Stmt::Block(stmts) => {
                                    for stmt in stmts {
                                        Stmt::run_stmt(stmt,interpreter);
                                    }
                                }
                                _ => {
                                    ScrapError::error(
                                        InvalidSyntax,
                                        "exected block",
                                        line!() as usize,
                                        file!()
                                    )
                                }
                            }
                        }
                    }
                    _ => {
                        ScrapError::error(
                            EvaluatorError,
                            "can't perform any other operation in if statement than comparison",
                            line!() as usize,
                            file!()
                        )
                    }
                }
            },

            _ => {
                println!("UNIMPLEMENTED")
            }

        }
    }
}