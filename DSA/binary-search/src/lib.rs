pub fn binary_search(numbers: &[i32], target: i32) -> Option<usize> {
    if numbers.is_empty() {
        return None;
    }
    let mut left = 0;
    let mut right = numbers.len();
    while left < right {
        let middle = left + (right - left) / 2;
        if numbers[middle] == target {
            return Some(middle);
        }

        if numbers[middle] < target {
            left = middle + 1;
        } else {
            right = middle
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::binary_search;

    #[test]
    fn finds_value_at_beginning() {
        assert_eq!(binary_search(&[2, 4, 7, 10, 15], 2), Some(0));
    }

    #[test]
    fn finds_value_in_middle() {
        assert_eq!(binary_search(&[2, 4, 7, 10, 15], 7), Some(2));
    }

    #[test]
    fn finds_value_at_end() {
        assert_eq!(binary_search(&[2, 4, 7, 10, 15], 15), Some(4));
    }

    #[test]
    fn returns_none_when_target_is_smaller_than_all_values() {
        assert_eq!(binary_search(&[2, 4, 7, 10, 15], 1), None);
    }

    #[test]
    fn returns_none_when_target_is_larger_than_all_values() {
        assert_eq!(binary_search(&[2, 4, 7, 10, 15], 20), None);
    }

    #[test]
    fn returns_none_when_target_is_between_values() {
        assert_eq!(binary_search(&[2, 4, 7, 10, 15], 8), None);
    }

    #[test]
    fn returns_none_for_empty_slice() {
        assert_eq!(binary_search(&[], 10), None);
    }

    #[test]
    fn finds_value_in_single_element_slice() {
        assert_eq!(binary_search(&[10], 10), Some(0));
    }

    #[test]
    fn returns_none_for_single_element_slice_without_target() {
        assert_eq!(binary_search(&[10], 5), None);
    }

    #[test]
    fn handles_even_number_of_elements() {
        assert_eq!(binary_search(&[1, 3, 5, 7, 9, 11], 7), Some(3));
    }

    #[test]
    fn handles_negative_numbers() {
        assert_eq!(binary_search(&[-50, -20, -10, 0, 15], -10), Some(2));
    }

    #[test]
    fn handles_i32_limits() {
        let numbers = [i32::MIN, -1, 0, 1, i32::MAX];

        assert_eq!(binary_search(&numbers, i32::MIN), Some(0));
        assert_eq!(binary_search(&numbers, i32::MAX), Some(4));
    }
}
