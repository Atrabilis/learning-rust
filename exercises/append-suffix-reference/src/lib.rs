pub fn append_suffix(text: &mut String, suffix: &str) {
    text.push_str(suffix);
}

#[cfg(test)]
mod tests {
    use super::append_suffix;

    #[test]
    fn appends_suffix_to_text() {
        let mut text = String::from("hello");

        append_suffix(&mut text, " world");

        assert_eq!(text, "hello world");
    }

    #[test]
    fn appends_to_empty_string() {
        let mut text = String::new();

        append_suffix(&mut text, "hello");

        assert_eq!(text, "hello");
    }

    #[test]
    fn appending_empty_suffix_does_not_change_text() {
        let mut text = String::from("hello");

        append_suffix(&mut text, "");

        assert_eq!(text, "hello");
    }

    #[test]
    fn handles_unicode_text() {
        let mut text = String::from("año");

        append_suffix(&mut text, " nuevo");

        assert_eq!(text, "año nuevo");
    }

    #[test]
    fn handles_unicode_suffix() {
        let mut text = String::from("hola");

        append_suffix(&mut text, " 👋");

        assert_eq!(text, "hola 👋");
    }

    #[test]
    fn can_append_multiple_times() {
        let mut text = String::from("rust");

        append_suffix(&mut text, " ownership");
        append_suffix(&mut text, " borrowing");

        assert_eq!(text, "rust ownership borrowing");
    }

    #[test]
    fn preserves_whitespace_exactly() {
        let mut text = String::from("hello");

        append_suffix(&mut text, "   world");

        assert_eq!(text, "hello   world");
    }
}
