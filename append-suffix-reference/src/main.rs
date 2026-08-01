fn append_suffix(text: &mut String, suffix: &str) {
    text.push_str(suffix);
}

fn main() {
    let mut message = String::from("hello");

    append_suffix(&mut message, " world");

    println!("{message}");
}
