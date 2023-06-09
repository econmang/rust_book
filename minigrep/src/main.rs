use std::env;
use std::process;
use minigrep::parser::Config;

// eprintln!() will print to standard error
// println!() will print to standard output

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = Config::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
