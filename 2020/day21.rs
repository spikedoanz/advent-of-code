mod aoc; use aoc::*;
use std::collections::HashMap;
use std::collections::HashSet;

fn part1(filename: &str) -> i32 {
    let mut lines: Vec<String> = Vec::new();
    for line in readlines(filename).expect("lines") {
        lines.push(line.expect("line"));
    }

    let mut map: HashMap<String, HashSet<&str>> = HashMap::new();
    
    for line in lines {
        let parts: Vec<&str> = line.split(" (").collect();

        if parts.len() == 2 {
            let ing: Vec<&str> = parts[0].split(" ").collect();
            let ing_set: HashSet<&str> = ing.iter().cloned().collect();
            let alg: Vec<&str> = parts[1][..parts[1].len()-1].split(" ").collect();
            for a in alg {
                match a {
                    "contains" => {},
                    _ => {
                        let name = a.replace(",", "");
                        if map.contains_key(a) {
                            print!("ding");
                        }
                        else {
                            map.insert(name.clone(), ing_set.clone());
                        }






                    },
                }
            }

            // println!("{} {:?}", ing, alg);
        }
    }

    return 0;        
}


fn main() {
    println!("Part 1: {}", part1("i21.tst"));
    // println!("Part 2: {}", part1("i19-2.txt"));
}
