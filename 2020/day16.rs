mod aoc; use aoc::*;
use std::collections::HashSet;

fn part1(filename: &str) -> i32 {
    let mut lines: Vec<String> = Vec::new();
    let mut valid: HashSet<i32> = HashSet::new();
    for line in readlines(filename).expect("lines") {
        lines.push(line.expect("line"));
    }

    for line in lines.clone() {
        if line == "" { break; }
        let parts: Vec<&str> = line.split(": ").collect();
        let ranges: Vec<&str> = parts[1].split(" or ").collect();
        let range0: Vec<&str> = ranges[0].split("-").collect();
        let range0_nums = (range0[0].parse::<i32>().unwrap(), range0[1].parse::<i32>().unwrap());
        let range1: Vec<&str> = ranges[1].split("-").collect();
        let range1_nums = (range1[0].parse::<i32>().unwrap(), range1[1].parse::<i32>().unwrap());

        for range in vec![range0_nums, range1_nums] {
            for num in range.0..=range.1 {
                valid.insert(num);
            }
        }
    }

    let mut start = false;
    let mut errrate = 0;
    for line in lines {
        if line == "nearby tickets:" { start = true; continue; }
        if start {
            let numstrings: Vec<&str> = line.split(",").collect();
            for string in numstrings {
                let num = string.parse::<i32>().unwrap();
                if !valid.contains(&num) {
                    errrate += num;
                }
            }
        }
    }
    
    return errrate;
}

fn part2(filename: &str) -> i64 {
    let mut lines: Vec<String> = Vec::new();
    let mut valid: HashSet<i32> = HashSet::new();
    let mut fields: Vec<(i32, i32, i32, i32)> = Vec::new();
    for line in readlines(filename).expect("lines") {
        lines.push(line.expect("line"));
    }

    for line in lines.clone() {
        if line == "" { break; }
        let parts: Vec<&str> = line.split(": ").collect();
        let ranges: Vec<&str> = parts[1].split(" or ").collect();
        let range0: Vec<&str> = ranges[0].split("-").collect();
        let range0_nums = (range0[0].parse::<i32>().unwrap(), range0[1].parse::<i32>().unwrap());
        let range1: Vec<&str> = ranges[1].split("-").collect();
        let range1_nums = (range1[0].parse::<i32>().unwrap(), range1[1].parse::<i32>().unwrap());

        fields.push( (range0_nums.0, range0_nums.1, range1_nums.0, range1_nums.1) );

        for range in vec![range0_nums, range1_nums] {
            for num in range.0..=range.1 {
                valid.insert(num);
            }
        }
    }

    let mut start = false;
    let mut invalids: Vec<usize> = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if line == "nearby tickets:" { start = true; continue; }
        if start {
            let numstrings: Vec<&str> = line.split(",").collect();
            for string in numstrings {
                let num = string.parse::<i32>().unwrap();
                if !valid.contains(&num) {
                    invalids.push(i);
                }
            }
        }
    }

    let mut shift = 0;
    for i in invalids {
        lines.remove(i-shift);
        shift += 1;
    }

    let mut fieldnums: Vec<HashSet<i32>> = Vec::new();
    for field in &fields {
        let mut currfield: HashSet<i32> = HashSet::new();
        for num in field.0..=field.1 {
            currfield.insert(num);
        }
        for num in field.2..=field.3 {
            currfield.insert(num);
        }
        fieldnums.push(currfield);
    }

    start = false;
    let mut lasts: Vec<HashSet<i32>> = Vec::new();
    let numfields = fieldnums.len();
    for line in lines.clone() {
        if line == "nearby tickets:" { start = true; continue; }
        if start {
            let numstrings: Vec<&str> = line.split(",").collect();
            let mut valid_order: Vec<Vec<i32>> = Vec::new();
            for (_i, string) in numstrings.iter().enumerate() {
                let mut curr_pos: Vec<i32> = Vec::new();
                let num = string.parse::<i32>().unwrap();
                for (j, field) in fieldnums.iter().enumerate() {
                    if field.contains(&num) {
                        curr_pos.push(j as i32);
                    }
                }
                valid_order.push(curr_pos);
            }
            /*
            for thing in valid_order {
                println!("{:?}", thing);
            }
            */
            for (i,order) in valid_order.iter().enumerate() {
                let curr: HashSet<i32> =order.clone().into_iter().collect();
                if lasts.len() <= i {
                    lasts.push(curr);
                }
                else {
                    lasts[i] = lasts[i].intersection(&curr).copied().collect();
                }
            }
        }
    }
    start = false;
    let mut ticket_nums: Vec<i64> = Vec::new();
    for line in &lines {
        if line == "your ticket:" { start = true; continue; }
        if start {
            let numstrings: Vec<&str> = line.split(",").collect();
            for num in numstrings {
                ticket_nums.push(num.parse::<i64>().unwrap());
            }
            break;
        }
    }

    let mut seen: HashSet<i32> = HashSet::new();
    let mut ret: i64 = 1;
    for _ in 0..numfields {
        for (i, order) in lasts.iter().enumerate() {
            let diff: HashSet<&i32> = order.difference(&seen).collect();
            if diff.len() == 1 {
                let hit = diff.iter().next().unwrap();
                if **hit < 6 {
                    ret *= ticket_nums[i];
                }
                seen.insert(**hit);
            }
        }
    }

    
    return ret;
}

fn main() {
    println!("Part 1: {}", part1("i16.txt"));
    println!("Part 2: {}", part2("i16.txt"));
}