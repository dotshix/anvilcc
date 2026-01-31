use anvilcc::lexer::lexer::Lexer;
use anvilcc::lexer::token::TokenKind;

fn kinds(src: &str) -> Vec<TokenKind> {
    let mut lex = Lexer::new(src);
    let mut out = Vec::new();

    loop {
        let token = lex.next_token();

        out.push(token.kind);

        if matches!(out.last(), Some(TokenKind::Eof) | Some(TokenKind::Error(_))) {
            break;
        }
    }
    out
}

// valid
#[test]
fn line_comment_ignored() {
    let src = "int x; // hello\nreturn 2;";
    let got = kinds(src);

    assert_eq!(
        got,
        vec![
            TokenKind::KwInt,
            TokenKind::Identifier,
            TokenKind::Semicolon,
            TokenKind::KwReturn,
            TokenKind::Number(2),
            TokenKind::Semicolon,
            TokenKind::Eof,
        ]
    );
}

#[test]
fn block_comment_multiline_ignored() {
    let src = "return /* line1\nline2\n*/ 2;";
    let got = kinds(src);

    assert_eq!(
        got,
        vec![
            TokenKind::KwReturn,
            TokenKind::Number(2),
            TokenKind::Semicolon,
            TokenKind::Eof,
        ]
    );
}

#[test]
fn multiline_with_comment_and_multi_digit_constant() {
    let src = r#"
int main(void) {
    // test case w/ multi-digit constant
    return 100;
}
"#;

    let got = kinds(src);

    assert_eq!(
        got,
        vec![
            TokenKind::KwInt,
            TokenKind::Identifier, // main
            TokenKind::LParen,
            TokenKind::KwVoid,
            TokenKind::RParen,
            TokenKind::LBrace,
            TokenKind::KwReturn,
            TokenKind::Number(100),
            TokenKind::Semicolon,
            TokenKind::RBrace,
            TokenKind::Eof,
        ]
    );
}

#[test]
fn tokens_split_across_lines() {
    let src = r#"
int
main
(
void
)
{
return
0
;
}"#;

    let got = kinds(src);

    assert_eq!(
        got,
        vec![
            TokenKind::KwInt,
            TokenKind::Identifier, // main
            TokenKind::LParen,
            TokenKind::KwVoid,
            TokenKind::RParen,
            TokenKind::LBrace,
            TokenKind::KwReturn,
            TokenKind::Number(0),
            TokenKind::Semicolon,
            TokenKind::RBrace,
            TokenKind::Eof,
        ]
    );
}

#[test]
fn compact_program_no_whitespace() {
    let src = "int main(void){return 0;}";

    let got = kinds(src);

    assert_eq!(
        got,
        vec![
            TokenKind::KwInt,
            TokenKind::Identifier, // main
            TokenKind::LParen,
            TokenKind::KwVoid,
            TokenKind::RParen,
            TokenKind::LBrace,
            TokenKind::KwReturn,
            TokenKind::Number(0),
            TokenKind::Semicolon,
            TokenKind::RBrace,
            TokenKind::Eof,
        ]
    );
}

#[test]
fn simple_program_with_whitespace() {
    let src = r#"
int main(void) {
    return 0;
}
"#;

    let got = kinds(src);

    assert_eq!(
        got,
        vec![
            TokenKind::KwInt,
            TokenKind::Identifier, // main
            TokenKind::LParen,
            TokenKind::KwVoid,
            TokenKind::RParen,
            TokenKind::LBrace,
            TokenKind::KwReturn,
            TokenKind::Number(0),
            TokenKind::Semicolon,
            TokenKind::RBrace,
            TokenKind::Eof,
        ]
    );
}

#[test]
fn simple_program_return_two() {
    let src = r#"
int main(void) {
    return 2;
}
"#;

    let got = kinds(src);

    assert_eq!(
        got,
        vec![
            TokenKind::KwInt,
            TokenKind::Identifier, // main
            TokenKind::LParen,
            TokenKind::KwVoid,
            TokenKind::RParen,
            TokenKind::LBrace,
            TokenKind::KwReturn,
            TokenKind::Number(2),
            TokenKind::Semicolon,
            TokenKind::RBrace,
            TokenKind::Eof,
        ]
    );
}

#[test]
fn excessive_whitespace_everywhere() {
    let src = "   int   main    (  void)  {   return  0 ; }";

    let got = kinds(src);

    assert_eq!(
        got,
        vec![
            TokenKind::KwInt,
            TokenKind::Identifier, // main
            TokenKind::LParen,
            TokenKind::KwVoid,
            TokenKind::RParen,
            TokenKind::LBrace,
            TokenKind::KwReturn,
            TokenKind::Number(0),
            TokenKind::Semicolon,
            TokenKind::RBrace,
            TokenKind::Eof,
        ]
    );
}

#[test]
fn tabs_as_whitespace() {
    let src = "int\tmain\t(\tvoid)\t{\treturn\t0\t;\t}";

    let got = kinds(src);

    assert_eq!(
        got,
        vec![
            TokenKind::KwInt,
            TokenKind::Identifier, // main
            TokenKind::LParen,
            TokenKind::KwVoid,
            TokenKind::RParen,
            TokenKind::LBrace,
            TokenKind::KwReturn,
            TokenKind::Number(0),
            TokenKind::Semicolon,
            TokenKind::RBrace,
            TokenKind::Eof,
        ]
    );
}

// invalid
#[test]
fn invalid_at_sign_in_expression() {
    let src = r#"
/* The @ symbol doesn't appear in any C tokens,
   except inside string or character literals. */
int main(void) {
    return 0@1;
}
"#;

    let got = kinds(src);

    // We expect lexing to fail with an Error token
    assert!(
        matches!(got.last(), Some(TokenKind::Error(_))),
        "expected lex error, got: {:?}",
        got
    );
}

#[test]
fn invalid_single_backslash() {
    let src = r#"
/* A single backslash is not a valid token. */
\
"#;

    let got = kinds(src);

    // We expect lexing to fail with an Error token
    assert!(
        matches!(got.last(), Some(TokenKind::Error(_))),
        "expected lex error for single backslash, got: {:?}",
        got
    );
}

#[test]
fn invalid_backtick() {
    let src = r#"
/* A backtick is not a valid token. */
`[
"#;

    let got = kinds(src);

    assert!(
        matches!(got.last(), Some(TokenKind::Error(_))),
        "expected lex error for backtick, got: {:?}",
        got
    );
}

#[test]
fn invalid_identifier_starting_with_digit() {
    let src = r#"
/* '1foo' is not a valid token, because identifier can't start with digits. */
int main(void) {
    return 1foo;
}
"#;

    let got = kinds(src);

    assert!(
        matches!(got.last(), Some(TokenKind::Error(_))),
        "expected lex error for invalid token '1foo', got: {:?}",
        got
    );
}

#[test]
fn invalid_at_sign_followed_by_identifier() {
    let src = r#"
int main(void)
{
    return @b;
}
"#;

    let got = kinds(src);

    assert!(
        matches!(got.last(), Some(TokenKind::Error(_))),
        "expected lex error for '@b', got: {:?}",
        got
    );
}
