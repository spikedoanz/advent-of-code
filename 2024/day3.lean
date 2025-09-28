import Aoc.AocInputs

def parseNum (s : List Char) : Option (List Char × List Char) × List Char :=
  let digitsA := s.takeWhile (·.isDigit)
  let rstA     := s.dropWhile (·.isDigit)
  match rstA with
  | ',' :: rstB =>
    let digitsB := rstB.takeWhile (·.isDigit)
    let rstC    := rstB.dropWhile (·.isDigit)
    match rstC with
    | ')' :: rstD => (some (digitsA, digitsB), rstD)
    | _ => (none, rstC)
  | _ => (none, rstA)

partial def parseMul 
  (s : List Char) 
  (acc : List (List Char × List Char)) 
  : List (List Char × List Char) := 
  match s with
  | 'm' :: 'u' :: 'l' :: '(' :: rstA => 
    let (nums, rstB) := parseNum rstA
    match nums with
    | some pair => parseMul rstB (pair :: acc)
    | _ => parseMul rstA acc
  | _ :: rstA => parseMul rstA acc
  | _ => acc

def pairToInts (a : List Char × List Char) : Option (Int × Int) :=
  match a.1.asString.toInt?, a.2.asString.toInt? with
  | some x, some y => some (x, y)
  | _, _ => none

def nums := (parseMul (day3test.toList) []).filterMap pairToInts

#eval nums.map (fun (a,b) => a * b) |>.sum

--------------------------------------------------------------------------------


