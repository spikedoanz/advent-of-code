import Lean.Data.HashMap

def getInt (s: String) : Option Int :=
  if s.isEmpty then
    some 0
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

def main : IO Unit := do
  let text ← IO.FS.readFile "2018/example.txt"
  IO.println "Part 1:"
  IO.println (part1 (text.splitOn "\n"))


#eval main

