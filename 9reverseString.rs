use std::io;
fn reverse_string(input:&str)->String{
    let mut result=String::new();
    for c in input.chars().rev(){
        result.push(c);
    }
    return result;
}
fn main(){
    println!("Enter a string:");
    let mut input=String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let reversed=reverse_string(input.trim());
    println!("Reversed String:{}",reversed);
}