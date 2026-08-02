pub fn find_second_largest(numbers: &[i32]) -> Option<i32> {
    if numbers.len() < 2 {
        return None;
    }
    let mut largest: i32 = i32::MIN;
    let mut largest_found = false;
    let mut candidate: i32 = i32::MIN;
    let mut candidate_found = false;
    for number in numbers {
        if !largest_found {
            largest = *number;
            largest_found = true;
            continue;
        } else if *number == i32::MIN {
            candidate_found = true;
        } else if *number > largest {
            candidate = largest;
            largest = *number;
            candidate_found = true;
        } else if *number > candidate && *number < largest {
            candidate = *number;
            candidate_found = true
        }
    }
    if candidate_found && candidate != largest{
        return Some(candidate);
    } else {
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::find_second_largest;

    #[test]
    fn finds_second_largest_in_unsorted_slice() {
        assert_eq!(find_second_largest(&[-1, 0, 10, 100, -500]), Some(10));
    }

    #[test]
    fn finds_second_largest_in_ascending_slice() {
        assert_eq!(find_second_largest(&[1, 2, 3, 4]), Some(3));
    }

    #[test]
    fn finds_second_largest_in_descending_slice() {
        assert_eq!(find_second_largest(&[100, 50, 20, 10]), Some(50));
    }

    #[test]
    fn handles_largest_value_at_the_beginning() {
        assert_eq!(find_second_largest(&[100, 10, 50, 20]), Some(50));
    }

    #[test]
    fn handles_negative_numbers() {
        assert_eq!(find_second_largest(&[-50, -10, -30, -20]), Some(-20));
    }

    #[test]
    fn ignores_duplicate_largest_values() {
        assert_eq!(find_second_largest(&[10, 10, 5]), Some(5));
    }

    #[test]
    fn ignores_duplicate_second_largest_values() {
        assert_eq!(find_second_largest(&[10, 5, 5]), Some(5));
    }

    #[test]
    fn handles_duplicates_in_arbitrary_positions() {
        assert_eq!(find_second_largest(&[5, 10, 5, 20, 20, 10]), Some(10));
    }

    #[test]
    fn handles_two_distinct_elements() {
        assert_eq!(find_second_largest(&[5, 10]), Some(5));
    }

    #[test]
    fn returns_none_when_all_values_are_equal() {
        assert_eq!(find_second_largest(&[10, 10, 10]), None);
    }

    #[test]
    fn returns_none_for_one_element() {
        assert_eq!(find_second_largest(&[10]), None);
    }

    #[test]
    fn returns_none_for_empty_slice() {
        assert_eq!(find_second_largest(&[]), None);
    }
    #[test]
    fn handles_i32_min_as_second_largest() {
        assert_eq!(find_second_largest(&[0, i32::MIN]), Some(i32::MIN));
    }
#[test]
fn handles_i32_min_when_it_appears_first() {
    assert_eq!(
        find_second_largest(&[i32::MIN, 0]),
        Some(i32::MIN)
    );
}

#[test]
fn handles_both_integer_limits() {
    assert_eq!(
        find_second_largest(&[i32::MAX, i32::MIN]),
        Some(i32::MIN)
    );
}

#[test]
fn returns_none_when_all_values_are_i32_min() {
    assert_eq!(
        find_second_largest(&[i32::MIN, i32::MIN]),
        None
    );
}

#[test]
fn ignores_duplicate_i32_max_values() {
    assert_eq!(
        find_second_largest(&[i32::MAX, i32::MAX, 10]),
        Some(10)
    );
}
}
