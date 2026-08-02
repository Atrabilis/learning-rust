fn last_word(text: &str) -> &str{
    let trimmed = text.trim();
    let mut left: usize = 0;
    for (idx, chr) in trimmed.bytes().enumerate(){
        if chr == b' '{
            left = idx + 1;
        }
    }
    &text[left..]
}

fn main() {
    println!("{}", last_word("rust ownership")); // "ownership"
    println!("{}", last_word("hello"));          // "hello"
    println!("{}", last_word("one two three"));  // "three"
}
