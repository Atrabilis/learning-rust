pub fn selection_sort(numbers: &mut [i32]){
    for current in 0..(numbers.len()){
        let mut smallest = current;
        for candidate in (current+1)..=(numbers.len()-1){
            if numbers[candidate] < numbers[smallest]{
                smallest = candidate;
            }
        }
    if smallest != current{
        numbers.swap(current,smallest);
    }
    }
}

#[cfg(test)]
mod tests {
    use super::selection_sort;

    #[test]
    fn sorts_unsorted_slice() {
        let mut numbers = [64, 25, 12, 22, 11];

        selection_sort(&mut numbers);

        assert_eq!(numbers, [11, 12, 22, 25, 64]);
    }

    #[test]
    fn keeps_ascending_slice_sorted() {
        let mut numbers = [1, 2, 3, 4, 5];

        selection_sort(&mut numbers);

        assert_eq!(numbers, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn sorts_descending_slice() {
        let mut numbers = [5, 4, 3, 2, 1];

        selection_sort(&mut numbers);

        assert_eq!(numbers, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn handles_duplicate_values() {
        let mut numbers = [4, 2, 4, 1, 2];

        selection_sort(&mut numbers);

        assert_eq!(numbers, [1, 2, 2, 4, 4]);
    }

    #[test]
    fn handles_negative_numbers() {
        let mut numbers = [-5, -1, -10, 0, -3];

        selection_sort(&mut numbers);

        assert_eq!(numbers, [-10, -5, -3, -1, 0]);
    }

    #[test]
    fn handles_empty_slice() {
        let mut numbers: [i32; 0] = [];

        selection_sort(&mut numbers);

        assert_eq!(numbers, []);
    }

    #[test]
    fn handles_single_element() {
        let mut numbers = [42];

        selection_sort(&mut numbers);

        assert_eq!(numbers, [42]);
    }

    #[test]
    fn handles_two_reversed_elements() {
        let mut numbers = [2, 1];

        selection_sort(&mut numbers);

        assert_eq!(numbers, [1, 2]);
    }

    #[test]
    fn handles_all_equal_values() {
        let mut numbers = [7, 7, 7, 7];

        selection_sort(&mut numbers);

        assert_eq!(numbers, [7, 7, 7, 7]);
    }

    #[test]
    fn handles_i32_limits() {
        let mut numbers = [0, i32::MAX, -1, i32::MIN];

        selection_sort(&mut numbers);

        assert_eq!(numbers, [i32::MIN, -1, 0, i32::MAX]);
    }

    #[test]
    fn handles_smallest_value_at_end() {
        let mut numbers = [10, 20, 30, 1];

        selection_sort(&mut numbers);

        assert_eq!(numbers, [1, 10, 20, 30]);
    }
}