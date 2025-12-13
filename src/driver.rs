use crate::args::parse_args;
use crate::lexer::lexer::Lexer;
use crate::lexer::token::TokenKind;
use std::error::Error;
use std::path::PathBuf;

fn dump_tokens(src: &str) {
    let mut lex = Lexer::new(src);

    loop {
        let tok = lex.next_token();
        match &tok.kind {
            TokenKind::Number(n) => println!("Number: {} ({:?})", n, tok.span.get_literal()),
            TokenKind::Identifier => println!("Identifier: {}", tok.span.get_literal()),

            TokenKind::KwReturn => println!("Keyword: return"),
            TokenKind::KwInt => println!("Keyword: int"),
            TokenKind::KwVoid => println!("Keyword: void"),

            TokenKind::LParen => println!("LParen"),
            TokenKind::RParen => println!("RParen"),
            TokenKind::LBrace => println!("LBrace"),
            TokenKind::RBrace => println!("RBrace"),
            TokenKind::Semicolon => println!("Semicolon"),

            TokenKind::Eof => break,
            TokenKind::Error(msg) => {
                eprintln!(
                    "Error: {} at {}..{} (literal: {:?})",
                    msg,
                    tok.span.get_start(),
                    tok.span.get_end(),
                    tok.span.get_literal()
                );
                break;
            }
        }
    }
}

fn preprocessed_path(file: &PathBuf) -> Result<String, &'static str> {
    let stem = file.file_stem().ok_or("not a valid file path")?;
    Ok(format!("{}_PREPROCESSED.i", stem.to_string_lossy()))
}

pub fn run() -> Result<(), Box<dyn Error>> {
    // temporary demo input
    dump_tokens("int main(void) {\n\treturn 2;\n}");

    let args = parse_args()?;
    let out = preprocessed_path(&args.file)?;
    println!("{:?} \n{out}", args);

    Ok(())
}
