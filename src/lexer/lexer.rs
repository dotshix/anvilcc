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

    pub fn length(&self) -> usize {
        self.end - self.start
    }

    pub fn get_start(&self) -> usize {
        self.start
    }
    pub fn get_end(&self) -> usize {
        self.end
    }
    pub fn get_literal(&self) -> String {
        self.literal.clone()
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

    pub fn read_number(&mut self) -> Token {
        let start = self.current_pos;

        // consume
        while matches!(self.current_char(), Some(c) if c.is_ascii_digit()) {
            self.advance();
        }

        let end = self.current_pos;

        // slice out the literal text
        let literal = self.src[start..end].to_string();
        let value = literal.parse::<i64>().unwrap(); // assume valid for now

        let span = TextSpan::new(start, end, literal);

        Token {
            kind: TokenKind::Number(value),
            span,
        }
    }
}
