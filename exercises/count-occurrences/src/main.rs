fn count_occurrences(slice:&[i32], target:i32) -> usize {
    let mut occurrences:usize = 0;
    for number in slice{
        if *number == target{
            occurrences += 1;
        }
    }
    occurrences
}

fn main(){
    println!{"{}",count_occurrences(&[4, 8, 2, 8, 8], 8)}; // 3
    println!{"{}",count_occurrences(&[4, 8, 2, 8], 5)};
}