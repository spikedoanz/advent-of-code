import Aoc.AocInputs

def input := day6input

/-
example input
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...

expected answer 41
-/

#eval input
/-
seems to be a cellular automata problem
shape patterns: (patt1)
  #   ->   #
  ^        >

  >#  ->   V#

  V   ->   <
  #        #

  #<  ->   #^

 ' '  ->   ^   -- skips empty spaces. ditto for the rest
  ^       ' '
only pattern matching with the cell immediately in front of the cursor is important
rewriting the entire matrix seems inefficient (as is for a pure functional approach
so a local rewrite is possible:

there is also the pattern of the cursor shift (patt2)

  on positions where x merely rotates:
  #  -> (curr_x, curr_y), (next_x, next_y) -> (curr_x, curr_y), (curr_x + 1, curr_y)
  ^
        - cursor position didn't change, so curr position didn't change
        - but where it points changes, thus changing (next_x, next_y)
  
  on positions where x moves forward:
 ' ' -> (curr_x, curr_y), (next_x, next_y) -> (next_x, next_y), (next_x, next_y - 1)
  ^

  - coordinate system is based off array indexing position, from the og input
  
let mut seen_idx := ![]
while cursor is on screen:
  1. track 2 tuples: (curr_x, curr_y), (next_x, next_y)
  2. extract out from matrix
  3. match against one of the patterns in (patt1)
  4. rewrite the pattern into the matrix
  5. update the coordinate tuple, based on (patt2)
  6. attempt insert into seen_idx (if hasn't seen, insert, else ignore)

#eval seen_idx.sum
-/


