pub struct Config {
    pub pattern: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments, usage: minigrep <pattern> <file> [--ignore-case]");
        }

        let pattern = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = args.iter().any(|arg| arg == "--ignore-case");

        Ok(Config { pattern, file_path, ignore_case })
    }
}

pub fn search<'a>(pattern: &str, contents: &'a str, ignore_case: bool) -> Vec<(usize, &'a str)> {
    let mut results = Vec::new();

    let search_pattern = if ignore_case {
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

        if line_to_check.contains(&search_pattern) {
            results.push((i + 1, line));
        }
    }

    results
}

pub fn highlight(line: &str, pattern: &str, ignore_case: bool) -> String {
    const RED: &str = "\x1b[31m";
    const RESET: &str = "\x1b[0m";

    if !ignore_case {
        return line.replace(pattern, &format!("{RED}{pattern}{RESET}"));
    }

    let lower_line = line.to_lowercase();
    let lower_pattern = pattern.to_lowercase();

    let mut result = String::new();
    let mut last_end = 0;
    let mut start = 0;

    while let Some(pos) = lower_line[start..].find(&lower_pattern) {
        let match_start = start + pos;
        let match_end = match_start + pattern.len();

        result.push_str(&line[last_end..match_start]);
        result.push_str(RED);
        result.push_str(&line[match_start..match_end]);
        result.push_str(RESET);

        last_end = match_end;
        start = match_end;
    }

    result.push_str(&line[last_end..]);
    result
}

// Util tests

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
        let result = highlight("hello world", "world", false);
        assert_eq!(result, "hello \x1b[31mworld\x1b[0m");
    }

    #[test]
    fn highlight_preserves_original_case() {
        let result = highlight("Hello WORLD", "world", true);
        assert_eq!(result, "Hello \x1b[31mWORLD\x1b[0m");
    }
}
