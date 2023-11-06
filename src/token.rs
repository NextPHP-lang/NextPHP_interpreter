use crate::object::obj;
use crate::tokentype::TType;

#[derive(Debug)]
pub struct Token {
    pub ttype: TType,
    pub literal: String,
    pub lexeme: Option<obj>,
    line: usize,
}

impl Token {
    pub fn new(ttype: TType, literal: String, lexeme: Option<obj>, line: usize) -> Token {
        Token {
            ttype,
            literal,
            lexeme,
            line
        }
    }
}