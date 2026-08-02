pub fn first_word(word: &str) -> String {
    if word.starts_with(" ") {
        return "".to_string();
    }
    let trimmed_text = word.trim();
    for (idx, chr) in trimmed_text.char_indices() {
        if chr == ' ' {
            return trimmed_text[..idx].to_string();
        }
    }
    word.to_string()
}

#[cfg(test)]
mod tests {
    use super::first_word;
    #[test]
    fn returns_first_word_from_two_words() {
        assert_eq!(first_word("rust ownership"), "rust");
    }

    #[test]
    fn returns_entire_text_when_there_is_no_space() {
        assert_eq!(first_word("hello"), "hello");
    }

    #[test]
    fn returns_empty_slice_for_empty_text() {
        assert_eq!(first_word(""), "");
    }

    #[test]
    fn returns_empty_slice_when_text_starts_with_space() {
        assert_eq!(first_word(" hello"), "");
    }

    #[test]
    fn handles_multiple_words() {
        assert_eq!(first_word("one two three"), "one");
    }

    #[test]
    fn handles_unicode_before_the_space() {
        assert_eq!(first_word("año nuevo"), "año");
    }

    #[test]
    fn returns_entire_text_when_unicode_has_no_space() {
        assert_eq!(first_word("español"), "español");
    }
}
