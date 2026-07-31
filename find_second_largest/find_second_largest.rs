fn find_second_largest(numbers:&[i32])->i32{
    let mut largest = numbers[0];
    let mut second_largest = numbers[1];
    for number in numbers{
        if *number >= largest{
            second_largest = largest;
            largest = *number;
        }
    }
    second_largest
}

fn main(){
    const NUMBERS_ARRAY:[i32;5] = [-1, 0, 10, 100, -500];
    let second_largest = find_second_largest(&NUMBERS_ARRAY);
    println!("second largest: {0}", second_largest);
}