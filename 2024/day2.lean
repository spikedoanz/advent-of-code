namespace List
def input := ("
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
".splitOn "\n").filter (λ x => x != "")

def nums := input.map (λ line => (line.splitOn " ").filterMap (·.toInt?))

def diff (l : List Int) : List Int :=
  zipWith (λ a b => a - b)  (l.drop 1) l

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

def _count : Bool -> Nat := λ x => if x then 1 else 0

#eval nums.map is_safe |>.map _count |> sum

--------------------------------------------------------------------------------

def is_safe_with_removal (l : List Int) : Bool :=
  is_safe l || 
  (List.range l.length).any (λ i => is_safe (l.eraseIdx i))

#eval nums.map is_safe_with_removal |>.map _count |> sum
