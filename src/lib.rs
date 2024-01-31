use std::{error::Error, fs};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}

pub fn sampel(a: &str, b: &str) -> &str {
    println!("{b}");
    a
}

/**  */
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}

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

    // Indicates that we're only using the run() 
    // function for its side effects, e.g. its a "void"
    // type function that propagates its errors upstream
    Ok(())
}

