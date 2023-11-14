use std::fmt::{Error, format};
use std::io;
use std::io::ErrorKind;
use std::ptr::write;
use crate::error::ScrapError;
use crate::error::ScrapError::ScannerError;
use crate::object::obj;
//use crate::Object::*;
use crate::token::Token;
use crate::tokentype::TType;
use crate::tokentype::TType::*;


pub struct Scanner {
    source: Vec<char>,
    pub tokens: Vec<Token>,
    start: usize,
    current:usize,
    line: usize,
    file: String
}


impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            file: source
        }
    }
    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()
                .expect("could not scan tokens");
        }

        self.tokens.push(Token::new(Eof,"".to_string(), None, self.line))
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
    fn scan_token(&mut self) -> Result<(), std::io::Error> {
        let c: char = self.advance();
        match c {
            '{' => self.add_token(LeftCurly),
            '}' => self.add_token(RightCurly),
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '[' => self.add_token(LeftBracket),
            ']' => self.add_token(RightBracket),
            ',' => self.add_token(Comma),
            ';' => self.add_token(Semicolon),
            '.' => self.add_token(Dot),
            '*' => self.add_token(Star),
            '$' => self.add_token(Var),
            '-' => {
                let token = if self.match_next('=') {
                    MinusEqual
                } else if self.match_next('-') {
                    MinusMinus
                } else {
                    Minus
                };
                self.add_token(token)
            },
            '+' =>  {
                let token = if self.match_next('=') {
                    PlusEqual
                } else if self.match_next('-') {
                    PlusPlus
                } else {
                    Plus
                };
                self.add_token(token)
            },
            '=' =>  {
                let token = if self.match_next('=') {
                    EqualEqual
                } else {
                    Equal
                };
                self.add_token(token)
            },
            '>' =>  {
                let token = if self.match_next('=') {
                    GreaterEqual
                } else {
                    Greater
                };
                self.add_token(token)
            },
            '<' =>  {
                let token = if self.match_next('=') {
                    LessEqual
                } else {
                    Less
                };
                self.add_token(token)
            },
            '!' =>  {
                let token = if self.match_next('=') {
                    BangEqual
                } else {
                    Bang
                };
                self.add_token(token)
            },
            '/' => {
                if self.match_next('/') {
                    while let Some(c) = self.peek() {
                        if self.peek() != Option::from('\n') {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                } else {
                    self.add_token(Slash)
                }
            },
            ' ' | '\r' | '\t' => {},
            '\n' => {
                self.line += 1;
            }
            '"' => {
                self.string()?;
            }
            '&' => {
                if self.match_next('&') {
                    self.add_token(And)
                }
            }
            '|' => {
                if self.match_next('|') {
                    self.add_token(Or)
                }
            }
            _ => {
                if Scanner::is_digit(Some(c)) {
                    self.number()
                } else if Scanner::is_alpha(Some(c)) {
                    self.identifier()
                } else {
                    let msg = format!("unexpected character: '{c}'");
                    ScrapError::error(ScannerError, msg.as_str() , self.line, file!());
                    io::Error::new(ErrorKind::Other, "unexpected character");
                }
                
            }
        }
        Ok(())
    }

    fn keywords(check: &str) -> Option<TType> {
        match check {
            "or" => Some(Or),
            "and" => Some(And),
            "if" => Some(If),
            "else" => Some(Else),
            "elseif" => Some(ElseIf),
            "true" => Some(True),
            "false" => Some(False),
            "while" => Some(While),
            "for" => Some(For),
            "fn" => Some(Fn),
            "class" => Some(Class),
            "return" => Some(Return),
            "echo" => Some(Echo),
            "null" => Some(Null),
            _ => {
                None
            }
        }
    }
    fn add_token(&mut self, ttype: TType) {
        self.add_token_object(ttype, None);
    }

    fn add_token_object(&mut self, ttype: TType, literal: Option<obj/*change this to object later*/>) {
        let lexeme: String = self.source[self.start..self.current].iter().collect();
        self.tokens
            .push(Token::new(ttype, lexeme, literal, self.line));
    }
    fn advance(&mut self) -> char {
        let result = *self.source.get(self.current).unwrap();
        self.current += 1;
        result
    }

    fn match_next(&mut self, expected: char) -> bool {
        match self.source.get(self.current) {
            Some(c) if *c == expected => {
                self.current += 1;
                true
            }
            _ => false
        }
    }
    fn peek(&self) -> Option<char> {
        self.source.get(self.current).copied()
    }

    fn peek_next(&self) -> Option<char> {
        self.source.get(self.current + 1).copied()
    }

    fn string(&mut self) -> Result<(), io::Error> {
        while let Some(c) = self.peek() {
            match c {
                '"' => {
                    break;
                },
                '\n' => {
                    self.line += 1;
                },
                _ => {}
            }
            self.advance();
        }
        if self.is_at_end() {
            io::Error::new(ErrorKind::Other, "unterminated string");
            ScrapError::error(ScannerError, format!("missing {}", '"').as_str(), self.line, file!())
        }
        let value: String = self.source[(self.start + 1)..(self.current)]
            .iter().collect();
        self.advance();
        self.add_token_object(TType::String_tok, Some(obj::str(value)));
        Ok(())
    }
    fn is_digit(c: Option<char>) -> bool {
        if c >= Some('0') && c <= Some('9') {
            true
        } else {
            false
        }
    }
    fn is_alpha(c: Option<char>) -> bool {
        if c >= Some('a') && c <= Some('z') ||
           c >= Some('A') && c <= Some('Z') ||
           c == Some('_') {
            true
        } else {
            false
        }
    }

    fn is_alphanumeric(c: Option<char>) -> bool {
        if Scanner::is_alpha(c) || Scanner::is_digit(c) {
            true
        } else {
            false
        }
    }
    fn number(&mut self) {
        while Scanner::is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == Some('.') && Scanner::is_digit(self.peek_next()) {
            self.advance();
        }
        let value: String = self.source[self.start..self.current].iter().collect();
        let num: f64 = value.parse().unwrap();
        self.add_token_object(Number, Some(obj::num(num)))
    }

    fn identifier(&mut self) {
        while Scanner::is_alphanumeric(self.peek()) {
            self.advance();
        }
        let text: String = self.source[self.start..self.current].iter().collect();
        if let Some(TType) = Scanner::keywords(text.as_str()) {
            self.add_token(TType);
        } else {
            self.add_token(Identifier);
        }
    }
}

