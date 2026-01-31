use crate::args::parse_args;
use crate::args::Stage;
use crate::lexer::Lexer;
use crate::token::TokenKind;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

fn dump_tokens(src: &str) -> Result<(), Box<dyn Error>> {
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

            TokenKind::Eof => return Ok(()),
            TokenKind::Error(msg) => {
                eprintln!(
                    "Error: {} at {}..{} (literal: {:?})",
                    msg,
                    tok.span.get_start(),
                    tok.span.get_end(),
                    tok.span.get_literal()
                );
                return Err(msg.clone().into());
            }
        }
    }
}

fn preprocessed_path(file: &Path) -> Result<PathBuf, &'static str> {
    let stem = file.file_stem().ok_or("not a valid file path")?;
    Ok(file.with_file_name(format!("{}_PREPROCESSED.i", stem.to_string_lossy())))
}

fn asm_path(file: &Path) -> Result<PathBuf, &'static str> {
    let stem = file.file_stem().ok_or("not a valid file path")?;
    Ok(file.with_file_name(format!("{}.s", stem.to_string_lossy())))
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let args = parse_args()?;
    let pre_process = preprocessed_path(&args.file)?;

    // gcc -E -P INPUT_FILE -o PREPROCESSED_FILE

    let status = Command::new("gcc")
        .arg("-E")
        .arg("-P")
        .arg(&args.file)
        .arg("-o")
        .arg(&pre_process)
        .status()?;

    if !status.success() {
        return Err("gcc preprocessing failed".into());
    }

    let src = fs::read_to_string(&pre_process)?;
    dump_tokens(&src)?;

    // STAGE GATES (stop early)
    match args.stage {
        Stage::Lex => {
            let _ = fs::remove_file(&pre_process);
            return Ok(());
        }
        Stage::Parse => {
            // parse not implemented yet; stop here anyway
            let _ = fs::remove_file(&pre_process);
            return Ok(());
        }
        _ => {} // Codegen / Run continue
    }

    // stub compiler
    let asm = asm_path(&args.file)?;

    let asm_text = "\
    .globl main
    main:
         mov $0, %eax
         ret
";

    fs::write(&asm, asm_text)?;

    if matches!(args.stage, Stage::Codegen) {
        let _ = fs::remove_file(&pre_process);
        let _ = fs::remove_file(&asm);
        return Ok(());
    }
    // build executable
    let exe = args.file.with_extension("");

    let status = Command::new("gcc").arg(&asm).arg("-o").arg(&exe).status()?;

    if !status.success() {
        // cleanup pre file

        let _ = fs::remove_file(&pre_process);
        return Err("gcc failed to assemble/link".into());
    }

    let _ = fs::remove_file(&pre_process);
    let _ = fs::remove_file(&asm);

    Ok(())
}
