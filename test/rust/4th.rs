// Implement a function that checks whether a given number is prime or not.




fn pr(num: u64) -> bool {
    if num <= 1 {
        return false;
    }
    for i in 2..=num / 2 {
        if num % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    println!("Enter a number to check if it's prime:");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let num_to_check: u64 = input.trim().parse().expect("Invalid input, please enter a valid number.");
    
    if pr(check) {
        println!("{} is prime", check);
    } else {
        println!("{} is not prime", check);
    }
}
