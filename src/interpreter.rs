use std::collections::HashMap;
use std::fmt::format;


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

    pub fn start(&mut self) {
        while self.index < self.statements.len() {
            Stmt::run_stmt(self.statements[self.index].clone(),  self);
            self.index += 1;
        }
    }
}
impl Expr {
    fn evaluate(&self, mut interpreter: &mut Interpreter) -> obj {
        match self {
            Expr::Grouping(expr) => {
               return expr.evaluate(interpreter)
            },
            Expr::Binary {left,operator,right} => {
                let mut left = left.evaluate(interpreter);
                let mut right = right.evaluate(interpreter);
                match left {
                    obj::Identifier(ref s) => {
                        if interpreter.variables.get(s.as_str()).is_some() {
                            left = interpreter
                                .variables
                                .get(s.as_str())
                                .unwrap().clone();
                        }
                    }
                    _ => {
                        left = left;
                    }
                }
                // println!("{left}");
                match right {
                    obj::Identifier(ref s) => {
                        if interpreter.variables.get(s.as_str()).is_some() {
                            right = interpreter.variables.get(s.as_str()).unwrap().clone();
                        }
                    }
                    _ => {
                        right = right;
                    }
                }
                // println!("{right}");
                match (left.clone(), right.clone()) {
                    (obj::Num(n1), obj::Num(n2)) => {
                        match operator.ttype {
                            TType::Plus => {
                                obj::Num(n1 + n2)
                            },
                            TType::Minus => {
                                obj::Num(n1 - n2)
                            },
                            TType::Slash => {
                                obj::Num(n1 / n2)
                            },
                            TType::Star => {
                                obj::Num(n1 * n2)
                            },
                            TType::GreaterEqual => {
                                if n1 > n2 || n1 == n2 {
                                    obj::Bool(true)
                                } else {
                                    obj::Bool(false)
                                }
                            },
                            TType::LessEqual => {
                                if n1 < n2 || n1 == n2 {
                                    obj::Bool(true)
                                } else {
                                    obj::Bool(false)
                                }
                            },
                            TType::EqualEqual => {
                                if n1 == n2 {
                                    obj::Bool(true)
                                } else {
                                    obj::Bool(false)
                                }
                            },
                            TType::Greater => {
                                if n1 > n2 {
                                    obj::Bool(true)
                                } else {
                                    obj::Bool(false)
                                }
                            },
                            TType::Less => {
                                if n1 < n2 {
                                    obj::Bool(true)
                                } else {
                                    obj::Bool(false)
                                }
                            },
                            TType::BangEqual => {
                                if n1 == n2 {
                                    obj::Bool(false)
                                } else {
                                    obj::Bool(true)
                                }
                            }
                            _ => {
                                ScrapError::error(
                                    EvaluatorError,
                                    "undefined binary operator",
                                    operator.line, file!()
                                );
                                obj::Null
                            }
                        }
                    },
                    (obj::Str(s1), obj::Str(s2)) => {
                        match operator.ttype {
                            TType::Plus => {
                                let s1 = s1.replace('"', "");
                                let s2 = s2.replace('"', "");
                                let mut str = s1;
                                str.push_str(&*s2);
                                obj::Str(str)
                            }
                            TType::Minus => {
                                let s1 = s1.replace('"', "");
                                let s2 = s2.replace('"', "");
                                let str = s1.replace(&s2, "");
                                obj::Str(str)
                            }
                            _ => {
                                ScrapError::error(
                                    InvalidSyntax,
                                    "unable to '-', '*' '/' a string ",
                                    operator.line, file!()
                                );
                                obj::Null
                            }
                        }
                    },
                    (obj::Bool(b1), obj::Bool(b2)) => {
                        match operator.ttype {
                            TType::EqualEqual => {
                                if b1 == b2 {
                                    obj::Bool(true)
                                } else {
                                    obj::Bool(false)
                                }
                            },
                            TType::BangEqual => {
                                if b1 == b2 {
                                    obj::Bool(false)
                                } else {
                                    obj::Bool(true)
                                }
                            },
                            TType::And => {
                                if b1 && b2 {
                                    obj::Bool(true)
                                } else {
                                    obj::Bool(false)
                                }
                            },
                            TType::Or => {
                                if b1 || b2 {
                                    obj::Bool(true)
                                } else {
                                    obj::Bool(false)
                                }
                            }
                            _ => {
                                ScrapError::error(
                                    EvaluatorError,
                                    "unable to do this operation on boolean values",
                                    operator.line, file!()
                                );
                                obj::Null
                            }
                        }
                    },
                    (_, obj::Num(_n)) | (obj::Num(_n), _) => {
                        ScrapError::error(
                            InvalidSyntax,
                            "unable to '+', '-', '*' and '/' here",
                            operator.line, file!()
                        );
                        obj::Null
                    }
                    _ => {
                        obj::Null
                    }
                }

            },
            Expr::Literal(val) => {
                return val.clone()
            },
            Expr::Unary {operator,right} => {
                let right = right.evaluate(interpreter);
                match right {
                    obj::Num(n) => {
                        match operator.ttype {
                            TType::Minus => {
                                obj::Num(-n)
                            },
                            _ => {
                                println!("not a unary operator");
                                obj::Null
                            }
                        }
                    },
                    obj::Bool(b) => {
                        match operator.ttype {
                            TType::Bang => {
                                if b == true {
                                    obj::Bool(false)
                                } else {
                                    obj::Bool(true)
                                }

                            },
                            _ => {
                                println!("not a unary operator");
                                obj::Null
                            }
                        }
                    },
                    _ => {
                        ScrapError::error(
                            InvalidSyntax,
                            "unable to make unary",
                            operator.line, file!()
                        );
                        obj::Null
                    }
                }
            },
            _ => {
                ScrapError::error(InvalidSyntax, "unimplemented", 0, file!());
                obj::Null
            }

        }

    }
}

