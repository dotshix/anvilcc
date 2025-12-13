use super::lexer::TextSpan;

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

impl Token {
    fn new(kind: TokenKind, span: TextSpan) -> Self {
        Token { kind, span }
    }
}
