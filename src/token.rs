use crate::tokentype::TType;

#[derive(Debug)]
pub struct Token {
    ttype: TType,
    literal: String,
    lexeme: (),
    line: i32
}

impl Token {
    pub fn new(ttype: TType, literal: String, lexeme: (), line: i32) -> Token {
        Token {
            ttype,
            literal,
            lexeme,
            line
        }
    }
}