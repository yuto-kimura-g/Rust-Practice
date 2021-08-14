extern crate mini_grep;

use std::env;
use std::process;

use mini_grep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    eprintln!("\nargs: {:?}", args);

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("err at parsing arguments: {}", err);
        process::exit(1);
    });
    println!("searching for [{}] in [{}]", config.query, config.filename);

    if let Err(e) = mini_grep::run(config) {
        eprintln!("application err: {}", e);
        process::exit(1);
    }
    return;
}
