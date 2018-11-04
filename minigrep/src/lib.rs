use std::error::Error;
use std::fs;
use std::env;

#[derive(Debug)]
pub struct Query<'a> {
    pattern: &'a String,
    file: &'a String,
    case_sensitive: bool,
}

impl<'a> Query<'a> {
    pub fn new(args: &[String]) -> Result<Query, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Query { pattern: &args[1], file: &args[2], case_sensitive })
    }
}

pub fn run(query: Query) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(query.file)?;
    let result = if query.case_sensitive {
        search(query.pattern, &content)
    }
    else {
        search_case_insensitive(query.pattern, &content)
    };

    for line in result {
        println!("{}", line);
    }

    Ok(())
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(query.to_lowercase().as_str()) {
            result.push(line);
        }
    }

    result
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

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
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