use std::thread::current;
use crate::Token;
use crate::ast::Expr;
use crate::ast::Expr::{Binary, Grouping, Literal, Unary};
use crate::error::ScrapError;
use crate::error::ScrapError::ParserError;
use crate::object::obj;
use crate::tokentype::TType;
use crate::tokentype::TType::{Bang, BangEqual, Eof, EqualEqual, False, Greater, GreaterEqual, LeftParen, Less, LessEqual, Minus, Null, Number, Plus, Slash, Star, String_tok, True, While};

pub struct Parser {
    pub tokens: Vec<Token>,
    current_token: Option<Token>,
    index: usize,
    pub expressions: Vec<Expr>
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
        println!("at parse func: {:?}", self.current().unwrap());
        let expr = self.expression();
        self.expressions.push(expr);
        self.advance();
    }
    //parsing functions
    fn expression(&mut self) -> Expr {
        println!("0");
        println!("{:?}", self.current().unwrap());
        let expr = self.equality();
        format!("{:?}", expr);
        return expr
    }
    fn equality(&mut self) -> Expr {
        println!("1");
        let mut expr = self.comparison();
        while self.match_next(&[BangEqual, EqualEqual]) {
            println!("true");
            let operator = self.previous().unwrap();
            let right = self.comparison();
            expr = Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right)
            };
            format!("{:?}", expr);
            return expr
        }
        expr
    }

    fn comparison(&mut self) -> Expr {
        println!("2");
        let mut expr = self.term();
        while self.match_next(&[Greater,GreaterEqual,Less,LessEqual]) {
            println!("true");
            let operator = self.previous().unwrap();
            let right = self.term();
            expr = Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right)
            };
            format!("{:?}", expr);
            return expr
        }
        expr
    }

    fn term(&mut self) -> Expr {
        println!("3");
        let mut expr = self.factor();
        while self.match_next(&[Plus, Minus]) {
            println!("true");
            let operator = self.previous().unwrap();
            let right = self.term();
            expr = Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right)
            };
            format!("{:?}", expr);
            return expr
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        println!("4");
        let mut expr = self.unary();
        while self.match_next(&[Star, Slash]) {
            println!("true");
            let operator = self.previous().unwrap();
            let right = self.factor();
            expr = Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right)
            };
            format!("{:?}", expr);
            return expr
        }
        expr
    }
    fn unary(&mut self) -> Expr {
        println!("5");
        println!("{:?}", self.current().unwrap());
        let expression: Expr = match self.current().unwrap().ttype {
            Minus | Bang => {
                let operator = self.current().unwrap();
                let right = self.unary();
                return Unary {
                    operator,
                    right: Box::new(right)
                }
            },
            Number | Null | String_tok | True | False | LeftParen => {
                return self.parse_primary();
            },
            _ => {
                self.parse_primary()
            }
        };
        return expression;


    }
    fn parse_primary(&mut self) -> Expr {
        println!("6");
        println!("{:?}", self.current().unwrap());
        match self.current().unwrap().ttype {
            TType::Number => {
                println!("parser: number");
                self.advance();
                println!("{:?}", self.current().unwrap());
                return Literal(obj::num(self.previous().unwrap().literal.parse::<f64>().unwrap()))
            },
            TType::String_tok => {
                println!("parser: String");
                self.advance();
                return Literal(obj::str(self.previous().unwrap().literal))
            },
            TType::True => {
                println!("parser: true");
                self.advance();
                return Literal(obj::bool(true))
            },
            TType::False => {
                println!("parser: false");
                self.advance();
                return Literal(obj::bool(false))
            },
            TType::Null => {
                println!("parser: null");
                self.advance();
                return Literal(obj::null)
            },
            TType::LeftParen => {
                println!("parser: group");
                let expr = self.expression();
                if self.current().unwrap().ttype == TType::RightParen {
                    self.advance();
                } else {
                    ScrapError::error(
                        ParserError,
                        "Missing ')' ",
                        self.current().unwrap().line,
                        file!()
                    );
                }
                return Grouping(Box::new(expr))
            }

            _ => {

                return Literal(obj::null);
                std::process::exit(0);
            }
        }
    }

    //helper functions
    fn match_next(&mut self, tokens: &[TType]) -> bool {
        for ttype in tokens {
            if self.check(ttype) {
                self.advance();
                return true
            };
        }
        false
    }

    fn advance(&mut self)  {
        println!("before advance is at end");
        if !self.is_at_end() {
            self.index += 1;
            println!("advance {:?}", self.current().unwrap());
            println!("advance {:?}", self.index);
            self.current_token = self.previous();
        }
    }
    fn is_at_end(&mut self) -> bool {
        if self.peek().unwrap().ttype == Eof {
            println!("is at end");
            true
        } else {
            println!("is not at end");
            false

        }
    }

    fn current(&mut self) -> Option<Token>{
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
    fn check(&mut self, ttype: &TType) -> bool {
        if !self.is_at_end() {
            return false
        }
        &self.peek().unwrap().ttype == ttype
    }
    fn peek(&mut self) -> Option<&Token> {
        if !self.is_at_end() {
            Some(&self.tokens[self.index])
        } else {
            None
        }
    }
}
