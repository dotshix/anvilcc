#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Keyword {
    Int,
    Void,
    Return,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Kind {
    LParen, // (
    RParen, // )
    LBrace, // {
    RBrace, // }
    Semicolon,

    // keywords
    KwInt,
    KwVoid,
    KwReturn,

    Identifier,
    Constant,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenValue {
    None,
    Number(i64),
    String(String),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Token {
    // Type
    pub kind: Kind,

    // start in source
    pub start: usize,

    // end in source
    pub end: usize,

    pub value: TokenValue,
}
