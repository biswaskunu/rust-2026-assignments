pub fn run_length_encode(input: &str) -> Vec<(char, u32)> {
    let mut result = Vec::new();
    let mut chars = input.chars();

    if let Some(mut c ) = chars.next() {
        let mut count = 1;

        for ch in chars {
            if ch == c {
                count += 1;
            } else {
                result.push((c, count));

                count = 1;
                c = ch;
            }
        }
        // Push the last run
        result.push((c, count));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_aaabbc() {
        assert_eq!(
            run_length_encode("aaabbc"),
            vec![('a', 3), ('b', 2), ('c', 1)]
        );
    }

    #[test]
    fn empty_input() {
        assert_eq!(run_length_encode(""), vec![]);
    }

    #[test]
    fn single_char() {
        assert_eq!(run_length_encode("x"), vec![('x', 1)]);
    }

    #[test]
    fn all_same() {
        assert_eq!(run_length_encode("aaaaa"), vec![('a', 5)]);
    }

    #[test]
    fn all_different() {
        assert_eq!(
            run_length_encode("abcd"),
            vec![('a', 1), ('b', 1), ('c', 1), ('d', 1)]
        );
    }

    #[test]
    fn alternating_runs() {
        assert_eq!(
            run_length_encode("aabbaa"),
            vec![('a', 2), ('b', 2), ('a', 2)]
        );
    }

    #[test]
    fn whitespace_counts_too() {
        assert_eq!(
            run_length_encode("aa  bb"),
            vec![('a', 2), (' ', 2), ('b', 2)]
        );
    }
}
