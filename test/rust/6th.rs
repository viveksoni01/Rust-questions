// Implement a function that finds the longest common prefix of a given set of strings


use std::io::{self, BufRead};

fn common_prefix(mut strings: Vec<String>) -> String {
    if strings.is_empty() {
        return String::new();
    }

    strings.sort();

    let first = strings[0].as_bytes();
    let last = strings.last().unwrap().as_bytes();

    let mut prefix = String::new();

    for (a, b) in first.iter().zip(last.iter()) {
        if a != b {
            break;
        }
        prefix.push(*a as char);
    }
   prefix
}
fn main() {
    let stdin = io::stdin();
    let strings: Vec<String> = stdin.lock().lines().filter_map(|line| line.ok()).collect();

    let common_prefix = common_prefix(strings);
    println!("Longest common prefix: {}", common_prefix);
}
