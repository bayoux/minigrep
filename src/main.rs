use std::{env, fs, process};

use clap::Parser;
use minigrep::{Config, highlight, search};

fn main() {
    let mut config = Config::parse();
    if env::var("IGNORE_CASE").is_ok() {
        config.ignore_case = true;
    }

    let contents = fs::read_to_string(&config.file_path).unwrap_or_else(|err| {
        eprintln!("Error reading file: {err}");
        process::exit(1);
    });

    for (line_number, line) in search(&config.pattern, &contents, config.ignore_case) {
        println!("{line_number}: {}", highlight(line, &config.pattern, config.ignore_case));
    }
}
