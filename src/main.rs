use std::{env, process};
use csvtutor::Config;


fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error reading input arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = csvtutor::run(config) {
        eprintln!("Error reading file: {}", e);
        process::exit(1);
    }
}
