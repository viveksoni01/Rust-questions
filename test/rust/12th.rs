// Find the maximum subarray sum in Rust



use std::io;

fn main() {
    println!("Enter the array elements separated by spaces:");

    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    
    let array: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse input"))
        .collect();

    
    let mut max_ending = 0;
    let mut max = std::i32::MIN;

    for &num in &array {
        max_ending = num.max(max_ending + num);
        max = max.max(max_ending);
    }

    println!("Maximum subarray sum: {}", max);
}
