# Solution notes: Censor vowels in place

## Approach

initialize an empty string result
scan through s and pushing each into result, 
before pushing, check if the current char is a vowel then repalce the appending char 'st' with "*"

## Edge cases handled

for empty string the for loop doesnot run , so the default empty result is returned

## Anything special

_Tricks, alternatives you considered, performance notes, etc._
