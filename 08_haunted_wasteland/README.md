# Day 8: Haunted Wasteland
Apparently, we were sitting on the back of a camel with a ghost. As a sand
storm approaches, the elf vanishes and we are left alone with a bunch of maps.
These maps have a left/right selector and a mapping.

## Puzzle 1 - how many steps required to go from AAA to ZZZ
This puzzle is not to hard, you need to book-keep the location and just
iterate through the mappings up to the solution. This took me not too long.
As I expected multiple paths for the next puzzle I created it such that it
accepts multiple path inputs.

## Puzzle 2 - multiple paths 
While I expected this puzzle, I did not expect it to come together. While I
could almost immediately run the code, after a few minutes I realized we
again could not work with a naive linear approach.

While I found by myself that eventually the indiviual matches should align and
that the pattern repeat together, I was not smart enough to think of the
least common multiple and needed a tiny nudge (hint). I was so close :-)
