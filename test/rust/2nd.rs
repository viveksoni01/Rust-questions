// Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given numberthe first occurrence of a given number.


fn occ(sorted_array: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = sorted_array.len() - 1;
    let mut result = None;

    while left <= right {
        let mid = left + (right - left) / 2;
        if sorted_array[mid] == target {
            result = Some(mid);
            right = mid - 1;
        } else if sorted_array[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    result
}

fn main() {
    let arr = vec![1, 2, 3, 3, 3, 4, 5, 6, 6, 7];
    let target = 3;

    match occ(&arr, target) {
        Some(index) => println!("First occurrence of {} is at index {}", target, index),
        None => println!("{} not found in the array", target),
    }
}
