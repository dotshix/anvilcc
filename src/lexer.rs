use super::token::{Token, TokenKind};

pub struct TextSpan {
    start: usize,
    end: usize,
    literal: String,
}

impl TextSpan {
    pub fn new(start: usize, end: usize, literal: String) -> Self {
        Self {
            start,
            end,
            literal,
        }
    }

    // comment for now
    // pub fn length(&self) -> usize {
    //     self.end - self.start
    // }

    pub fn get_start(&self) -> usize {
        self.start
    }
    pub fn get_end(&self) -> usize {
        self.end
    }
    pub fn get_literal(&self) -> &str {
        &self.literal
    }
}

pub struct Lexer<'a> {
    src: &'a str,
    current_pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src,
            current_pos: 0,
        }
    }

    fn advance(&mut self) {
        if let Some(ch) = self.current_char() {
            self.current_pos += ch.len_utf8();
        }
    }

    fn current_char(&self) -> Option<char> {
        self.src[self.current_pos..].chars().next()
    }

    fn peek_next(&self) -> Option<char> {
        self.src[self.current_pos..].chars().nth(1)
    }

    pub fn read_number(&mut self) -> Token {
        let start = self.current_pos;

        while matches!(self.current_char(), Some(c) if c.is_ascii_digit()) {
            self.advance();
        }

        if matches!(self.current_char(), Some(c) if c.is_ascii_alphanumeric() || c == '_') {
            while matches!(self.current_char(), Some(c) if     c.is_ascii_alphanumeric() || c == '_')
            {
                self.advance();
            }
            let bad_end = self.current_pos;
            let lit = self.src[start..bad_end].to_string();
            return Token {
                kind: TokenKind::Error(
                    "invalid constant (digits followed by identifier char)".into(),
                ),
                span: TextSpan::new(start, bad_end, lit),
            };
        }
        let end = self.current_pos;
        let literal = self.src[start..end].to_string();

        match literal.parse::<i64>() {
            Ok(value) => Token {
                kind: TokenKind::Number(value),
                span: TextSpan::new(start, end, literal),
            },
            Err(_) => Token {
                kind: TokenKind::Error("integer error".into()),
                span: TextSpan::new(start, end, literal),
            },
        }
    }

    fn single_char(&mut self, kind: TokenKind) -> Token {
        let start = self.current_pos;
        let c = self.current_char().unwrap();
        self.advance();
        let end = self.current_pos;

        let span = TextSpan::new(start, end, c.to_string());
        Token { kind, span }
    }

    pub fn read_kw_or_identifier(&mut self) -> Token {
        let start = self.current_pos;

        // consume
        while matches!(self.current_char(), Some(c) if c.is_ascii_alphanumeric() || c == '_') {
            self.advance();
        }

        let end = self.current_pos;

        // slice
        let literal = self.src[start..end].to_string();

        let kind = match literal.as_str() {
            "int" => TokenKind::KwInt,
            "void" => TokenKind::KwVoid,
            "return" => TokenKind::KwReturn,
            _ => TokenKind::Identifier,
        };

        let span = TextSpan::new(start, end, literal);

        Token { kind, span }
    }

    fn skip_trivia(&mut self) {
        loop {
            match (self.current_char(), self.peek_next()) {
                //whitepsace
                (Some(c), _) if c.is_whitespace() => {
                    self.advance();
                }

                // line comment
                (Some('/'), Some('/')) => {
                    self.advance();
                    self.advance();

                    while matches!(self.current_char(), Some(c) if c != '\n') {
                        self.advance();
                    }
                }

                // block comment /*  */
                (Some('/'), Some('*')) => {
                    self.advance();
                    self.advance();

                    // consume until */
                    loop {
                        match (self.current_char(), self.peek_next()) {
                            (Some('*'), Some('/')) => {
                                self.advance();
                                self.advance();

                                break;
                            }

                            // anything else
                            (Some(_), _) => {
                                self.advance();
                            }

                            (None, _) => break,
                        }
                    }
                }

                _ => break,
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_trivia();

        let start = self.current_pos;

        match self.current_char() {
            None => {
                let span = TextSpan::new(start, start, String::new());
                Token {
                    kind: TokenKind::Eof,
                    span,
                }
            }
            Some(c) if c.is_ascii_digit() => self.read_number(),

            Some(c) if c.is_ascii_alphabetic() || c == '_' => self.read_kw_or_identifier(),

            Some('(') => self.single_char(TokenKind::LParen),
            Some(')') => self.single_char(TokenKind::RParen),
            Some('{') => self.single_char(TokenKind::LBrace),
            Some('}') => self.single_char(TokenKind::RBrace),
            Some(';') => self.single_char(TokenKind::Semicolon),
            Some(c) => {
                self.advance();

                let literal = c.to_string();
                let end = self.current_pos;
                let span = TextSpan::new(start, end, literal.clone());

                Token {
                    kind: TokenKind::Error(format!("unexpected character `{}`", c)),
                    span,
                }
            }
        }
    }
}
