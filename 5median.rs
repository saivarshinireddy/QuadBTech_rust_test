use std::io;
fn median(arr:&[i32])->f64{
    let len=arr.len();
    if len%2==0{
        return (arr[len/2]+arr[(len/2)-1]) as f64/2.0;
    }
    else{
        return arr[len/2] as f64;
    }
}
fn main(){
    println!("Enter the elements of array seperated by space:");
    let mut input=String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let arr:Vec<i32>=input.trim().split_whitespace().map(|x| x.parse().expect("Invalid input")).collect();
    println!("Median of the Array is {}",median(&arr));
}