use std::{error::Error, fs};

pub struct Config {
    query: String,
    file_path: String,
}
impl Config {
    // Parses the command line arguments passed to the function at runtime
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // Checks for 3 arguments because args() inherently 
        // captures program name as the first
        if args.len() < 3 {
            return Err("Insufficient arugments; need query and file path");
        }
        // Prints a reminder instead of panicking
        if args.len() > 3 {
            println!("NOTE: Only the first two arguments are read for query and file path");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

// On success this returns a unit type containing the 
// contents of the file path
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Attempts to open the file at the argument path
    let contents = fs::read_to_string(&config.file_path)?;

    println!("query: {}\npath: {}", config.query, config.file_path);
    println!("Contents of file:\n\n{}", contents);

    // Indicates that we're only using the run() function for its side effects
    Ok(())
}
