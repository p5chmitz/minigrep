//! This is a doc comment from src/main.rs tho.

use minigrep::Config;
use std::{env, process};

/** This program searched for sub-strings based on arguments passed at runtime.
    To run the program enter `cargo run -- querytext file.extension`
    for case-sensitive searches. Pass invoke case-insensitive searches pass the
    environment variable IGNORE_CASE before the command like
    `IGNORE_CASE=true cargo run -- quertytext file.extension`
*/
fn main() {
    // The env::arts() function actually returns an iterator of type Args.
    // The iterator yields Strings for each argument and panics of the argument
    // is not valid Unicode. The iterator can either be collected into a vector of strings,
    // or passed directly to the build function to parse.

    // Collects the String args from the iterator into a vector
    //let args: Vec<String> = env::args().collect();
    //let config = Config::build(&args).unwrap_or_else(|err| {
    //    println!("Error parsing arguments: {err}");
    //    process::exit(1);
    //});

    // Does the same thing as above but uses a full match statement
    // instead of the unwrap_or_else function
    //let config = match Config::build(&args) {
    //    Ok(config) => config,
    //    Err(error) => {
    //        eprintln!("Error parsing arguments: {error}");
    //        process::exit(1);
    //    }
    //};

    // Passes the env::args() iterator directly to the build function
    let config = Config::build_2(env::args()).unwrap_or_else(|err| {
        eprintln!("Could not parse arguments: {err}");
        process::exit(1);
    });

    // Calls the core logic of the program and handles any propagated
    // errors from the main program logic contained in run()
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    };
}
