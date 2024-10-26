use std::env;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() != 3 {
            return Err("two arguments expected");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Self {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), String> {
    let Config {
        file_path,
        query,
        ignore_case,
    } = config;

    let contents = match fs::read_to_string(&file_path) {
        Ok(text) => text,
        Err(error) => return Err(format!("failed to read file {file_path}: {error}",)),
    };

    let results = if ignore_case {
        search_case_insensitively(&query, &contents)
    } else {
        search(&query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

// TODO: find out if rust fns support default args
// TODO: find out how to write logical operators in rust
// TODO: find out if rust fns support keyword args
// TODO: find out if rust fns support variadic args

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitively<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let lower_query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&lower_query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

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
            search_case_insensitively(query, contents)
        );
    }
}
