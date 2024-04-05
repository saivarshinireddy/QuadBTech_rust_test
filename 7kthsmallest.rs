use std::io;
fn k_th_smallest(arr:&[i32],k:usize)->i32{
   let  mut arr=arr.to_vec();
    arr.sort();
    if k<arr.len(){
        return arr[k-1];
    }
    else{
        return -1;
    }
}
fn main(){
    println!("Enter the elements of the array seperated by spaces");
    let mut input=String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let  arr:Vec<i32>=input.trim().split_whitespace().map(|x| x.parse().expect("Invalid input")).collect();
    println!("Enter k value:");
    let mut input_k=String::new();
    io::stdin().read_line(&mut input_k).expect("Failed to read line");
    let value:usize=input_k.trim().parse().expect("Invalid number");
    let result=k_th_smallest(&arr,value);
    if result!=-1{
    println!("{}th smallest element in the array is:{}",value,result);
    }
    else {
        println!("No such element");
    }
}