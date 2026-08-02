fn is_even(num:i32)-> bool{
    num % 2 == 0
}

fn main() {
    let test_cases:[i32;4] = [1,2,3,4];
    for number in test_cases{
        println!("{} is even: {}", number, is_even(number));
    }
}