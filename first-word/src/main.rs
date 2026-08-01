fn first_word(word: &str) -> String {
    let mut left: usize = 0;
    let mut right: usize = 0;
    for chr in word.chars() {
        if chr == ' ' && right == left {
            left += 1;
            right += 1;
            continue;
        }
        if chr == ' ' && right != 0 {
            return word[left..right].to_string();
        }
        right += 1;
    }
    word.to_string()
}

fn main() {
    let a = first_word(" rust ownership"); // "rust"
    let b = first_word("hello"); // "hello"
    println!("{}", a);
    println!("{}", b);
}
