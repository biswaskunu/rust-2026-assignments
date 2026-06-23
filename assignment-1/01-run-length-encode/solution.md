# Solution notes: Run-length encode

## Approach
split the input into individual by .chars() and get the iterators in variable char.
get first character in current_char by chars.next() .
put current_char into c as some(mut c) which runs if part when none is not returned,
start traversing chars and counting each element, when miss match found pust to vector with pair
finally return result

## Edge cases handled
the "if" executes only when current_char is not None so for empty string the vecot is returned empty

## Anything special

tried to make it compact by 3-4 modifications
