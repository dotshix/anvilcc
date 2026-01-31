use super::lexer::TextSpan;

#[derive(Debug, PartialEq)]
pub enum TokenKind {
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
    Number(i64),

    Eof,
    Error(String),
}

pub struct Token {
    // Type
    pub kind: TokenKind,
    pub span: TextSpan,
}
