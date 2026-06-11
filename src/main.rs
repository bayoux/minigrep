use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: minigrep <pattern> <file>");
        process::exit(1);
    }

    let pattern = &args[1];
    let file_path = &args[2];

    let contents = fs::read_to_string(file_path).unwrap_or_else(|err| {
        eprintln!("Error reading file: {err}");
        process::exit(1);
    });

    let results = search(pattern, &contents);
    
    for line in results {
        println!("{line}");
    }
}

fn search<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(pattern) {
            results.push(line);
        }
    }

    results
}
