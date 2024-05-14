// implement a function that checks wheather a given string is a palindrome or not


fn pal(s: &str) -> bool {
    let s = s.trim().to_lowercase(); // Convert to lowercase and trim whitespace
    let reverse = s.chars().rev().collect::<String>(); // Reverse the string
    s == reverse // Check if the original string is equal to the reversed string
}

fn main() {
    let string1 = "racecar";
    let string2 = "hello";
    
    println!("Is '{}' a palindrome? {}", string1, pal(string1));
    println!("Is '{}' a palindrome? {}", string2, pal(string2));
}


    