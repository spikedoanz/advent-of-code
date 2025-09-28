def enumerate (l : List α) : List (Nat × α) :=
  List.zip (List.range l.length) l

#eval enumerate "cowabunga".toList
