
fn largest_common_prefix(strings:&[&str])->String{
    if strings.is_empty(){
        return String::new();
    }
    let mut prefix=String::new();
    for(i,char_) in strings[0].chars().enumerate(){
        let mut common=true;
        for s in &strings[1..]{
            if let Some(c)=s.chars().nth(i){
                if c!=char_ {
                    common=false;
                    break;
                }
            }
            else{
                common=false;
                break;
            }
        }
        if common{
            prefix.push(char_);
        }else{
            break;
        }
    }
    return prefix;
}
fn main(){
    let strings=vec!["Tomb","Torch","Top"];
    let prefix=largest_common_prefix(&strings);
    println!("Longest common prefix:{}",prefix);
}