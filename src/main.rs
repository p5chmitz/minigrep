// Returns an iterator of the arguments passed to the command line
// The std::env captures the name of the program as an argument
// The std::fs opens the file and returns std::io::Result<String>
use minigrep::Config;
use std::{env, process};

fn main() {
    // Collects arguments passed to the program to a vector
    // and passes them to a configuration function
    let args: Vec<String> = env::args().collect();

    ////////////////////////////////////////
    // TESTIN ENVIORNMENT VARIABLES */
    //pub struct ENV {
    //    is_true: bool,
    //}
    //impl ENV {
    //    pub fn show(args: &[String]) -> Result<ENV, &'static str> {
    //        let is_true = env::var("IS_TRUE").is_ok();
    //        Ok(ENV { is_true })
    //    }
    //}
    ////////////////////////////////////////

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
            eprintln!("Error parsing arguments: {error}");
            process::exit(1);
        }
    };

    // Handles a propagated error from the main program logic
    // contained in run()
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    };
}
