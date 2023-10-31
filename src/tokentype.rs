#[derive(Debug)]
pub enum TType {
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftCurly,
    RightCurly,

    Comma,
    Plus,
    Minus,
    Equal,
    EqualEqual,
    PlusEqual,
    MinusEqual,
    PlusPlus,
    MinusMinus,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    Bang,
    BangEqual,
    Semicolon,
    Dot,
    Star,
    Slash,

    Identifier,
    STRING,
    Number,

    And,
    Or,
    Var,
    Class,
    Else,
    ElseIf,
    If,
    For,
    While,
    Return,
    True,
    False,
    Null,
    Echo,
    Fn,

    Eof



}