namespace List
def input := "
3   4
4   3
2   5
1   3
3   9
3   3
"

def lines := input.splitOn "\n"
def pairs := (lines.map (λ line => line.splitOn " ")).filter (λ x => x != [""])
def nums := pairs.map (λ pair => pair.filterMap String.toNat?)

def left := mergeSort (nums.map (λ n => n.get! 0)) (λ a b => a <= b)
def rght := mergeSort (nums.map (λ n => n.get! 1)) (λ a b => a <= b)

def diff := sum (zipWith (λ a b => max (a-b) (b-a)) left rght)

#eval diff
