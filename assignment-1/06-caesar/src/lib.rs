pub const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

pub fn caesar(input: &str, shift: i32) -> String {
    let mut result = String::new();
    let alphabet_length = ALPHABET.len() as i32;

    let adjusted_shift = ((shift % alphabet_length) + alphabet_length) % alphabet_length ;

    for c in input.chars(){
        if ALPHABET.contains(c.to_ascii_lowercase()) {

            let is_uppercase = c.is_ascii_uppercase(); 
            let base_letter = (if is_uppercase { 'A' } else { 'a' } ) as u8;

            let shifting_value = (c as u8 - base_letter) as i32 + adjusted_shift;
            let shifted_char = base_letter + ( shifting_value % alphabet_length) as u8;
            result.push(shifted_char as char);

        } else {
            result.push(c);
        }

    }
    
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world_shift_three() {
        assert_eq!(caesar("Hello, World!", 3), "Khoor, Zruog!");
    }

    #[test]
    fn shift_minus_one() {
        assert_eq!(caesar("abc", -1), "zab");
    }

    #[test]
    fn shift_twenty_seven_wraps_to_one() {
        assert_eq!(caesar("xyz", 27), "yza");
    }

    #[test]
    fn empty_input() {
        assert_eq!(caesar("", 5), "");
    }

    #[test]
    fn shift_zero_is_identity() {
        assert_eq!(caesar("Rust!", 0), "Rust!");
    }

    #[test]
    fn shift_twenty_six_is_identity() {
        assert_eq!(caesar("abc", 26), "abc");
    }

    #[test]
    fn non_letters_preserved() {
        assert_eq!(caesar("1 2 3 !", 5), "1 2 3 !");
    }

    #[test]
    fn large_negative_shift_wraps() {
        assert_eq!(caesar("abc", -27), "zab");
    }
}
