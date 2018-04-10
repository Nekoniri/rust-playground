use std::{env, error::Error, fs::File, io::prelude::*};

#[derive(Debug, PartialEq)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        let mut args = args.into_iter().skip(1);

        let query = match args.next() {
            Some(arg) => arg.to_string(),
            None => return Err("query argument not provided")
        };

        let filename = match args.next() {
            Some(arg) => arg.to_string(),
            None => return Err("filename argument not provided")
        };

        let case_sensitive = match args.next() {
            Some(arg) => arg != "-i",
            None => env::var("CASE_INSENSITIVE").is_err()
        };

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut contents = String::new();
    let mut file = File::open(&config.filename)?;

    file.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Add tests for case sensitivity flag/env
    // Test if config is properly set:
    // If `-i` is provided `case_sensitive` should be false
    // If `-i` is not provided:
    //   Check if `CASE_INSENSITIVE` is set:
    //   If set `case_sensitive` should be false
    //   If not set `case_sensitive` should be true

    #[test]
    fn case_sensitive_search() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive_search() {
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

    #[test]
    fn config_errors_on_invalid_arg_count() {
        let args = [String::from("path")];

        assert!(Config::new(&args).is_err());
    }

    #[test]
    fn config_contains_correct_values() {
        let args = [
            String::from("path"),
            String::from("query"),
            String::from("filename"),
        ];

        let config = Config::new(&args).unwrap();

        assert_eq!(config.query, String::from("query"));
        assert_eq!(config.filename, String::from("filename"));
    }

    #[test]
    fn can_run_with_valid_config() {
        let args = [
            String::from("path"),
            String::from("query"),
            String::from("poem.txt"),
        ];

        let config = Config::new(&args).unwrap();

        assert!(run(config).is_ok());
    }

    #[test]
    #[should_panic]
    fn cannot_run_with_invalid_config() {
        let args = [
            String::from("path"),
            String::from("query"),
            String::from("file_does_not_exist.txt"),
        ];

        let config = Config::new(&args).unwrap();

        run(config).unwrap();
    }
}
