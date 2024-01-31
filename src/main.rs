// Returns an iterator of the arguments passed to the command line
// The std::env captures the name of the program as an argument
// The std::fs opens the file and returns std::io::Result<String>
use minigrep::Config;
use std::{env, process};

fn main() {
    // Collects arguments passed to the program to a vector
    // and passes them to a configuration function
    let args: Vec<String> = env::args().collect();

    // Uses unwrap_or_else() with a closure to bind return value
    //let config = Config::build(&args).unwrap_or_else(|err| {
    //    println!("Error parsing arguments: {err}");
    //    process::exit(1);
    //});
    // Does the same thing as above but uses a full match 
    // statement to bind results
    let config = match Config::build(&args) {
        Ok(config) => config,
        Err(error) => {
            println!("Error parsing arguments: {error}");
            process::exit(1);
        },
    };

    // Handles a propagated error from the main program logic
    // contained in run()
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    };
}
