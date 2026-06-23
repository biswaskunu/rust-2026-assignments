# Solution notes: Character frequency

## Approach

The goal is to count the frequency of each character in a given string and return the results sorted. The sorting must prioritize higher frequencies first; if frequencies are identical, it must fall back to alphabetical order.

1. **Frequency Counting via Hash Map:** We initialize a `HashMap<char, u32>` to track counts. We loop through every character `c` in the input string using `input.chars()`.
2. **The Entry API:** Rather than using the commented-out `if let` block, we use Rust's idiomatic `freq.entry(c).or_insert(0)`. This returns a mutable reference to the count (inserting `0` if the character isn't already tracked), which we immediately dereference and increment (`+= 1`).
3. **Conversion to Vector:** Because maps are unordered collections, we consume the map and turn it into a list of tuples using `freq.into_iter().collect()`. This yields a `Vec<(char, u32)>`.
4. **Custom Sorting:** We sort the vector using `.sort_by()` paired with a custom closure:
   * **Primary Criterion (`b.1.cmp(&a.1)`):** We compare the counts in reverse order (`b` vs `a`) to achieve a descending sort (highest frequency first).
   * **Secondary Criterion (`then_with(...)`):** If the counts are equal, `then_with` breaks the tie by sorting the characters (`a.0.cmp(&b.0)`) in ascending alphabetical order.

## Edge cases handled

* **Empty String:** If the input is `""`, the loop never runs, the vector is empty, and an empty `Vec` is safely returned without any panics.
* **Tie-Breaking Order:** Characters with the exact same count are reliably sorted in ascending alphabetical order rather than remaining in arbitrary hash map order.
* **Whitespace and Special Characters:** Spaces, punctuation, and newlines are counted and sorted just like regular letters because `.chars()` evaluates all valid Unicode scalar values.

## Anything special

### Idiomatic Entry API Optimization
* The pattern `*freq.entry(c).or_insert(0) += 1` is highly efficient in Rust. It computes the hash of the character exactly once to locate its bucket, whereas standard `contains_key` followed by an `insert` requires hashing the key twice.

### Multi-Criteria Sorting
* The `.then_with()` method is a powerful feature of Rust's `Ordering` enum. It lazily evaluates the second comparison *only* if the primary comparison evaluates to `Ordering::Equal`. This keeps the sorting logic incredibly clean, readable, and lightning-fast compared to writing nested `match` statements or manual conditional branches.