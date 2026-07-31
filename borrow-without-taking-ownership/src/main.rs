fn count_characters(message:&String) -> usize{
    let mut cnt = 0;
    for _char in message.chars(){
        cnt +=1;
    }
    cnt
}

fn main() {
    let message = String::from("rust");
    let count = count_characters(&message);
    println!("{count}");
    println!("{message}");
}
