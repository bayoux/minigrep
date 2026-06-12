use std::{env, fs, process, thread};

use clap::Parser;
use minigrep::{Config, highlight, search};

fn main() {
    let mut config = Config::parse();
    if env::var("IGNORE_CASE").is_ok() {
        config.ignore_case = true;
    }

    let pattern = config.pattern;
    let ignore_case = config.ignore_case;

    let handles: Vec<_> = config.file_paths.into_iter().map(|file_path| {
        let pattern = pattern.clone();

        thread::spawn(move || {
            let contents = fs::read_to_string(&file_path).unwrap_or_else(|err| {
                eprintln!("Error reading {file_path}: {err}");
                process::exit(1);
            });

            let results: Vec<(usize, String)> = search(&pattern, &contents, ignore_case)
                .into_iter()
                .map(|(n, line)| (n, highlight(line, &pattern, ignore_case)))
                .collect();

            (file_path, results)
        })
    }).collect();

    for handle in handles {
        let (file_path, results) = handle.join().unwrap();

        for (line_number, line) in results {
            println!("{file_path}:{line_number}: {line}");
        }
    }
}
