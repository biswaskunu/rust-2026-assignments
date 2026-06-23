# Solution notes: Split and double

## Approach

The objective is to split a mutable vector into two distinct mutable slices at a given index, double every integer within both slices in place, and return the two modified slices.

1. **Safe Disjoint Splitting:** Standard slice indexing doesn't allow multiple mutable references to different parts of the same vector simultaneously because the borrow checker cannot guarantee they don't overlap. To bypass this safely, we use Rust's built-in `split_at_mut(mid)`. This returns two separate, non-overlapping mutable slices: `left` ($[0 \dots mid-1]$) and `right` ($[mid \dots \text{end}]$).
2. **In-place Mutation:** We iterate over each slice. Because we need to modify the underlying values, we invoke `.iter_mut()` to get mutable references (`&mut i32`) to each element.
3. **De-referencing:** Inside the loops, we dereference the pointer using the asterisks operator (`*x *= 2`) to modify the actual integer value stored in memory.
4. **Return Slices:** Finally, we return the two mutable slices as a tuple `(left, right)`.

## Edge cases handled

* **`mid` at Boundaries ($0$ or `xs.len()`):** * If `mid == 0`, `left` is empty and `right` contains all elements. 
  * If `mid == xs.len()`, `left` contains all elements and `right` is empty. 
  The loops handle empty slices gracefully by simply not executing.
* **Out of Bounds `mid`:** If `mid > xs.len()`, the standard library's `split_at_mut` function will panic intentionally to prevent memory-unsafe access. 

## Anything special

### Rust Borrow Checker Mechanics
* **Why `iter_mut()` is mandatory:** As noted in the code comments, attempting a direct loop like `for x in left` would try to consume/move the slice itself, which fails because `left` is a mutable borrow (`&mut [i32]`) and does not implement `Copy`. Using `.iter_mut()` yields mutable references to the items *inside* the slice without trying to take ownership of the slice structure itself.
* **Re-borrowing Rules:** The function signature successfully returns `(&mut [i32], &mut [i32])` bound to the lifetime of the input vector `xs`. This works exclusively because `split_at_mut` uses internal `unsafe` code guaranteed by the standard library to prove to the compiler that the two returned slices point to completely distinct, non-overlapping memory regions.