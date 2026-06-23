# Solution notes: Inventory restock and summary

## Approach

The code manages a simple inventory system represented as a list of key-value pairs using a vector of tuples (`Vec<(String, u32)>`), where each tuple contains an item name and its current quantity.

### 1. Restock Function
* **Iterate over New Stock:** The function consumes the incoming `more` vector and iterates through each `(name, qty)` pair.
* **Linear Search:** For each incoming item, it uses `.iter_mut().find(...)` to perform a linear search through the current `inventory`.
* **Update or Insert:** * If a matching name is found (`Some(existing)`), it updates the existing quantity in place (`existing.1 += qty`).
  * If the item is entirely new, it appends the `(name, qty)` pair to the end of the `inventory` vector.

### 2. Summary Function
* **Count Items:** It determines the total number of unique item types using `.len()`.
* **Sum Total Units:** It iterates over the borrowed inventory slice, extracts the quantity (`item.1`), and aggregates them using `.sum()`.
* **Format:** It builds and returns a clean, summary string utilizing the `format!` macro.

## Edge cases handled

* **Empty Input Vectors:** * If `more` is empty, the loop never runs, and the original inventory is returned untouched.
  * If the initial `inventory` is empty, every item from `more` falls into the `else` block and is pushed as a new item.
* **Duplicate Items in `more`:** If the incoming `more` vector contains multiple entries of the same item name, the first instance is appended to the inventory (if it didn't exist), and subsequent instances will successfully find and update that newly added item.

## Anything special

### Performance and Trade-offs
* **Linear Time Complexity ($O(N \times M)$):** Because `inventory` is structured as a `Vec` rather than a `HashMap`, finding an existing item requires a sequential scan. This is highly efficient for small inventories due to CPU cache locality but will scale poorly ($O(N \cdot M)$) if both the current inventory ($N$) and the incoming restock list ($M$) grow large.
* **Alternative Considered:** If performance becomes an issue with larger datasets, converting the inventory structure to a `HashMap<String, u32>` or sorting the `Vec` and using a binary search would reduce lookup times to $O(1)$ or $O(\log N)$ respectively.

### Idiomatic Rust
* **In-place Mutation:** The `inventory` parameter uses `mut inventory` by value. This allows the function to reuse the allocated memory of the original vector instead of creating and returning a brand new one.
* **Tuple Pattern Matching:** Destructuring tuples directly within the `for` loop (`for (name, qty) in more`) makes the code readable and idiomatic.