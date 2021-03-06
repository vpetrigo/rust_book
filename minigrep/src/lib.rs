use std::error::Error;
use std::fs;
use std::env;

#[derive(Debug)]
pub struct Query {
    pattern: String,
    file: String,
    case_sensitive: bool,
}

impl Query {
    pub fn new(args: std::env::Args) -> Result<Query, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        // skip the program name argument
        let mut parameters = args.skip(1);
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        let query = parameters.next().ok_or("Unable to get query parameter")?;
        let filename = parameters.next().ok_or("Unable to get filename")?;

        Ok(Query { pattern: query, file: filename, case_sensitive })
    }
}

pub fn run(query: Query) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(query.file)?;
    let result = if query.case_sensitive {
        search(&query.pattern, &content)
    } else {
        search_case_insensitive(&query.pattern, &content)
    };

    for line in result {
        println!("{}", line);
    }

    Ok(())
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content.lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content.lines()
        .filter(|line|
            line.to_lowercase().as_str().contains(query.to_lowercase().as_str()))
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
