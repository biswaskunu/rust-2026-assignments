pub fn censor_vowels(s: &mut String) {
    let mut result = String::new();
    for c in s.chars(){

        let st = match c {
            'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => '*',
            _ => c
        };
        result.push(st);
    }
    *s = result;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(input: &str) -> String {
        let mut s = String::from(input);
        censor_vowels(&mut s);
        s
    }

    #[test]
    fn example_hello_world() {
        assert_eq!(run("Hello, World!"), "H*ll*, W*rld!");
    }

    #[test]
    fn all_uppercase_vowels() {
        assert_eq!(run("AEIOU"), "*****");
    }

    #[test]
    fn empty_input() {
        assert_eq!(run(""), "");
    }

    #[test]
    fn no_vowels() {
        assert_eq!(run("bcdfg"), "bcdfg");
    }

    #[test]
    fn all_lowercase_vowels() {
        assert_eq!(run("aeiou"), "*****");
    }

    #[test]
    fn mixed_case() {
        assert_eq!(run("AaEeIi"), "******");
    }

    #[test]
    fn digits_and_letters() {
        assert_eq!(run("h3ll0 wOrld"), "h3ll0 w*rld");
    }

    #[test]
    fn punctuation_only() {
        assert_eq!(run("!@#$%"), "!@#$%");
    }
}
