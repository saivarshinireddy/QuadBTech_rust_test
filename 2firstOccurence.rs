use std::io;
fn first_occurence(arr:&[i32],value:i32)->usize{
    for(index,&num) in arr.iter().enumerate(){
        if num==value{
             return index;
        }
    }
    return arr.len();
}
fn main(){
    println!("Enter the elements of array seperate with space:");
    let mut inp=String::new();
    io::stdin().read_line(&mut inp).expect("failed to read line");
    let arr:Vec<i32>=inp.trim().split_whitespace().map(|x| x.parse().expect("Invalid input")).collect();
    println!("Enter the target integer:");
    let mut inp=String::new();
    io::stdin().read_line(&mut inp).expect("failed to read line");
    let value:i32=inp.trim().parse().expect("Invalid input");
    let result=first_occurence(&arr,value);
    if result==arr.len(){
        println!("No such occurence");
    }
    else{
        println!("The first occurence of {} is {}(index-based)",value,result);
    }
}