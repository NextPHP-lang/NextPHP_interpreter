


use crate::Token;
use crate::ast::{Expr, Stmt};
use crate::ast::Expr::{Binary, Grouping, Literal, Unary};
/*use crate::ast::Stmt::Block;*/
use crate::error::ScrapError;
use crate::error::ScrapError::{InvalidSyntax, ParserError};
use crate::object::obj;
use crate::tokentype::TType;
use crate::tokentype::TType::{And, Bang, BangEqual, Echo, Else, Eof, Equal, EqualEqual, False, Greater, GreaterEqual, Identifier, If, LeftCurly, LeftParen, Less, LessEqual, Minus, Null, Number, Or, Plus, RightCurly, Semicolon, Slash, Star, String_tok, True, Var, While};

pub struct Parser {
    pub tokens: Vec<Token>,
    current_token: Option<Token>,
    index: usize,
    pub expressions: Vec<Expr>,
    pub statements: Vec<Stmt>
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        let current_token = tokens.get(0).cloned();
        Parser {
            tokens,
            current_token,
            index: 0,
            expressions: Vec::new(),
            statements: Vec::new(),
        }
    }
    pub fn parse(&mut self) {
        while !self.is_at_end() {
            let expr = self.declaration();
            self.statements.push(expr.clone()); //remove clone when removing debug println
            let semicolons: Vec<TType> = vec![Semicolon,LeftCurly,RightCurly];
            if semicolons.contains(&self.peek().unwrap().ttype) {
                self.advance();
            } else {
                /*println!("prev token: {:?}", self.previous());
                println!("current token: {:?}", self.peek());*/
                ScrapError::error(
                    InvalidSyntax,
                    "Missing semicolon",
                    self.peek().unwrap().line,
                    file!()
                );
            }
            println!("stmt: {:#?}", &expr)
        }

    }
    //parsing functions
    fn declaration(&mut self) -> Stmt {
        if self.match_next(&[Var]) {
            return self.variable_declaration();
        } else {
            return self.statement();
        }
    }
    fn statement(&mut self) -> Stmt {
        // println!("{} returned statement", self.current_token.clone().unwrap().line);
        if self.match_next(&[Echo]) {
            return self.print_stmt()
        } else if self.match_next(&[If]) {
            // println!("reached if in statement");
            let test = self.if_stmt();
            // println!("{:#?}", test);
            return test;
        } else if self.match_next(&[While]) {
            return self.while_stmt();
        } else {
            return Stmt::Expression(Box::new(self.expression()));
        }
    }
    fn variable_declaration(&mut self) -> Stmt {
        // println!("{:?}", self.current_token);
        let operations = [Equal, Plus, Minus, Star, Slash];
        let mut val = Expr::Literal(obj::Null);
        let mut identifier= String::new();
        let mut statement = Stmt::Expression(Box::new(Literal(obj::Null)));
        if self.match_next(&[Identifier]) {
            // println!("{:?}", self.current_token);
            identifier = self.previous().unwrap().literal.clone();
            if self.match_next(&[Equal]) {
                val = self.expression();
                statement =  Stmt::VariableAssign {
                    identifier,
                    value: Box::new(val)
                };
            } else {
                statement = Stmt::VariableCall {identifier};
            }
        }
        statement
    }
    fn if_stmt(&mut self) -> Stmt {
        let expr = Box::new(self.expression());
        let mut block = self.declaration();
        let mut block2 = self.declaration();
        if self.match_next(&[LeftCurly]) {
            let mut stmts = Vec::new();
            while !self.check(&RightCurly) {
                let expr = self.declaration();

                stmts.push(expr);
                self.advance();
            }

            block = Stmt::Block(stmts);
        }
        self.advance();
        if self.match_next(&[Else]) {
            if self.match_next(&[LeftCurly]) {
                let mut stmts = Vec::new();
                while !self.check(&RightCurly) {
                    let expr = self.declaration();

                    stmts.push(expr);
                    self.advance();
                }
                block2 = Stmt::Block(stmts);

            }
            Stmt::Ifstmt {
                expr,
                block: Box::new(block),
                elseblock: Some(Box::new(block2))
            }
        } else {
            Stmt::Ifstmt {
                expr,
                block: Box::new(block),
                elseblock: None
            }
        }

    }
    fn while_stmt(&mut self) -> Stmt {
        let expr = Box::new(self.expression());
        let mut block = self.declaration();
        if self.match_next(&[LeftCurly]) {
            let mut stmts = Vec::new();
            while !self.check(&RightCurly) {
                let expr = self.declaration();

                stmts.push(expr);
                self.advance();
            }

            block = Stmt::Block(stmts);
        }
        Stmt::WhileStmt {
            expr,
            block: Box::new(block)
        }
    }
    fn print_stmt(&mut self) -> Stmt {
        let value = self.declaration();
        Stmt::Print(Box::new(value))
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
            };
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
            Number | Null | String_tok | True | False | Identifier | LeftParen => {
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
                return Literal(obj::Num(self.previous().unwrap().literal.parse::<f64>().unwrap()))
            },
            TType::String_tok => {
                self.advance();
                return Literal(obj::Str(self.previous().unwrap().clone().literal))
            },
            TType::True => {
                self.advance();
                return Literal(obj::Bool(true))
            },
            TType::False => {
                self.advance();
                return Literal(obj::Bool(false))
            },
            TType::Null => {
                self.advance();
                return Literal(obj::Null)
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
            },
            TType::Identifier => {
                self.advance();
                return Literal(obj::Identifier(self.previous().unwrap().literal.clone()))
            }
            _ => {
                return Literal(obj::Null);
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
