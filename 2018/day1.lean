import Lean.Data.HashSet

def getInt (s: String) : Option Int :=
  if s.isEmpty then
    none
  else if s.front == '-' then
    s.toInt?
  else
    (s.drop 1).toInt?

def part1 (lines : List String) : Int := 
  match lines with
  | [] => 0
  | line :: rest =>
    match getInt line with
    | some n => n + part1 rest
    | none => part1 rest

def part2 (curr : Int := 0)
          (lines : List String)
          (seen : Lean.HashSet Int := Lean.HashSet.empty) 
          (source : List String) 
          (max_loops : Nat := 10^20) : Int :=
  if max_loops = 0 then
    0
  else
    match lines with
    | [] => part2 curr source seen source (max_loops - 1)
    | line :: rest =>
      if seen.contains curr then
        curr
      else
        match getInt line with
        | some n => part2 (curr + n) rest (seen.insert curr) source (max_loops - 1)
        | none => part2 curr rest seen source (max_loops - 1)
termination_by max_loops

def main : IO Unit := do
  let text ← IO.FS.readFile "2018/inputday1.txt"
  IO.println "-- Part 1:"
  IO.println (part1 (text.splitOn "\n"))
  IO.println "-- Part 2:"
  IO.println (part2 0 (lines := text.splitOn "\n") (source := text.splitOn "\n"))

#eval main
