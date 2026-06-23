# Solution notes: Caesar cipher

## Approach

The goal is to implement a Caesar cipher that shifts alphabetic characters by a given integer offset while preserving case and ignoring non-alphabetic characters.

1. **Shift Normalization (The Modulo Trick):** The shift value can be any integer, including large numbers or negative values. To keep the shift within the bounds of a 26-letter alphabet, we calculate an `adjusted_shift` using a double-modulo formula: 
   $$((shift \pmod{26}) + 26) \pmod{26}$$
   This converts negative shifts into their positive equivalents (e.g., a shift of `-1` becomes `25`).
2. **Character Filtering:** We iterate through each character `c` of the input string. Using `ALPHABET.contains(c.to_ascii_lowercase())`, the function checks if the character is an English letter.
3. **Preserving Casing with Base Offsets:** If it is a letter, we determine if it's uppercase. We establish a `base_letter` (`'A'` or `'a'`) converted to its ASCII byte representation (`u8`).
4. **Byte Arithmetic:** * We calculate the letter's 0-indexed alphabet position: `c as u8 - base_letter`.
   * We add the `adjusted_shift`.
   * We apply modulo 26 (`% alphabet_length`) to wrap around if it goes past 'Z' or 'z'.
   * We add the value back to the `base_letter` to get the final shifted ASCII value and push it onto the `result` string.
5. **Pass-through for Non-alphabetic Characters:** Any spaces, punctuation, or numbers skip the math block entirely and are pushed directly to the output without alteration.

## Edge cases handled

* **Negative Shifts:** Handled cleanly by the math formula. For instance, shifting `'a'` by `-1` correctly yields `'z'`.
* **Large Shifts ($> 26$ or $< -26$):** The modulo operations compress large values (like a shift of `1000`) into a valid index between `0` and `25`.
* **Casing Integrity:** Inputting `"Hello"` results in `"Khoor"` (with a shift of 3)—the capitalized 'H' stays capitalized, and lowercase letters stay lowercase.
* **Non-ASCII/Special Characters:** Spaces, numbers, symbols, and emojis pass through unchanged because they fail the `ALPHABET.contains` check.

## Anything special

### The Negative Modulo Trick
* In Rust (and many other languages like C++), the `%` operator calculates the *remainder*, not the mathematical *modulo*. This means a negative number yields a negative remainder (e.g., `-1 % 26 == -1`). 
* By adding the length of the alphabet and applying the remainder operator a second time—`((shift % 26) + 26) % 26`—the code safely forces a positive wrap-around, entirely eliminating the risk of subtraction underflows or out-of-bounds character casting.

### Potential Optimization Note
* **`ALPHABET.contains()` Check:** Calling `.contains()` runs a sub-string lookup behind the scenes. Since this is an ASCII-only Caesar cipher, checking boundaries directly with `.is_ascii_alphabetic()` or evaluating `c.is_ascii_lowercase() || c.is_ascii_uppercase()` would perform faster by eliminating the need to look up data inside a global string slice constant.