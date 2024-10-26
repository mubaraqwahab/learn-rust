use std::env;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("missing query argument"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("missing file path argument"),
        };

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

// TODO: find out if rust fns support variadic args

/// Returns all lines in `contents` that contain `query`.
///
/// The search is case sensitive. For case insensitive search,
/// use [search_case_insensitively] instead.
///
/// # Examples
///
/// ```
/// let contents = vec!["hello", "hey", "hi"].join("\n");
/// assert_eq!(minigrep::search("he", &contents), vec!["hello", "hey"]);
/// ```
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
