// Implement a function that returns the kth smallest element in a given array


use std::io;


fn smallest(arr: &[i32], k: usize) -> Option<i32> {
    
    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort()
    sorted_arr.get(k - 1).cloned()
}

fn main() {
    println!("Please enter the numbers of your array separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let arr: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();

    println!("Enter the value of k (position of the smallest element you want to find):");
    let mut k_input = String::new();
    io::stdin().read_line(&mut k_input).expect("Failed to read line");

    let k: usize = k_input.trim().parse().expect("Invalid number");

    if let Some(val) = smallest(&arr, k) {
        println!("The {}th smallest element in the array is: {}", k, val);
    } else {
        println!("Invalid value of k. Please enter a valid position.");
    }
}

