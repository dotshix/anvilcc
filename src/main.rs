mod args;
mod driver;
mod lexer;

fn main() {
    if let Err(e) = driver::run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
