pub fn bubble_sort(numbers: &mut [i32]) {
    if numbers.len() < 2 {
        return;
    }
    let mut swapped = true;
    while swapped {
        swapped = false;
        for current in 0..=(numbers.len() - 2) {
            if numbers[current] > numbers[current + 1] {
                numbers.swap(current, current + 1);
                swapped = true
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::bubble_sort;

    #[test]
    fn sorts_unsorted_slice() {
        let mut numbers = [5, 2, 4, 6, 1, 3];

        bubble_sort(&mut numbers);

        assert_eq!(numbers, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn keeps_ascending_slice_sorted() {
        let mut numbers = [1, 2, 3, 4, 5];

        bubble_sort(&mut numbers);

        assert_eq!(numbers, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn sorts_descending_slice() {
        let mut numbers = [5, 4, 3, 2, 1];

        bubble_sort(&mut numbers);

        assert_eq!(numbers, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn handles_duplicate_values() {
        let mut numbers = [4, 2, 4, 1, 2];

        bubble_sort(&mut numbers);

        assert_eq!(numbers, [1, 2, 2, 4, 4]);
    }

    #[test]
    fn handles_negative_numbers() {
        let mut numbers = [-5, -1, -10, 0, -3];

        bubble_sort(&mut numbers);

        assert_eq!(numbers, [-10, -5, -3, -1, 0]);
    }

    #[test]
    fn handles_empty_slice() {
        let mut numbers: [i32; 0] = [];

        bubble_sort(&mut numbers);

        assert_eq!(numbers, []);
    }

    #[test]
    fn handles_single_element() {
        let mut numbers = [42];

        bubble_sort(&mut numbers);

        assert_eq!(numbers, [42]);
    }

    #[test]
    fn handles_two_reversed_elements() {
        let mut numbers = [2, 1];

        bubble_sort(&mut numbers);

        assert_eq!(numbers, [1, 2]);
    }

    #[test]
    fn handles_all_equal_values() {
        let mut numbers = [7, 7, 7, 7];

        bubble_sort(&mut numbers);

        assert_eq!(numbers, [7, 7, 7, 7]);
    }

    #[test]
    fn handles_i32_limits() {
        let mut numbers = [0, i32::MAX, -1, i32::MIN];

        bubble_sort(&mut numbers);

        assert_eq!(numbers, [i32::MIN, -1, 0, i32::MAX]);
    }

    #[test]
    fn moves_largest_value_from_beginning_to_end() {
        let mut numbers = [100, 10, 20, 30];

        bubble_sort(&mut numbers);

        assert_eq!(numbers, [10, 20, 30, 100]);
    }
}
