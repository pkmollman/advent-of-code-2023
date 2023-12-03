use std::fs;
use regex::Regex;
use once_cell::sync::Lazy;

static SYMBOL: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^0-9\.]").unwrap());

fn file_path_to_lines(file_path: String) -> Vec<String> {
    let mut lines: Vec<String> = vec![];
    for line in  fs::read_to_string(file_path).unwrap().lines() {
        lines.push(line.to_string())
    }
    return lines;
}

