pub fn reverse_words(sentence: &str) -> String {
    let words = sentence.split_whitespace();
    let mut reversed_string = String::from("");

    for st in words.rev(){
        if reversed_string.len() == 0{
            reversed_string.push_str(st);
        }
        else{
            // reversed_string = st.to_string() + " " + &reversed_string;
            reversed_string = reversed_string+ " " + st;
        }
        // println!("{} {}", st,reversed_string);
    }

    reversed_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_words() {
        assert_eq!(reverse_words("hello world rust"), "rust world hello");
    }

    #[test]
    fn collapses_inner_whitespace() {
        assert_eq!(reverse_words("   one   two  "), "two one");
    }

    #[test]
    fn empty_input() {
        assert_eq!(reverse_words(""), "");
    }

    #[test]
    fn single_word() {
        assert_eq!(reverse_words("single"), "single");
    }

    #[test]
    fn whitespace_only() {
        assert_eq!(reverse_words("    "), "");
    }

    #[test]
    fn many_short_words() {
        assert_eq!(reverse_words("a b c d e"), "e d c b a");
    }

    #[test]
    fn tabs_and_newlines_count_as_whitespace() {
        assert_eq!(reverse_words("a\tb\nc"), "c b a");
    }

    #[test]
    fn leading_and_trailing_trim() {
        assert_eq!(
            reverse_words("  leading and trailing  "),
            "trailing and leading"
        );
    }
}
