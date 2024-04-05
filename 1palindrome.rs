use std::io;
fn palindrome(s:&str) ->bool{
    let rev_string:String=s.chars().rev().collect();//reversing the string
    s==rev_string //comparing reversed string with actual string
}
fn main(){
    println!("Enter a string:");
    let mut string_inp=String::new();
    io::stdin().read_line(&mut string_inp).expect("read a line");
    let string_inp=string_inp.trim();
    if palindrome(&string_inp) {
        println!("{} is a palindrome",string_inp);
    }
    else{
        println!("{} is not a palindrome",string_inp);
    }
}