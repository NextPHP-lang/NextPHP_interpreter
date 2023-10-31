use crate::Object::*;
use crate::token::Token;
use crate::tokentype::TType;
use crate::tokentype::TType::*;


pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current:usize,
    line:usize
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }
    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()
        }

        self.tokens.push(Token::new(Eof,"".to_string(), None, self.line))
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
    fn scan_token(&mut self) -> Result<(), Err()> {
        let c: char = self.advance();
        match c {
            '{' => self.add_token(LeftCurly),
            '}' => self.add_token(RightCurly),
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '[' => self.add_token(LeftBracket),
            ']' => self.add_token(RightBracket),
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
            ',' => self.add_token(Comma),
            ';' => self.add_token(Semicolon),
            '.' => self.add_token(Dot),
            '*' => self.add_token(Star),
            _ => {
                Err("unexpected character")
            }
        }
        Ok(())
    }
    fn add_token(&mut self, ttype: TType) {
        self.add_token_object(ttype, None);
    }

    fn add_token_object(&mut self, ttype: TType, literal: Option<()/*change this to object later*/>) {
        let lexeme: String = self.source[self.start..self.current].iter().collect();
        self.tokens
            .push(Token::new(ttype, lexeme, literal, self.line));
    }
    fn advance(&mut self) -> char {
        let result = self.source.get(self.current).unwrap;
        self.current += 1;
        result
    }

    fn match_next(&mut self, expected: char) -> bool {
        match self.source.get(self.source) {
            Some(c) if c == expected => {
                self.current += 1;
                true
            }
            _ => false
        }
    }
}