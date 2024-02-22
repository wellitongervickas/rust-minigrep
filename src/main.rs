use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // @dev get cmd line args
    let args: env::Args = env::args();
    
    let config: Config = Config::build(args).unwrap_or_else(|err: &str| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });


    // is it like try catch?
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

