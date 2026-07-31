fn replace_negatives(array: &mut [i32]) -> (){
    for number in array.iter_mut(){
        if *number < 0 {
            *number = 0;
        }
    }

}

fn main() {
    let mut numbers = [-4, 8, -2, 0, 5];
    replace_negatives(&mut numbers);
    println!("{numbers:?}");
}
