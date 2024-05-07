mod aoc; use aoc::*;
use std::collections::HashMap;

fn parse(rule: String, seq: String, map: &HashMap<String, String>) -> Vec<String> {
    let ps: Vec<&str> = map.get(&rule).unwrap().split(" ").collect();
    let mut ret: Vec<String> = Vec::new();
    let mut eval: Vec<String> = vec![seq.clone()];
    if ps[0].starts_with('"') {
        if ps[0].chars().nth(1) == seq.chars().next() {
            if seq.len() == 1 {
                return vec!["".to_string()];
            }
            return vec![seq[1..].to_string()];
        }
        return ret;
    }
        
    for p in ps {
        if p == "|" {
            ret.extend(eval.clone());
            eval = vec![seq.clone()];
            continue;
        }
        let mut next_eval: Vec<String> = Vec::new();
        for e in eval.iter() {
            let curr = parse(p.to_string(), e.clone(), map);
            for c in curr {
                next_eval.push(c);
            }
        }
        eval = next_eval;
    }
    ret.extend(eval.clone());
    return ret;
}

fn part1(filename: &str) -> i32 {
    let mut lines: Vec<String> = Vec::new();
    for line in readlines(filename).expect("lines") {
        lines.push(line.expect("line"));
    }
    let mut rs: Vec<String> = Vec::new();
    let mut ms: Vec<String> = Vec::new();
    
    let mut sep = 0;
    for line in lines {
        if line == "" {
            sep = 1;
            continue;
        }
        if sep == 0 { rs.push(line); }
        else { ms.push(line); }
    }

    // for a in rs { println!("{}", a); }
    // for a in ms { println!("{}", a); }
    
    let mut map: HashMap<String, String> = HashMap::new();
    for r in rs {
        let parts: Vec<&str> = r.split(": ").collect();
        map.insert(parts[0].to_string(), parts[1].to_string());
    }
    let mut ret = 0;
    for m in ms {
        let curr = parse("0".to_string(), m.clone(), &map);
        if curr.len() != 0 {
            if curr[0] == "" { ret += 1; }
        }
    }
    return ret;        
}


fn main() {
    println!("Part 1: {}", part1("i19.txt"));
    println!("Part 2: {}", part1("i19-2.txt"));
}
