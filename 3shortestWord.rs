use std::io;
fn shortest_word(sentence:&str)-> &str{
    let words:Vec<&str>=sentence.split_whitespace().collect();
    let mut min=words[0];
    for word in words{
        if word.len()<min.len(){
            min=word;
        }
    }
    return min;
}
fn main(){
    println!("Enter the String:");
    let mut input=String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    let sentence=input.trim();
    let result=shortest_word(sentence);
    println!("The shortest word in given string is:{}",result);
}