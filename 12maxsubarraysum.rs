use std::io;

fn max_subarray_sum(arr: &[i32]) -> i32 {
    let mut max_ending_here = 0;
    let mut max_so_far = std::i32::MIN;

    for &num in arr {
        max_ending_here = i32::max(num, max_ending_here + num);
        max_so_far = i32::max(max_so_far, max_ending_here);
    }

    max_so_far
}

fn main() {
    // Read input from the user
    println!("Enter array elements separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    // Parse input string into a vector of integers
    let arr: Vec<i32> = input.trim().split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();

    // Find the maximum subarray sum using Kadane's algorithm
    let max_sum = max_subarray_sum(&arr);
    println!("Maximum subarray sum: {}", max_sum);
}
