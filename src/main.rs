use std::env;
use std::process;

use minigrep::Config;
use minigrep::run;

fn main() {
    // Collecting the command line arguments into a vector
    let args: Vec<String> = env::args().collect();

    // env::args() provides an iterator over the arguments
    let config = Config::build(env::args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}