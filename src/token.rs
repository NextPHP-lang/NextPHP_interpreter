use crate::tokentype::TType;

#[derive(Debug)]
pub struct Token {
    ttype: TType,
    literal: String,
    lexeme: Option<()>,
    line: usize,
}

impl Token {
    pub fn new(ttype: TType, literal: String, lexeme: Option<()>, line: usize) -> Token {
        Token {
            ttype,
            literal,
            lexeme,
            line
        }
    }
}