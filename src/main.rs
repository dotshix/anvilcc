mod driver;
mod args;
mod lexer;
mod token;

fn main() {
    if let Err(e) = driver::run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
