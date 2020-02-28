extern crate gree;

use std::env;
use std::process;

use gree::config::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error: {}", err);
        process::exit(1);
    });

    if let Err(err) = gree::run(config) {
        println!("Error: {}", err);
        process::exit(1);
    }
}
