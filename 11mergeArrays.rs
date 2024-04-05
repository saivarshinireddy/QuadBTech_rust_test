use std::io;

fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    let (mut i, mut j) = (0, 0);

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] <= arr2[j] {
            result.push(arr1[i]);
            i += 1;
        } else {
            result.push(arr2[j]);
            j += 1;
        }
    }

    while i < arr1.len() {
        result.push(arr1[i]);
        i += 1;
    }

    while j < arr2.len() {
        result.push(arr2[j]);
        j += 1;
    }

    result
}

fn main() {
    // Read input for the first sorted array
    println!("Enter sorted array 1 (space-separated integers):");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    let arr1: Vec<i32> = input.split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();

    // Read input for the second sorted array
    println!("Enter sorted array 2 (space-separated integers):");
    input.clear();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    let arr2: Vec<i32> = input.split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();

    let merged_array = merge_sorted_arrays(&arr1, &arr2);
    println!("Merged array: {:?}", merged_array);
}
