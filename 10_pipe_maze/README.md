# Day 10: Pipe maze
We used the kite to get up to Metal island. As the name suggested, everything
is made of metal here. We notice an animal watching us and then disapearing
into a huge pipe system.

## Puzzle 1 - length of the pipe system
To outsmart the animal we are going to map the pipe system and calculate the
maximum distance between two points in the pipe loop. I started this puzzle
too simplistic and eventually made a solver that did the job but was
incompatible with puzzle 2.

## Puzzle 2 - number of nodes enclosed by the loop
Now we need to find the number of nodes that are enclosed by the loop. I
realized how to solve this puzzle much to late and wasted quite some time.
Eventually, I first rewrote puzzle 1 such that I have also a direction
for each point in the loop. With this direction I can then get the vertical
stroke and use that to use a non-zero unwinding method to calculate if a point
is within the loop.

This was a very challenging day, also because I had a small fight with the
Rust compiler again.

![The map with the vertical strokes and enclosed nodes.](https://github.com/dennisbakhuis/advent_of_code_2023/blob/main/10_pipe_maze/pipe_maze_map.png?raw=true)

