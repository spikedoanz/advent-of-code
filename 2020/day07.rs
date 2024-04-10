use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use std::collections::HashMap;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where 
    P: AsRef<Path>,
{
    Ok(io::BufReader::new(File::open(filename)?).lines())
}

fn count_key(bag: &str, map: &HashMap<String, Vec<(i32, String)>>) -> i32 {
    if bag == "shiny gold bag" { 
        return 1; 
    }
    if !map.contains_key(bag) { return 0; }
    let mut count = 0;
    let contains = &map[bag];
    for thing in contains {
        let _num = thing.0;
        let con = thing.1.clone();
        count += count_key(&con, map);
    }
    return count;
}


fn part1(filename: &str) -> i32 {
    let lines = read_lines(filename).expect("lines");
    let mut total = 0;
    let mut bags: HashMap<String, Vec<(i32, String)>> = HashMap::new();
    for line in lines {
        let mut line = line.expect("line");
        line = line.replace("bags", "bag");
        line = line.replace(".", "");
        let map: Vec<&str> = line.split("contain").collect();
        let contains: Vec<&str> = map[1].split(", ").collect();
        let key = map[0].trim().to_string();

        //Create the hashmap 
        let mut entry: Vec<(i32, String)> = Vec::new();
        if contains[0].trim() != "no other bag" {
            for content in &contains {
                let info: Vec<&str> = content.trim().split(' ').collect();
                let num = info[0].parse::<i32>().unwrap();
                let bag = info[1..].join(" ");
                entry.push((num, bag));
            }
        }
        bags.insert(key, entry);
    }

    for (key, _value) in &bags {
        if count_key(&key, &bags) > 0 && key != "shiny gold bag" {
            total += 1;
        }
    }
    total 
}

fn count_bags(bag: &str, bags: &HashMap<String, Vec<(i32, String)>>) -> i32 {
    let mut total = 1;
    let index = &bags[bag];
    for (num, key) in index {
        total += num * count_bags(&key, &bags);
    }
    return total;
}

fn part2(filename: &str) -> i32 {
    let lines = read_lines(filename).expect("lines");
    let mut bags: HashMap<String, Vec<(i32, String)>> = HashMap::new();
    for line in lines {
        let mut line = line.expect("line");
        line = line.replace("bags", "bag");
        line = line.replace(".", "");
        let map: Vec<&str> = line.split("contain").collect();
        let contains: Vec<&str> = map[1].split(", ").collect();
        let key = map[0].trim().to_string();

        //Create the hashmap 
        let mut entry: Vec<(i32, String)> = Vec::new();
        if contains[0].trim() != "no other bag" {
            for content in &contains {
                let info: Vec<&str> = content.trim().split(' ').collect();
                let num = info[0].parse::<i32>().unwrap();
                let bag = info[1..].join(" ");
                entry.push((num, bag));
            }
        }
        bags.insert(key, entry);
    }
    return count_bags("shiny gold bag", &bags);
}

fn main() {
    println!("Part 1: {}", part1("inputday7.txt"));
    println!("Part 2: {}", part2("inputday7.txt") - 1);
}
