pub fn find_largest(numbers: &[i32]) -> Option<i32> {
    if numbers.len() == 0 {
        return None;
    }
    let mut largest: i32 = numbers[0];
    for number in numbers {
        if *number >= largest {
            largest = *number;
        }
    }
    Some(largest)
}

#[cfg(test)]
mod tests {
    use super::find_largest;

    #[test]
    fn finds_largest_in_unsorted_slice() {
        assert_eq!(find_largest(&[-1, 0, 10, 100, -500]), Some(100));
    }

    #[test]
    fn finds_largest_in_ascending_slice() {
        assert_eq!(find_largest(&[1, 2, 3, 4]), Some(4));
    }

    #[test]
    fn finds_largest_in_descending_slice() {
        assert_eq!(find_largest(&[100, 50, 20, 10]), Some(100));
    }

    #[test]
    fn handles_negative_numbers() {
        assert_eq!(find_largest(&[-50, -10, -30, -20]), Some(-10));
    }

    #[test]
    fn handles_single_element() {
        assert_eq!(find_largest(&[42]), Some(42));
    }

    #[test]
    fn handles_duplicate_largest_values() {
        assert_eq!(find_largest(&[10, 50, 50, 20]), Some(50));
    }

    #[test]
    fn handles_all_equal_values() {
        assert_eq!(find_largest(&[7, 7, 7]), Some(7));
    }

    #[test]
    fn handles_i32_min() {
        assert_eq!(find_largest(&[i32::MIN]), Some(i32::MIN));
    }

    #[test]
    fn handles_i32_max() {
        assert_eq!(find_largest(&[0, i32::MAX, -10]), Some(i32::MAX));
    }

    #[test]
    fn handles_both_integer_limits() {
        assert_eq!(find_largest(&[i32::MIN, 0, i32::MAX]), Some(i32::MAX));
    }

    #[test]
    fn returns_none_for_empty_slice() {
        assert_eq!(find_largest(&[]), None);
    }
}
