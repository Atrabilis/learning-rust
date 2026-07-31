fn find_first_index(numbers:&[i32], target:i32) -> Option<usize>{
    let mut index:usize= 0;
    for number in numbers{
        if *number == target{
            return Some(index);
        } 
        index +=1;
        
    }
    None
}

fn main(){
    if let Some(index) = find_first_index(&[4, 8, 2, 8], 8){
        println!{"{}",index}
    }
    if let Some(index) = find_first_index(&[4, 8, 2, 8], 5){
        println!{"{}",index};
    } else{
        println!("None");
    }
}