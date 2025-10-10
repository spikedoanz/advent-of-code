import Std 
import Aoc.AocTestInputs

def toLists (l : List String) (sep : String) : List (List Nat) := 
  l.map (λ x => x.splitOn sep |>.filterMap (λ x => x.toNat?)) |>.filter (λ x => x != [])

def getOrders (orders : List (List Nat)) (n : Nat) :=
  let matched := orders.filter (λ x => x[1]! == n)
  let rec extractFirsts (orders : List (List Nat)) : List Nat :=
    match orders with
    | fst :: rst => fst[0]! :: (extractFirsts rst)
    | [] => []
  extractFirsts matched

def isIn (l : List Nat) (n : Nat) : Bool :=
  match l.find? (λ x => x == n) with
  | some _ => true
  | none => false

def isAnyIn (l : List Nat) (ns : List Nat) : Bool :=
  match ns with
  | fst :: rst => (isIn l fst) || isAnyIn l rst
  | [] => false

def validate (orders : List (List Nat)) (l : List Nat) : Bool :=
  match l with
  | fst :: rst => 
    let matched := getOrders orders fst
    !(isAnyIn rst matched) && validate orders rst
  | [] => true

def Graph := List (Nat × List Nat)

def addEdge (graph : Graph) (source target : Nat) : Graph :=
  match graph.find? (fun (node, _) => node == source) with
  | some (_, neighbors) =>
      let graph' := graph.filter (fun (node, _) => node != source)
      (source, target :: neighbors) :: graph'
  | none =>
      (source, [target]) :: graph

def buildGraph (orders : List (List Nat)) : Graph :=
  -- First, collect all unique nodes
  let allNodes := orders.foldl (init := []) fun acc order =>
    match order with
    | [a, b] => 
        let acc := if acc.contains a then acc else a :: acc
        if acc.contains b then acc else b :: acc
    | _ => acc
  
  -- Initialize graph with all nodes having empty lists
  let initGraph := allNodes.map (fun n => (n, []))
  
  -- Now add edges
  orders.foldl (init := initGraph) fun graph order =>
    match order with
    | [before, after] =>
        match graph.find? (fun (node, _) => node == before) with
        | some (_, neighbors) =>
            let graph' := graph.filter (fun (node, _) => node != before)
            (before, after :: neighbors) :: graph'
        | none => graph
    | _ => graph


def removeEdge (g: Graph) (e: Nat) : Graph :=
  match g with
  | h :: g' =>
    if    h.1 == e then removeEdge g' e
    else  (h.1, h.2.filter (fun x => x != e)) :: removeEdge g' e
  | [] => []


def getMinEdge (g : Graph) : Option Nat :=
  match g with
  | [] => none
  | h :: t => some (t.foldl (fun min x => if x.2.length < min.2.length then x else min) h).1



partial def topoSort (l : List Nat) (g : Graph) : List Nat :=
  match g with
  | [] => []
  | _ =>
    match getMinEdge g with
    | some min =>
      let graph' := removeEdge g min
      match l.find? (· == min) with
      | some m => m :: topoSort (l.filter (· != m)) graph'
      | _ => topoSort l graph'
    | none => []

def localSort (l : List Nat) (orders : List (List Nat)) : List Nat :=
  let localOrders := orders.filter (fun order =>
    match order with
    | [a,b] => l.contains a && l.contains b
    | _ => false)
  let localGraph := buildGraph localOrders
  topoSort l localGraph

--------------------------------------------------------------------------------
def input := day5input
def halves := input.splitOn "\n\n"
def fst := halves[0]!.splitOn "\n"
def snd := halves[1]!.splitOn "\n"

def orders := toLists fst "|" |>.filter (λ x => x.length == 2)
def lsts := toLists snd  ","

def validated := lsts.filter (λ x => validate orders x)
def getMiddle (l : List Nat) := l[l.length / 2]!

def unvalidated := lsts.filter (λ x => !validate orders x)
def sortedUnvalidated := unvalidated.map (fun x => localSort x orders)

def part1 := validated.map getMiddle |>.sum
def part2 := sortedUnvalidated |>.map getMiddle |>.sum

#eval part1
#eval part2
