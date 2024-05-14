// Given a sorted array of integers, implement a function that returns the median of the array



use std::io;

fn findmed(sorted_array: &[i32]) -> f64 {
    let len = sorted_array.len();

    if len % 2 == 1 {
        sorted_array[len / 2] as f64
    } else {
        let mid = len / 2;
        let median = (sorted_array[mid - 1] + sorted_array[mid]) as f64 / 2.0;
        median
    }
}

fn main() {
    println!("Enter a sorted array of integers separated by spaces:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let sorted_array: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    let median = findmed(&sorted_array);

    println!("The median of the array is: {}", median);
}
