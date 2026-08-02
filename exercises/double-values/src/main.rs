fn double_values(numbers:&mut[i32]) -> (){
    for number in numbers.iter_mut(){
        *number = *number *2;
    }
}

fn main() {
    let mut numbers = [1, -2, 3, 0];
    double_values(&mut numbers);
    println!("{numbers:?}");
}
