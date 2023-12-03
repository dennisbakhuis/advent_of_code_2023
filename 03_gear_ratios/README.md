# Day 3: Gear Ratios
We have met an engineer elf at the Gondola Lift, who yet again has a problem.
The gandola lift, the one we need to take, is broken and we need to identify
the parts. These are on an cryptic map.

## Puzzle 1 - find all parts
Parts, or actually part numbers are the numbers that are connected to a
symbol. We need to look through the map for numbers and check if there are
any symbols around that number.


## Puzzle 2 - find gears and calculate gear ratios
Now we need to find gears. Gears are '*' that are connected by at exactly two
numbers. The gear ratio is the product of these two numbers and the answer is
the sum of all gear ratios.

This puzzle was a bit harder, not because the puzzle was harder but because
it got a bit into a fight with the Rust compiler. I wanted to create some
structures to hold all the data and did not pay too much attention to 
ownership and lifetimes. 

