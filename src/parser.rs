use std::fmt::format;
use std::thread::current;
use log::error;
use crate::token::Token;
use crate::ast::*;
use crate::error::ScrapError;
use crate::error::ScrapError::ParserError;
use crate::object::obj;
use crate::object::obj::null;
use crate::tokentype::TType;
use crate::tokentype::TType::*;
use colored::*;
use crate::parser::Peekmode::Reverse;

pub struct Parser {
    pub tokens: Vec<Token>,
    current_token: Option<Token>,
    index: usize,
    pub expressions: Vec<Expr>
}
enum Peekmode {
    Reverse,
    Forward,
}


impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        let current_token = tokens.get(0).cloned();
        Parser {
            tokens,
            current_token,
            index: 0,
            expressions: Vec::new()
        }
    }


    pub fn parse(&mut self) {
        while !self.is_at_end() {
            // println!("[current token]: {:?}", self.current().unwrap());
            // println!("[current index]: {:?} \n", self.index);
            match self.current().unwrap().ttype { //toodo put all the things that use binary in 1 match arm, and put the diffrent logic in another match
                Number => {
                    if self.check_next(&[Plus, Minus, Star, Slash]) {
                        let expr = self.parse_binary();
                        println!("[starts as binary]: {:?} \n", expr);
                        self.expressions.push(expr)
                    }
                },
                LeftParen => {
                  let expr = self.parse_group();
                  println!("[starts as group]: {:?} \n", expr);
                  self.expressions.push(expr);
                },
                Bang | Minus => {
                    if self.check_next(&[Number]) {
                        let expr = self.parse_binary();
                        println!("[starts as unary]: {:?} \n", expr);
                        self.expressions.push(expr)
                    }
                },
                Var => {
                        let expr = self.parse_var();
                        println!("[starts as var]: {:?} \n", expr);
                        self.expressions.push(expr)
                }
                Semicolon => {
                    let semicolon= self.current().unwrap();
                    let expr = Expr::Eol {
                        semicolon
                    };
                    println!("[EOL]: {:?} \n", expr);
                    self.expressions.push(expr)
                },
                _ => println!("{:?} is not implemented yet or is unknown", self.current().unwrap())
            }
            self.advance();

        }
        }

