use std::env;
use std::process;

use rust_example::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let config = Config::parse(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = rust_example::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1)
    }
}
