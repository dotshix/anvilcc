use std::{path::PathBuf, process::Command};

mod lexer;
mod token;

#[derive(Debug)]
struct AppArgs {
    file: PathBuf,
}

fn parse_args() -> Result<AppArgs, lexopt::Error> {
    use lexopt::prelude::*;

    let mut file = None;

    let mut parser = lexopt::Parser::from_env();
    while let Some(arg) = parser.next()? {
        match arg {
            Value(path) if file.is_none() => file = Some(path.into()),
            _ => return Err(arg.unexpected()),
        }
    }
    Ok(AppArgs {
        file: file.ok_or("missing file path")?,
    })
}

fn main() {
    //let path = Path::new("./foo/bar.txt");
    let args = match parse_args() {
        Ok(args) => args,
        Err(err) => {
            eprintln!("Error: {}.", err);
            std::process::exit(1);
        }
    };

    let file_path = format!(
        "{}_PREPROCESSED.i",
        args.file
            .file_stem()
            .expect("Not a valid file")
            .to_string_lossy()
    );

    // run preprocess
    // let preprocess = Command::new("gcc")
    //     .arg("-E")
    //     .arg("-P")
    //     .arg(&args.file)
    //     .arg("-o")
    //     .arg(&file_path)
    //     .spawn()
    //     .expect("gcc failed to start");

    // compile using anvil
    // Needs to do
    // delete preprocessed file

    // assemble and link, todo

    println!("{file_path}");
    println!("Hello, world!");
}
