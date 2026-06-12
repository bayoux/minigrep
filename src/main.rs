use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: minigrep <pattern> <file> [--ignore-case]");
        process::exit(1);
    }

    let pattern = &args[1];
    let file_path = &args[2];

    let ignore_case = args.iter().any(|arg| arg == "--ignore-case");

    let contents = fs::read_to_string(file_path).unwrap_or_else(|err| {
        eprintln!("Error reading file: {err}");
        process::exit(1);
    });

    for (line_number, line) in search(pattern, &contents, ignore_case) {
        println!("{line_number}: {line}");
    }
}

fn search<'a>(pattern: &str, contents: &'a str, ignore_case: bool) -> Vec<(usize, &'a str)> {
    let mut results = Vec::new();

    let pattern = if ignore_case {
        pattern.to_lowercase()
    } else {
        pattern.to_string()
    };

    for (i, line) in contents.lines().enumerate() {
        let line_to_check = if ignore_case {
            line.to_lowercase()
        } else {
            line.to_string()
        };

        if line_to_check.contains(&pattern) {
            results.push((i + 1, line));
        }
    }

    results
}
