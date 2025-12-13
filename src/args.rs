use std::path::PathBuf;

#[derive(Debug)]
pub struct AppArgs {
    pub file: PathBuf,
}

pub fn parse_args() -> Result<AppArgs, lexopt::Error> {
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
