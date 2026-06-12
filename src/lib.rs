use clap::Parser;
use colored::Colorize;

#[derive(Parser)]
#[command(name = "minigrep", about = "Search for a pattern in a file")]
pub struct Config {
    /// Pattern to search for
    pub pattern: String,
    /// File to search in
    pub file_path: String,
    /// Case-insensitive search (also enabled by IGNORE_CASE env var)
    #[arg(short = 'i', long)]
    pub ignore_case: bool,
}

pub fn search<'a>(pattern: &str, contents: &'a str, ignore_case: bool) -> Vec<(usize, &'a str)> {
    let pattern = if ignore_case { pattern.to_lowercase() } else { pattern.to_string() };

    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| {
            let line_check = if ignore_case { line.to_lowercase() } else { line.to_string() };
            line_check.contains(&pattern)
        })
        .map(|(i, line)| (i + 1, line))
        .collect()
}

pub fn highlight(line: &str, pattern: &str, ignore_case: bool) -> String {
    if !ignore_case {
        return line.replace(pattern, &pattern.red().to_string());
    }

    let lower_line = line.to_lowercase();
    let lower_pattern = pattern.to_lowercase();

    let mut result = String::new();
    let mut last_end = 0;
    let mut start = 0;

    while let Some(pos) = lower_line[start..].find(&lower_pattern) {
        let match_start = start + pos;
        let match_end = match_start + lower_pattern.len();

        result.push_str(&line[last_end..match_start]);
        result.push_str(&line[match_start..match_end].red().to_string());

        last_end = match_end;
        start = match_end;
    }

    result.push_str(&line[last_end..]);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive_finds_match() {
        let contents = "\
Rust:
safe, fast, productive.
Pick three, duct tape.";

        assert_eq!(vec![(1, "Rust:")], search("Rust", contents, false));
    }

    #[test]
    fn case_sensitive_no_match() {
        let contents = "\
Rust:
safe, fast, productive.";

        assert_eq!(Vec::<(usize, &str)>::new(), search("rust", contents, false));
    }

    #[test]
    fn case_insensitive_finds_matches() {
        let contents = "\
Rust:
Trust me.
DUCT TAPE.";

        assert_eq!(
            vec![(1, "Rust:"), (2, "Trust me.")],
            search("rUsT", contents, true)
        );
    }

    #[test]
    fn highlight_wraps_match_in_red() {
        colored::control::set_override(true);
        let result = highlight("hello world", "world", false);
        assert_eq!(result, format!("hello {}", "world".red()));
    }

    #[test]
    fn highlight_preserves_original_case() {
        colored::control::set_override(true);
        let result = highlight("Hello WORLD", "world", true);
        assert_eq!(result, format!("Hello {}", "WORLD".red()));
    }
}
