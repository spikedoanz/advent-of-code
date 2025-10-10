import Aoc.AocInputs

def parseNum (s : List Char) : Option (List Char × List Char) × List Char :=
  let digitsA   := s.takeWhile (·.isDigit)
  let rstA      := s.dropWhile (·.isDigit)
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

partial def parseMulToggle
  (s : List Char)
  (acc : List (List Char × List Char))
  (flag : Bool)
  : List (List Char × List Char) :=
  match s with
  -- number parsing
  | 'm' :: 'u' :: 'l' :: '(' :: rstA =>
    let (nums, rstB) := parseNum rstA
    match nums with
    | some pair =>
      match flag with -- flag only gets consumed if pair matched
      | true  =>  parseMulToggle rstB (pair :: acc) flag
      | _     =>  parseMulToggle rstB acc flag -- toss pair if flag false
    | _ =>        parseMulToggle rstA acc flag
  -- flags entrypoint
  | 'd' :: 'o' :: '(' :: ')' :: rst =>
    parseMulToggle rst acc true
  | 'd' :: 'o' :: 'n' :: '\'' :: 't' :: '(' :: ')' :: rst =>
    parseMulToggle rst acc false
  -- base cases
  | _ :: rstA => parseMulToggle rstA acc flag
  | _ => acc

def pairToInts (a : List Char × List Char) : Option (Int × Int) :=
  match a.1.asString.toInt?, a.2.asString.toInt? with
  | some x, some y => some (x, y)
  | _, _ => none

--------------------------------------------------------------------------------
def input := day3input
def nums1 := (parseMul (input.toList) []).filterMap pairToInts
def nums2 := (parseMulToggle (input.toList) [] true).filterMap pairToInts

def part1 := nums1.map (fun (a,b) => a * b) |>.sum
def part2 := nums2.map (fun (a,b) => a * b) |>.sum

#eval part1
#eval part2
