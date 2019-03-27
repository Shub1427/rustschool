use std::env;
use std::process;

use minigrep;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = minigrep::Config::new(&args).unwrap_or_else(|err| {
    	println!("Problem parsing arguments: {}", err);
    	process::exit(1);
    });
    minigrep::run(config);
}