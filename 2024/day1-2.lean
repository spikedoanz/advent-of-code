import Std
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

def left_original := nums.map (λ n => n.get! 0)
def right_original := nums.map (λ n => n.get! 1)

def createFreqMap (l : List Nat) : Std.HashMap Nat Nat :=
  l.foldl (λ m n => m.insert n (m.getD n 0 + 1)) (Std.HashMap.empty)

def similarityScore :=
  let freqMap := createFreqMap right_original
  left_original.foldl (λ sum n => sum + n * (freqMap.getD n 0)) 0

#eval similarityScore

end List
