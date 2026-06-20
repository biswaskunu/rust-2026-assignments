# Solution notes: Reverse the word order

## Approach

split the given sentence w.r.t white spaces,
initialize an empty string reveresed_string
traverse through the words in reverse order and append all the words to reversed string witha white space
return reversed_string

## Edge cases handled

the initialization of reversed_string is empty string ,
so if the given string is empty the returned value is also empty.
the .split_whitespace() method takes care of cosecutive white spaces while forming words

## Anything special

_Tricks, alternatives you considered, performance notes, etc._
