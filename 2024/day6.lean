import Aoc.AocTestInputs

def inBounds (m : List (List Char)) (y : Nat) (x : Nat) : Bool :=
  y < m.length && (match m[y]? with | some line => x < line.length | none => false)

def getChar (m : List (List Char)) (y : Nat) (x : Nat) : Char :=
  if inBounds m y x then m[y]![x]! else ' '

def findCursor (m : List (List Char)) : Option (Nat × Nat × Char) :=
  let rec findInLine (l : List Char) (y : Nat) (x : Nat) : Option (Nat × Nat × Char) :=
    match l with
    | [] => none
    | c :: rst => 
      if c == '^' || c == '>' || c == 'v' || c == '<' 
      then some (y, x, c)
      else findInLine rst y (x + 1)
  
  let rec findInMatrix (lines : List (List Char)) (y : Nat) : Option (Nat × Nat × Char) :=
    match lines with
    | [] => none
    | line :: rst =>
      match findInLine line y 0 with
      | some coord => some coord
      | none => findInMatrix rst (y + 1)
  
  findInMatrix m 0

def turnRight (dir : Char) : Char :=
  match dir with
  | '^' => '>'
  | '>' => 'v'
  | 'v' => '<'
  | '<' => '^'
  | _ => dir

def getNextPos (y : Nat) (x : Nat) (dir : Char) : (Int × Int) :=
  match dir with
  | '^' => (y - 1, x)
  | '>' => (y, x + 1)
  | 'v' => (y + 1, x)
  | '<' => (y, x - 1)
  | _ => (y, x)

def simulate (m : List (List Char)) (startY : Nat) (startX : Nat) (startDir : Char) : Nat :=
  let rec loop (y : Nat) (x : Nat) (dir : Char) (visited : List (Nat × Nat)) (fuel : Nat) : List (Nat × Nat) :=
    if fuel = 0 then visited
    else
      -- add current position to visited
      let newVisited := if visited.contains (y, x) then visited else (y, x) :: visited
      
      -- get next position
      let (nextY, nextX) := getNextPos y x dir
      
      -- check if next position is out of bounds
      if nextY < 0 || nextX < 0 then newVisited
      else
        let nextY' := nextY.toNat
        let nextX' := nextX.toNat
        
        if !inBounds m nextY' nextX' then newVisited
        else
          -- check what's at the next position
          let nextChar := getChar m nextY' nextX'
          if nextChar == '#' then
            -- turn right and stay in same position
            loop y x (turnRight dir) newVisited (fuel - 1)
          else
            -- move forward
            loop nextY' nextX' dir newVisited (fuel - 1)
  
  let visited := loop startY startX startDir [] 10000
  visited.length

def part1 (matrix : List (List Char)) : Nat :=
  match findCursor matrix with
  | none => 0
  | some (y, x, dir) => simulate matrix y x dir

--------------------------------------------------------------------------------

def input := day6input
def matrix := input.splitOn "\n" |>.map String.toList

#eval part1 matrix
