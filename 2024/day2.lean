import Aoc.AocInputs
def diff (l : List Int) : List Int :=
  List.zipWith (λ a b => a - b)  (l.drop 1) l

def is_monotonic : List Int -> Bool
  | f :: s :: rst =>
    if f * s <= 0 then false
    else is_monotonic (s :: rst)
  | _ => true

def abs := λ x => max x (-x)

def is_safe_diff : List Int -> Bool
  | f :: rst =>
    if abs f > 3 || abs f < 1 then false
    else is_safe_diff rst
  | _ => true

def is_safe (l : List Int) : Bool :=
  let d := diff l
  is_monotonic d && is_safe_diff d

def is_safe_with_removal (l : List Int) : Bool :=
  is_safe l ||
  (List.range l.length).any (λ i => is_safe (l.eraseIdx i))

def _count : Bool -> Nat := λ x => if x then 1 else 0

--------------------------------------------------------------------------------
def input := day2input.splitOn "\n" |>.filter (λ x => x != "")

def nums := input.map (λ line => (line.splitOn " ").filterMap (·.toInt?))

def part1 := nums.map is_safe |>.map _count |> List.sum
def part2 := nums.map is_safe_with_removal |>.map _count |> List.sum

#eval part1
#eval part2
