fn reverse_slice(collection:&[i32])-> Vec<i32>{
    let mut resulting_array:Vec<i32> = vec![];
    let len = collection.len();
    let mut right_idx = len;
    while right_idx > 0{
        right_idx -=1;
        resulting_array.push(collection[right_idx]);
    }
    return resulting_array;      
}

fn main(){
    let mut numbers = [1, 2, 3, 4, 5];
    let rev_sort = reverse_slice(&mut numbers);
    println!("{:?}", rev_sort);
}

