fn add_suffix(text:String) -> String{
    let mut updated_string = text;
    updated_string.push_str("!");
    updated_string
}

fn main() {
    let message = String::from("hello");
    let updated = add_suffix(message);
    println!("{updated}");
}