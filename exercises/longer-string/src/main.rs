fn longer(first:String, second:String) -> String{
    if second.len() > first.len(){
        return second;
    }    
    first
}


fn main() {
    let first = String::from("rust");
    let second = String::from("ownership");
    let result = longer(first, second);
    println!("{result}");
}
