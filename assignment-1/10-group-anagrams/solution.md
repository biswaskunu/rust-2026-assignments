# Solution notes: Group anagrams

## Approach

The core idea is to group words that share the same character frequencies. Since anagrams contain the exact same characters, sorting the characters of any word will yield a unique "signature" common to all its anagrams.

1. **Initialize a Hash Map:** We use a `HashMap<String, Vec<String>>` where the key is the sorted character signature, and the value is a list of original words that match that signature.
2. **Generate Signatures:** For each word in the input list, we convert it to lowercase, break it into characters, sort those characters unstable-ly (for better performance), and re-collect them into a standard `String`.
3. **Group by Signature:** We use Rust's efficient Entry API (`map.entry(...).or_default().push(...)`) to either find the existing group or initialize a new vector, then append the original cloned word.
4. **Collect Results:** Finally, we extract and return only the values of the map using `map.into_values().collect()`.

## Edge cases handled

* **Empty Input:** If the input slice is empty, the map remains empty, and the function safely returns an empty vector (`[]`).
* **Single Character/Single Word Lists:** Handled naturally without crashing; lists with a single element just map to themselves.
* **Case Insensitivity:** By applying `.to_lowercase()` before sorting, words like "Eat" and "tea" correctly resolve to the same signature (`aet`) and group together.
* **Identical Duplicates:** If the exact same word appears multiple times in the input, it is safely duplicated within its respective anagram group.

## Anything special