//add parser logic in here
    fn parse_binary(&mut self) -> Expr {
        let mut expr: Expr = Expr::Literal(obj::null);
        let mut right_is_null = false;
        loop {
            let mut left = match self.current().unwrap().ttype {
                LeftParen => {
                    Box::new(self.parse_group())
                },
                Number => {
                    Box::new(self.parse_primary())
                },
                Minus | Bang => {
                    // println!("[token]: minus or bang");
                    if !self.check_previous(&[Number]) && self.check_next(&[Number, LeftParen]) {
                        // println!("should add unary");
                        Box::new(self.parse_unary())
                    } else {
                        Box::new(self.parse_primary())
                    }
                },
                Var => {
                  Box::new(self.parse_var())
                },
                _ => Box::new(self.parse_primary())
            };
            // println!("[left]: {:?}", left);
            match self.current().unwrap().ttype {
                Plus | Minus | Star | Slash => {
                    self.advance();
                    let operator = self.previous().unwrap();
                    // println!("[operator]: {:?}, [current token]: {:?}", operator, self.current_token);

                    let mut right = match self.current().unwrap().ttype {
                        LeftParen => {
                            Box::new(self.parse_group())
                        },
                        Number => {
                            Box::new(self.parse_primary())
                        },
                        Minus | Bang => {
                            // println!("[token]: minus");
                            if !self.check_previous(&[Number]) && self.check_next(&[Number]) {
                                // println!("should add unary");
                                Box::new(self.parse_unary())
                            } else {
                                Box::new(self.parse_primary())
                            }
                        },
                        Var => {
                            Box::new(self.parse_var())
                        },
                        _ => {
                            right_is_null = true;
                            Box::new(self.parse_primary())
                        }
                    };
                    // println!("[right]: {:?}", right);
                    if Some(&left).is_some()  && Some(&right).is_some() {
                        // println!("\n\n[doing binary]\n\n");
                        expr = Expr::Binary {
                            left,
                            operator,
                            right
                        };
                    } else if right_is_null {
                        if matches!(*left, Expr::Unary { .. }) {
                            // println!("\n\n[doing unary]\n\n");
                            expr = Expr::Unary {
                                operator,
                                right
                            }
                        }
                    }

                    return expr;
                },
                _ => {
                    break; //there are no more binary or grouping expressions to add
                }
            };
        }
    expr
}

    fn parse_group(&mut self) -> Expr {
        match self.current().unwrap().ttype {
            LeftParen => {
                // println!("token is a left paren");
                self.advance();
                let groupexpr = self.parse_binary();
                if self.current().unwrap().ttype == RightParen {
                    self.advance();
                } else {
                    ScrapError::error(
                        ParserError,
                        "Missing ')' ",
                        self.current().unwrap().line,
                        file!()
                    );
                }
                // println!("[group expr]: {:?}", groupexpr);
                let expr = Expr::Grouping(Box::new(groupexpr));
                expr
            },
            _ => {
                println!("\n[parse_group]: literal value of null gets added\n\n");
                ScrapError::error(ParserError, format!("{:?} is not an unary value", self.current().unwrap().ttype).as_str(), self.current().unwrap().line, file!());
                Expr::Literal(obj::null)
            }
        }
    }

    fn parse_unary(&mut self) -> Expr {
        // println!("[unary]: entered unary parse");
        match self.current().unwrap().ttype {
            Bang | Minus => {
                let operator = self.current().unwrap();
                self.advance();
                // println!("[unary]: operator = {:?}", operator);
                let mut right = match self.current().unwrap().ttype {
                    LeftParen => { //it should start making a group expression
                        // println!("[unary][group]: group expression");
                        Box::new(self.parse_group())
                    },
                    Number => { //it should make a literal expression
                        println!("[unary][number]: literal expression");
                        Box::new(self.parse_primary())
                    },
                    Var => {
                        Box::new(self.parse_var())
                    },
                    _ => Box::new(self.parse_primary()) //dont know what it should do so number.
                };
                // println!("[unary]: right = {:?}", right);
                let expr = Expr::Unary {
                    operator,
                    right
                };
                println!("[unary]: expression = {:?}", expr);
                expr
            },
            _ => {
                println!("\n[parse_unary]: literal value of null gets added\n\n");
                ScrapError::error(ParserError, format!("{:?} is not an unary value", self.current().unwrap().ttype).as_str(), self.current().unwrap().line, file!());
                Expr::Literal(obj::null)
            }
        }

    }

    fn parse_primary(&mut self) -> Expr {
        let token = self.current().unwrap();
        match token.ttype {
            True => {
                println!("[obj]: true");
                self.advance();
                Expr::Literal(obj::bool(true))
            },
            False => {
                println!("[obj]: false");
                self.advance();
                Expr::Literal(obj::bool(false))
            },
            Null => {
                println!("[obj]: null");
                self.advance();
               Expr::Literal(obj::null)
            },
            Number => {
                println!("[obj]: number");
                self.advance();
                println!("[obj number]: {:?}", token.literal.parse::<f64>().unwrap());
                Expr::Literal(obj::num(token.literal.parse::<f64>().unwrap()))

            },
            String_tok =>{
                println!("[obj]: String");
                self.advance();
                Expr::Literal(obj::str(token.literal.clone()))
            } ,
            _ => {
                println!("\n[parse_primary]: literal value of null gets added\n\n");
                ScrapError::error(ParserError, format!("{:?} is not a literal value", token.ttype).as_str(), token.line, file!());
                Expr::Literal(obj::null)
            }
        }

    }
    fn parse_var(&mut self) -> Expr {
        let mut expr = Expr::Literal(obj::null);
        self.advance();
        let identifier = self.current().unwrap();
        // println!("[identifier]: {:?}", identifier);
        self.advance();
        // println!("[token before match]: {:?}", self.current().unwrap());
        match self.current().unwrap().ttype {
            Equal | PlusEqual | MinusEqual => {
                self.advance();
                let value = Box::new(self.parse_binary());

                expr = Expr::VarAssign {
                    identifier,
                    value
                };
            }
            _ => {
                expr = Expr::VarRef {
                    identifier
                };
            }
        }

        expr
    }

    fn parse_echo(&mut self) -> Stmt {
       let expr = Box::new(self.parse_binary());
       Stmt::Print {value: expr}
    }




    // helper functions under here

    fn match_next(&mut self, tokens: &[TType]) -> Option<Token> {
        let next_tok = self.peek().unwrap();
        if tokens.contains(&next_tok.ttype) {
            self.advance();
        }
        None
    }

    fn advance(&mut self)  {
        if !self.is_at_end() {
            self.index += 1;
            self.current_token = self.tokens.get(self.index).cloned();
        }
        // return self.previous() //dont need this yet
    }
    fn is_at_end(&mut self) -> bool {
       if self.index >= self.tokens.len() {
           true
       } else { false }
    }

    fn at_eol(&mut self) -> bool {
        if self.current().unwrap().ttype  == Semicolon { //is true if its at the end of the line
            true
        } else {
            false
        }
    }

    fn peek(&mut self) -> Option<&Token> {
        if !self.is_at_end() {
            Some(&self.tokens[self.index])
        } else {
            None
        }
    }

    fn index_peek(&mut self, peekmode: Peekmode, index: usize) -> Option<&Token> {
        if !self.is_at_end() {
            match peekmode {
                Peekmode::Forward => {
                    return Some(&self.tokens[self.index + index]);
                }
                Peekmode::Reverse => {
                    return Some(&self.tokens[self.index - index]);
                }
            }
        } else {
            None
        }

    }

    fn expect(&mut self, ttype: TType) -> Option<Token> {
        let t = self.match_next(&[ttype]);

        if t.is_none() {
            ScrapError::error(ParserError, format!("expected {:?}", t).as_str(), self.current().unwrap().line, file!());
            return None;
        }
        t
    }

    fn current(&mut self) -> Option<Token> {
        self.current_token.clone()
    }
    fn previous(&mut self) -> Option<Token> {
        if self.index != 0 {
            Some(self.tokens[self.index - 1].clone())
        } else {
            println!("[previous]: returned first token");
            Some(self.tokens[self.index].clone())
        }

    }
    fn is_number(&mut self) -> bool {
        if self.current().unwrap().ttype == Number {
            return true;
        }
        false
    }

    fn check_next(&mut self, ttype: &[TType]) -> bool {
        if ttype.contains(&self.tokens[self.index + 1].ttype) {
            return true;
        }
        false
    }
    fn check_previous(&mut self, ttype: &[TType]) -> bool {
        if self.index != 0 && ttype.contains(&self.tokens[self.index - 1].ttype) {
            return true;
        } else if ttype.contains(&self.tokens[self.index].ttype) {
            println!("[check_prev]: returned token 0");
            return true
        }
        false
    }



}