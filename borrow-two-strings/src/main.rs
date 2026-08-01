fn total_length(first: &str, second: &str) -> usize {
    first.len() + second.len()
}

fn main() {
    let first = String::from("rust");
    let second = String::from("lang");

    let total = total_length(&first, &second);

    println!("{total}");
    println!("{first}");
    println!("{second}");
}
