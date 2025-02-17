namespace List
def input := "2333133121414131402"
def list := input.toList.map (λ x => x.toNat-48)

def stringMul (s : String) (n : Nat) : String := String.join (List.replicate n s)

def diskify : List Nat -> Nat -> List String
  | [], _ => []
  | [x], n => [stringMul (Char.ofNat (n + 48) |> toString) x]
  | x :: y :: ls, n =>
    stringMul (Char.ofNat (n + 48) |> toString) x :: stringMul "." y :: diskify ls (n + 1)

def disk := (String.join (diskify list 0)).toList

def nums := (filter (λ x => x != '.') disk).reverse

def fill : List Char -> List Char -> List Char
  | '.' :: ds, n :: ns => n :: fill ds ns
  | d :: ds, ns => d :: fill ds ns
  | _, _ => []

def sorted_disk := zipWith (λ a _ => a.toNat-48) (fill disk nums) nums

#eval sum (zipWith (λ a b => a * b) (range (length sorted_disk)) sorted_disk)
