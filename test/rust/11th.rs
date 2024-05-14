// .Merge two sorted arrays in Rust


use std::io;

fn main() {
    fn input_array() -> Vec<i32> {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.split_whitespace().map(|x| x.parse().unwrap()).collect()
    }

    let mut array1 = input_array();
    let mut array2 = input_array();

    array1.extend(array2);
    array1.sort();

    println!("Merged and sorted array: {:?}", array1);
}
