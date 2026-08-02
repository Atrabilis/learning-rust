pub fn linear_search(numbers: &[i32], target: i32) -> Option<usize>{
    if numbers.is_empty(){
        return None
    }
    for (idx, &number) in numbers.iter().enumerate(){
        if number == target{
            return Some(idx)
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::linear_search;

    #[test]
    fn finds_value_at_beginning() {
        assert_eq!(linear_search(&[10, 20, 30], 10), Some(0));
    }

    #[test]
    fn finds_value_in_middle() {
        assert_eq!(linear_search(&[10, 20, 30], 20), Some(1));
    }

    #[test]
    fn finds_value_at_end() {
        assert_eq!(linear_search(&[10, 20, 30], 30), Some(2));
    }

    #[test]
    fn returns_first_index_when_value_is_repeated() {
        assert_eq!(linear_search(&[5, 10, 5, 20], 5), Some(0));
    }

    #[test]
    fn returns_none_when_value_does_not_exist() {
        assert_eq!(linear_search(&[10, 20, 30], 100), None);
    }

    #[test]
    fn returns_none_for_empty_slice() {
        assert_eq!(linear_search(&[], 10), None);
    }

    #[test]
    fn handles_negative_numbers() {
        assert_eq!(linear_search(&[-10, -20, -30], -20), Some(1));
    }

    #[test]
    fn handles_i32_min() {
        assert_eq!(
            linear_search(&[0, i32::MIN, 10], i32::MIN),
            Some(1)
        );
    }

    #[test]
    fn handles_i32_max() {
        assert_eq!(
            linear_search(&[0, 10, i32::MAX], i32::MAX),
            Some(2)
        );
    }
}