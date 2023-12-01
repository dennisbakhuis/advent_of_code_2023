# Day 1: Trebuchet puzzles
The first two puzzles are decoding a specific document. This document contains
lines of text which have numbers encoded in them. The puzzles have an increase
in difficulty. 

## Puzzle 1 - just numbers
The first puzzle only takes into account digit characters. To get the proper
value we need to extract the first character from both sides (left and right),
combine these together, and convert them to a number. E.g. `dhf4bla5bla2uyo`
will result in `42`. The final answer is the sum of all numbers.


## Puzzle 2 - adding "written" digits
Some might already have seen but some lines contained words of numbers, e.g.
`eight` or `two`. The puzzle is the same so we still need to find the first
digit on the left and right, however, written digits also counts. 

My first try was a simple dictionary replace, but this did not work as the
order of words matter: `eightwo`. Here eight and two overlap and depending on
what number is translated first, the other is invalid. This fails the puzzle
and therefore, a simple transaltion before the other method did not work.

