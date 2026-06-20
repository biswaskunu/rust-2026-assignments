# Solution notes: Longest word slice

## Approach

split the sentence into sords w.r.t white spaces.
get the first word in var - longest
initialize longest_length by the length of the string inside longest by unwraping.
found the maximum length substring and assign it to longest
return longest

## Edge cases handled

when empty string is faced, .next() method gives None, so the if part is not executd and the default
None value is returned

## Anything special

_Tricks, alternatives you considered, performance notes, etc._
