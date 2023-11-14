use std::io::ErrorKind::InvalidData;
use std::thread::current;
use crate::Token;
use crate::ast::{Expr};
use crate::ast::Expr::{Binary, Grouping, Literal, Unary};
use crate::error::ScrapError;
use crate::error::ScrapError::{InvalidSyntax, ParserError};
use crate::object::obj;
use crate::tokentype::TType;
use crate::tokentype::TType::{And, Bang, BangEqual, Echo, Eof, EqualEqual, False, Greater, GreaterEqual, LeftParen, Less, LessEqual, Minus, Null, Number, Or, Plus, Semicolon, Slash, Star, String_tok, True, While};

pub struct Parser {
    pub tokens: Vec<Token>,
    current_token: Option<Token>,
    index: usize,
    pub expressions: Vec<Expr>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        let current_token = tokens.get(0).cloned();
        Parser {
            tokens,
            current_token,
            index: 0,
            expressions: Vec::new(),
        }
    }
    pub fn parse(&mut self) {
        println!("at parse func: {:?}", self.peek().unwrap());
        let expr = self.statement();
        self.expressions.push(expr);
        if self.peek().unwrap().ttype == Semicolon {
            self.advance();
        } else {
            ScrapError::error(
                InvalidSyntax,
                "Missing semicolon",
                self.peek().unwrap().line,
                file!()
            );
        }

        println!("at parse func after advance: {:?}", self.peek().unwrap());
    }
    //parsing functions
    fn statement(&mut self) -> Expr {
        if self.match_next(&[Echo]) {
            return self.print_stmt()
        } else {
            return self.expression()
        }
    }
    fn print_stmt(&mut self) -> Expr {
        let value = self.expression();
        Expr::Print(Box::new(value))
    }
    fn expression(&mut self) -> Expr {
        let expr = self.or();
        return expr
    }

    fn or(&mut self) -> Expr {
        let mut expr = self.and();
        while self.match_next(&[Or]) {
            let operator = self.previous().unwrap().clone();
            let right = self.and();
            expr = Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right)
            };
            return expr
        }
        expr
    }

    fn and(&mut self) -> Expr {
        let mut expr = self.equality();
        while self.match_next(&[And]) {
            let operator = self.previous().unwrap().clone();
            let right = self.equality();
            expr = Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right)
            };
            return expr
        }
        expr
    }
    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        while self.match_next(&[BangEqual, EqualEqual]) {
            let operator = self.previous().unwrap().clone();
            let right = self.comparison();
            expr = Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right)
            };
            return expr
        }
        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();
        while self.match_next(&[Greater,GreaterEqual,Less,LessEqual]) {
            let operator = self.previous().unwrap().clone();
            let right = self.term();
            expr = Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right)
            };;
            return expr
        }
        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        while self.match_next(&[Plus, Minus]) {
            let operator = self.previous().unwrap().clone();
            let right = self.term();
            expr = Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right)
            };
            return expr
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        while self.match_next(&[Star, Slash]) {
            let operator = self.previous().unwrap().clone();
            let right = self.factor();
            expr = Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right)
            };
            return expr
        }
        expr
    }
    fn unary(&mut self) -> Expr {
        let expression: Expr = match self.peek().unwrap().ttype {
            Minus | Bang => {
                let operator = self.peek().unwrap().clone();
                self.advance();
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
        match self.peek().unwrap().ttype {
            TType::Number => {
                self.advance();
                return Literal(obj::num(self.previous().unwrap().literal.parse::<f64>().unwrap()))
            },
            TType::String_tok => {
                self.advance();
                return Literal(obj::str(self.previous().unwrap().clone().literal))
            },
            TType::True => {
                self.advance();
                return Literal(obj::bool(true))
            },
            TType::False => {
                self.advance();
                return Literal(obj::bool(false))
            },
            TType::Null => {
                self.advance();
                return Literal(obj::null)
            },
            TType::LeftParen => {
                let expr = self.expression();
                if self.peek().unwrap().ttype == TType::RightParen {
                    self.advance();
                } else {
                    ScrapError::error(
                        ParserError,
                        "Missing ')' ",
                        self.peek().unwrap().line,
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

    fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_end() {
            self.index += 1;
            self.current_token = self.previous().cloned();
        }
        self.previous()
    }
    fn is_at_end(&mut self) -> bool {
        self.peek().unwrap().ttype == Eof
    }

    fn previous(&mut self) -> Option<&Token> {
        if self.index != 0 {
            Some(&self.tokens[self.index - 1])
        } else {
            println!("[previous]: returned first token");
            Some(&self.tokens[self.index])
        }

    }
    fn check(&mut self, ttype: &TType) -> bool {
        if self.is_at_end() {
            return false
        }
        &self.peek().unwrap().ttype == ttype
    }
    fn peek(&mut self) -> Option<&Token> {
        Some(&self.tokens[self.index])
    }
}
