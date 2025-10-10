import Aoc.AocInputs
import Aoc.Std

def parseMatrix (input : String) : Array (Array Char) :=
  input.splitOn "\n" 
  |>.filter (·.length > 0)  -- Remove empty lines
  |>.map (·.toList.toArray)
  |>.toArray
def directions : Array (Int × Int) := #[
  (0, 1),   -- right
  (1, 0),   -- down
  (1, 1),   -- diagonal down-right
  (1, -1),  -- diagonal down-left
  (0, -1),  -- left
  (-1, 0),  -- up
  (-1, -1), -- diagonal up-left
  (-1, 1)   -- diagonal up-right
]

def getChar (matrix : Array (Array Char)) (row col : Int) : Option Char := do
  guard (0 ≤ row && row < matrix.size)
  guard (0 ≤ col && col < matrix[row.toNat]!.size)
  return matrix[row.toNat]![col.toNat]!

def checkWord 
  (matrix : Array (Array Char)) 
  (word : String)
  (row col : Int) 
  (dr dc : Int) : Bool :=
  (enumerate word.data).all fun (i,c) =>
    getChar matrix (row + i * dr) (col + i * dc) = some c

def countWord (matrix : Array (Array Char)) (word : String) : Nat := Id.run do
  let mut count := 0
  let firstChar := word.data[0]!
  for r in [:matrix.size] do
    for c in [:matrix[r]!.size] do
      if matrix[r]![c]! = firstChar then
        for (dr, dc) in directions do
          if checkWord matrix word r c dr dc then
            count := count + 1
  
  return count

def checkXPattern (matrix : Array (Array Char)) (row col : Int) : Bool :=
  -- Center must be 'A'
  if getChar matrix row col != some 'A' then false
  else
    -- Check diagonal 1: top-left to bottom-right
    let tl := getChar matrix (row - 1) (col - 1)
    let br := getChar matrix (row + 1) (col + 1)
    let diag1Valid := (tl = some 'M' && br = some 'S') ||
                      (tl = some 'S' && br = some 'M')

    -- Check diagonal 2: top-right to bottom-left
    let tr := getChar matrix (row - 1) (col + 1)
    let bl := getChar matrix (row + 1) (col - 1)
    let diag2Valid := (tr = some 'M' && bl = some 'S') ||
                      (tr = some 'S' && bl = some 'M')

    diag1Valid && diag2Valid

def countXPattern (matrix : Array (Array Char)) : Nat := Id.run do
  let mut count := 0
  -- Skip edges since X pattern needs surrounding cells
  for r in [1:matrix.size-1] do
    for c in [1:matrix[r]!.size-1] do
      if checkXPattern matrix r c then
        count := count + 1
  return count

--------------------------------------------------------------------------------

def input := day4test
def matrix := parseMatrix input

def part1 := countWord matrix "XMAS"
def part2 := countXPattern matrix

#eval part1
#eval part2
