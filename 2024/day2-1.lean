namespace List
def input := ("
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
".splitOn "\n").filter (λ x => x != "")

def num := input.map (λ line => (line.splitOn " ").filterMap (·.toInt?))

def diff (l : List Int) : List Int :=
  zipWith (λ a b => a - b)  (l.drop 1) l

def safe : List Int -> Bool
  | f :: s :: rst =>
    if      max f (-f) > 3 || max f (-f) < 1 then false
    else if f * s <= 0                       then false
    else    safe (s :: rst)
  | f :: rst =>
    if      max f (-f) > 3 || max f (-f) < 1 then false
    else    safe rst
  | _ => true

#eval num.map (λ n => safe (diff n)) |>.map (λ x => if x then 1 else 0) |> sum
