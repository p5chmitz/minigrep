//! This should be front and center even though its from the src/lib.rs file.

use std::{env, error::Error, fs};

/// Main program logic
/// Uses the contains() method in a for loop to match against the query.
/// If a match is found, the line is pushed to the results vector */
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}
/// This code is run if the IGNORE_CASE flag is passed at runtime
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}
/// REFACTOR!!
/// The following primary search logic is refactored to use iterator adapters instead
/// of the for loop
/// Main program logic
/// Uses the contains() method in a for loop to match against the query.
/// If a match is found, the line is pushed to the results vector
pub fn search_2<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
/** This code is run if the IGNORE_CASE flag is passed at runtime */
pub fn search_case_insensitive_2<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}
impl Config {
    /** Parses the command line arguments passed to the function at runtime
    This version uses array indexes to deal with the program arguments */
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
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
    /** Parses the command line arguments passed tot he function at runtime
    This version processes the iterator directly using the next()
    function for the Iterator trait */
    pub fn build_2(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // Skips the default argument (zero-indexed program name)
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing or illegal query string"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("No file path"),
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

/** This function actually runs the logic component
 * contained in search(). On success this returns a unit
 * type containing the contents of the file path */
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Attempts to open the file at the argument path
    let contents = fs::read_to_string(&config.file_path)?;

    println!("query: {}\npath: {}", config.query, config.file_path);
    //println!("Contents of file:\n\n{}", contents);

    // Checks for the ignore case flag
    let results = if config.ignore_case {
        println!("Searching case insensitive...");
        search_case_insensitive_2(&config.query, &contents)
    } else {
        println!("Searching case sensitive...");
        search_2(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    // Indicates that we're only using the run()
    // function for its side effects, e.g. its a "void"
    // type function that propagates its errors upstream
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test text
    const CONTENTS: &str = "\
Rust:
safe, fast, productive.
Pick three.
DrEaRY Duct tape.
Trust me.";

    #[test]
    // Test to ensure that "duct" is ignored because its "DrEaRY Duct tape." in the CONTENTS
    // However, "duct" does match a slice of "productive", so it should return that line.
    fn case_sensitive() {
        let query = "duct";
        assert_eq!(vec!["safe, fast, productive."], search(query, CONTENTS));
    }

    #[test]
    // Test to ensure searching for "rUsT" returns all instances and substrings,
    // including "Rust:" and "Trust me."
    fn case_insensitive() {
        let query = "rUsT";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, CONTENTS)
        );
    }
}
