# Solution notes: Min max

## Approach

The goal is to find both the minimum and maximum values in a slice of integers in a single pass, returning them wrapped safely in an `Option`.

1. **Empty Check:** We first check if the slice is empty using `xs.is_empty()`. If it is, there is no valid minimum or maximum, so we immediately return `None`.
2. **Initialization:** We initialize both `min` and `max` variables with the first element of the slice (`xs[0]`). This gives us a valid baseline to compare against.
3. **Single-Pass Loop:** We iterate through every element in the slice. By using the pattern `&x` in the `for` loop, we copy/dereference the value directly (since `i32` implements `Copy`), allowing us to work with a plain `i32` instead of a reference `&i32`.
4. **Tracking Updates:** For each element, we perform two independent checks:
   * If `x < min`, we update our minimum tracker.
   * If `x > max`, we update our maximum tracker.
5. **Return Value:** After checking all elements, we return the final pair wrapped in `Some((min, max))`.

## Edge cases handled

* **Empty Slice:** Handled gracefully at the beginning by returning `None`, preventing out-of-bounds panics when accessing `xs[0]`.
* **Single-Element Slice:** If the slice has only one element, the loop evaluates it, keeps `min` and `max` equal to that single element, and correctly returns `Some((x, x))`.
* **All Elements Identical:** If the input is something like `[5, 5, 5]`, the conditions `x < min` and `x > max` simply evaluate to false, and the function correctly returns `Some((5, 5))`.
* **Negative Numbers:** Since `min` and `max` are initialized to an actual element from the slice rather than arbitrary values like `0`, the logic correctly tracks negative ranges (e.g., `[-10, -5, -2]`).

## Anything special

### Performance Note
* **Linear Time Complexity ($O(N)$):** This approach visits each element exactly once, making it highly efficient. 
* **Redundant Comparison:** Note that for the very first element (`xs[0]`), the loop evaluates `xs[0] < xs[0]` and `xs[0] > xs[0]`. While technically redundant, skipping the first element using `.iter().skip(1)` generally introduces slightly more iterator overhead than the simple CPU branch predictor cost of running the checks on index 0.

### Alternative Idiomatic Rust
* Alternatively, the Rust standard library provides built-in methods on iterators that could achieve this in a single line, such as:
  ```rust
  xs.iter().min().zip(xs.iter().max()).map(|(&min, &max)| (min, max))