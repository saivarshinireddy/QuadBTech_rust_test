use std::io;
fn is_prime(number:u64)->bool{
    if number<=1{
        return false;
    }
    for i in 2..=((number as f64).sqrt() as u64){
        if number%i==0 {
            return false;
        }
    }
    return true;
}
fn main(){
    println!("Enter the number:");
    let mut input=String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let number=input.trim().parse().expect("Invalid number");
    if is_prime(number){
        println!("{} is a prime number",number);
    }
    else{
        println!("{} is not a prime number",number);
    }
}