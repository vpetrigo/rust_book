extern crate minigrep;

use std::env;
use std::process;

#[allow(dead_code)]
enum AppStatus {
    Ok,
    QueryErr,
    AppErr
}

fn print_help() {
    eprintln!("Usage: mingrep <pattern> <file>");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = minigrep::Query::new(&args).unwrap_or_else(|err| {
        print_help();
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(AppStatus::QueryErr as i32);
    });

    if let Err(e) = minigrep::run(query) {
        eprintln!("Application error: {}", e);
        process::exit(AppStatus::AppErr as i32);
    }
}
