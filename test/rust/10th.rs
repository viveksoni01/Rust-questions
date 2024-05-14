// Check if a number is prime in Rust


use std::io;

fn main() {
    println!("Enter a number to check if it's prime:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: u32 = input.trim().parse().expect("Please enter a number");

    let is_prime = (2..input).all(|i| input % i != 0);

    println!("{} is {} prime.", input, if is_prime { "a" } else { "not a" });
}
