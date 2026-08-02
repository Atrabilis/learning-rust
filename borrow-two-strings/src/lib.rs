pub fn total_length(first: &str, second: &str) -> usize {
    first.len() + second.len()
}

#[cfg(test)]
mod tests {
    use super::total_length;

    #[test]
    fn adds_length_of_two_ascii_strings() {
        assert_eq!(total_length("rust", "lang"), 8);
    } 

    #[test]
    fn handles_empty_first_string() {
        assert_eq!(total_length("", "rust"), 4);
    }

    #[test]
    fn handles_empty_second_string() {
        assert_eq!(total_length("rust", ""), 4);
    }

    #[test]
    fn handles_two_empty_strings() {
        assert_eq!(total_length("", ""), 0);
    }

    #[test]
    fn counts_utf8_bytes() {
        assert_eq!(total_length("año", "ñ"), 6);
    }

    #[test]
    fn handles_whitespace() {
        assert_eq!(total_length("hello world", " "), 12);
    }
}
