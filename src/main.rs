use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // @dev get cmd line args
    let args: Vec<String> = env::args().collect();
    
    let config: Config = Config::build(&args).unwrap_or_else(|err: &str| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // is it like try catch?
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

