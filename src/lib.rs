use std::env;
use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool
}

impl Config {
    // @dev always return string with lifetime 'static
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("missing arguments");
        }
        
        let query: String= args[1].clone();
        let file_path: String= args[2].clone();
        let ignore_case: bool = env::var("IGNORE_CASE").is_ok();

        Ok(Config {query, file_path, ignore_case})
    }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // @dev Result<String,Error>
    let contents: String = fs::read_to_string(config.file_path)?;

    let results: Vec<&str> = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}")
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();

    // @dev shadow query
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
// @dev {Tests}
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

        let expected_line: &str = "safe, fast, productive.";
        assert_eq!(vec![expected_line], search(query, contents));
    }
    
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}