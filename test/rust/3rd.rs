// Given a string of words, implement a function that returns the shortest word in the string.


use std::io; 
fn short(input: &str) -> Option<&str> {

    let words = input.split();
    
    words.min_by_key(|word| word.len())
}

fn main() {
    println!("Enter a string of words:");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    match short(&input) {
        
        Some(shortest) => println!("Shortest word: {}", shortest.trim()),
        
        None => println!("No words found"),
    }
}