impl Stmt {
    pub fn run_stmt(stmt: Stmt, interpreter: &mut Interpreter) {
        match stmt {
            Stmt::Print(statement) => {
                match *statement {
                    Stmt::Expression(expression) => {
                        let val = expression.evaluate(interpreter);
                        match val {
                            obj::Num(n) => {
                                println!("{n}");
                            }
                            obj::Bool(b) => {
                                println!("{b}");

                            }
                            obj::Str(s) => {
                                println!("{}", s);

                            }
                            obj::Null => {
                                println!("Null");

                            }
                            obj::Identifier(i) => {
                                print!("{:?}", interpreter.variables.get(i.as_str()))
                            }
                        }
                    },
                    Stmt::VariableCall {identifier} => {
                        let var = interpreter.variables.get(&*identifier).unwrap();
                        match var {
                            obj::Num(n) => {
                                println!("{n}");
                            }
                            obj::Bool(b) => {
                                println!("{b}");

                            }
                            obj::Str(s) => {
                                println!("{}", s);

                            }
                            obj::Null => {
                                println!("Null");

                            }
                            obj::Identifier(i) => {
                                println!("{i}");
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
            Stmt::VariableAssign {identifier, value} => {
                let val = value.evaluate(interpreter);

                if interpreter.variables.contains_key(&*identifier) {
                    interpreter.variables.remove(&*identifier);
                    interpreter.variables.insert(identifier,val);
                } else {
                    interpreter.variables.insert(identifier,val);
                }
            }
            Stmt::VariableCall {identifier} => {
                let _var = interpreter.variables.get(&*identifier).unwrap();
            }
            Stmt::Expression(expression) => {
                expression.evaluate(interpreter);
            },
            Stmt::Ifstmt {expr, block, elseblock} => {
                let expression = expr.evaluate(interpreter);
                match expression {
                    obj::Bool(b) => {
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
            Stmt::WhileStmt {expr, block} => {
                while expr.evaluate(interpreter) == obj::Bool(true) {
                    match *block.clone() {
                        Stmt::Block(b) => {
                            for stmt in b {
                                Stmt::run_stmt(stmt, interpreter)
                            }
                        }
                        _ => {
                            ScrapError::error(
                                EvaluatorError,
                                "expected bool",
                                    line!() as usize,
                                file!()
                            )
                        }
                    }
                }
            }

            _ => {
                println!("UNIMPLEMENTED")
            }

        }
    }
}