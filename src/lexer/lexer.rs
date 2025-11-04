use std::str::Chars;

struct Lexer<'a> {
    // source text
    source: &'a str,

    // remaining text
    chars: Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.chars(),
        }
    }

    // get how many bytes we've advanced
    fn offset(&self) -> usize {
        self.source.len() - self.chars.as_str().len()
    }

    fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.chars.next();
            } else {
                break;
            }
        }
    }
}
