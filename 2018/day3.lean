import Lean.Data.HashMap

def get_nums(s : String) : List (Option Nat) :=
  match s.replace " " "" |>.splitOn "@" with
  | [] => []
  | [""] => []
  | _ :: tail =>
    match String.join tail |>.splitOn ":" with
    | [] => []
    | head :: tail =>
      let ret := (head.splitOn ",") ++ (String.join tail |>.splitOn "x")
      ret.map String.toNat?

def parse_nums (a : List (Option Nat)) : List (Nat × Nat) :=
  match a with
  | [some x, some y, some w, some h] => 
  List.range w |>.bind λ dx => 
    List.range h |>.map λ dy => (x+dx,y+dy)
  | _ => [] 

def part1 (s: String) : Nat := 
  let count : Lean.HashMap (Nat × Nat) Nat := s.splitOn "\n" 
    |>.foldl (init := Lean.HashMap.empty) (λ acc line =>
      let nums := parse_nums (get_nums line)
      nums.foldl (λ acc num =>
        acc.insert num (acc.findD num 0 + 1)
      ) acc
    )
  count.fold (init := 0) (λ acc _ v => if v > 1 then acc + 1 else acc)

def part2 (s: String) : Option String := 
  let lines := s.splitOn "\n"
  
  let count : Lean.HashMap (Nat × Nat) Nat := lines
    |>.foldl (init := Lean.HashMap.empty) (λ acc line =>
      let nums := parse_nums (get_nums line)
      nums.foldl (λ acc num =>
        acc.insert num (acc.findD num 0 + 1)
      ) acc
    )
  
  let onces := count.fold 
    (init := Lean.HashMap.empty) 
    (λ acc k v => if v == 1 then acc.insert k () else acc)
  
  lines.findSome? (λ line =>
    let nums := parse_nums (get_nums line)
    if nums.all (λ num => onces.contains num) then
      some line
    else
      none
  )

def main : IO Unit := do
  let text ← IO.FS.readFile "2018/inputday3.txt"
  IO.println "--- Part1"
  IO.println (part1 text)
  IO.println "--- Part2"
  IO.println (part2 text)

#eval main
