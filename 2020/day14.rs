mod aoc; use aoc::*;
use std::collections::HashMap;

fn num_to_bin(s: &str) -> String {
    let mut num = s.parse::<i64>().unwrap(); 
    let mut arr: Vec<char> = "000000000000000000000000000000000000".chars().collect();
    let arrsz = (arr.len()-1) as i32;
    for i in 0..36 {
        let curr = i64::pow(2,(arrsz-i) as u32);
        if num >= curr {
            arr[(i) as usize] = '1';
            num -= curr; 
        }
    }
    return arr.iter().collect();
}

fn apply_mask(m: String, b: String) -> String {
    let mask: Vec<char> = m.chars().collect();
    let mut bin: Vec<char>  = b.chars().collect();
    for (i, c) in mask.iter().enumerate() {
        match c {
            'X' => continue,
            _   => bin[i] = *c,
        }
    }
    return bin.iter().collect();
}

fn bin_to_num(s: String) -> i64 {
    let mut chars: Vec<char> = s.chars().collect(); 
    chars.reverse();
    let mut curr: i64 = 1;
    let mut ret = 0;
    for c in chars {
        if c == '1' {
            ret += curr;
        }
        curr *= 2
    }
    return ret;
}

fn part1(filename: &str) -> i64 {
    let mut lines: Vec<String> = Vec::new();
    for line in readlines(filename).expect("lines") {
        lines.push(line.expect("line"));
    }

    let mut mem: HashMap<String, String> = HashMap::new();
    let mut mask: String = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".to_string();
    for line in lines {
        let parts: Vec<&str> = line.split(" = ").collect();
        let oper = parts[0];
        let opan = parts[1];
        match oper {
            "mask" => mask = opan.to_string(),
            _ => {
                //println!("{}", mask);
                //println!("{}", num_to_bin(opan));
                let bin = num_to_bin(opan);
                mem.insert(oper.to_string(), apply_mask(mask.clone(), bin));
            }
        }

        //println!("{} {}", oper, opan);
    }
    let mut ret = 0;
    for (_key, value) in mem {
        ret += bin_to_num(value);
    }
    return ret;
}

fn apply_mask2(m: String, b: String) -> String {
    println!("{}", b);
    let mask: Vec<char> = m.chars().collect();
    let mut bin: Vec<char>  = b.chars().collect();
    for (i, c) in mask.iter().enumerate() {
        match c {
            'X' => bin[i] = *c,
            '1' => bin[i] = *c,
            _   => (),
        }
    }
    return bin.iter().collect();
}

fn part2(filename: &str) -> i64 {
    let mut lines: Vec<String> = Vec::new();
    for line in readlines(filename).expect("lines") {
        lines.push(line.expect("line"));
    }

    let mut mem: HashMap<String, String> = HashMap::new();
    let mut mask: String = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".to_string();
    for line in lines {
        let parts: Vec<&str> = line.split(" = ").collect();
        let oper = parts[0];
        let opan = parts[1];
        match oper {
            "mask" => mask = opan.to_string(),
            _ => {
                let bin = num_to_bin(opan);
                mem.insert(oper.to_string(), apply_mask2(mask.clone(), bin));
            }
        }

    }
    let mut ret = 0;
    for (_key, value) in mem {
        println!("{} {}", _key, value);
        ret += bin_to_num(value);
    }
    return ret;
}

fn main() {
    println!("Part 1: {}", part1("i14.txt"));
    println!("Part 2: {}", part2("i142.tst"));
}
