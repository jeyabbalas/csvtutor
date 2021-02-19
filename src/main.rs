use std::process;


fn main() {
    if let Err(e) = csvtutor::run() {
        println!("Oh ho, kai zhaale?: {}", e);
        process::exit(1);
    }
}
