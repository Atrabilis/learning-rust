fn find_largest(numbers:&[i32]) -> i32{
    let mut largest:i32 = numbers[0];
    for number in numbers{
        if *number >= largest{
            largest = *number;
        }
    }
    largest
}

fn main(){
    const NUMBERS_ARRAY:[i32;5] = [-1,0,10,100, -500];
    let largest = find_largest(&NUMBERS_ARRAY);
    println!("Largest number: {0}", largest);
}