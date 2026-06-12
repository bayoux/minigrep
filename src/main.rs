use std::{env, fs, process};

use minigrep::{Config, highlight, search};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let contents = fs::read_to_string(&config.file_path).unwrap_or_else(|err| {
        eprintln!("Error reading file: {err}");
        process::exit(1);
    });

    for (line_number, line) in search(&config.pattern, &contents, config.ignore_case) {
        println!("{line_number}: {}", highlight(line, &config.pattern, config.ignore_case));
    }
}
