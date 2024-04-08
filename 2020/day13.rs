mod aoc; use aoc::*;
use std::collections::HashSet;

fn part1(filename: &str) -> i32 {
    let mut lines: Vec<String> = Vec::new();
    for line in readlines(filename).expect("lines") {
        lines.push(line.expect("line"));
    }
    let start = lines[0].clone().parse::<i32>().unwrap();
    let sched = lines[1].split(",");
    let mut min = start as f32;
    let mut minbus = 0;
    for bus in sched {
        match bus {
            "x" => continue,
            _ => {
                let curr = start as f32 / bus.parse::<i32>().unwrap() as f32;
                let minutes = (curr.ceil() - curr) * bus.parse::<i32>().unwrap() as f32;
                if minutes < min {
                    min = minutes;
                    minbus = bus.parse::<i32>().unwrap()
                }
            }
        }
    }
    return min.ceil() as i32 * minbus;
}

/*
// Way too slow :(((
fn part2(filename: &str) -> i32 {
    let mut lines: Vec<String> = Vec::new();
    for line in readlines(filename).expect("lines") {
        lines.push(line.expect("line"));
    }
    let sched = lines[1].split(",");
    let mut idx = 0;
    let mut shift: Vec<i64> = Vec::new();
    let mut buses: Vec<i64> = Vec::new();
    for bus in sched {
        match bus {
            "x" => idx -= 1,
            _ => {
                shift.push(idx);
                buses.push(bus.parse::<i64>().unwrap());
                idx -=1;
            }
        }
    }
    println!("{:?}", shift);
    println!("{:?}", buses);
    let mut sievesz: i64 = 1;
    for bus in &buses[6..] {
        sievesz *= bus;
    }
    //let mut currs: i64 = 100001568000000;
    let mut currs: i64 = 0;
    let mut num: i64 = 0;
    let mut sieves: Vec<HashSet<i64>> = Vec::new();
    let mut sievea: HashSet<i64> = (0..sievesz).collect(); 
    for (s, b) in shift.iter().zip(buses.iter()) {
        let mut sieveb = HashSet::new();
        for i in 0..(sievesz/b+1) {
            num = i*b+s; 
            sieveb.insert(num);
        }
        sieves.push(sievea.intersection(&sieveb).cloned().collect::<HashSet<_>>());
    }

    /*
    println!("{:?}", sievea);
    for sieve in &sieves {
        println!("{:?}", sieve);
    }
    */
    for _ in 0..100000000 as i64 { 
        let mut sievea = sieves[0].clone();

        for sieve in &sieves[1..] {
            sievea = sievea.intersection(sieve).cloned().collect::<HashSet<_>>();
        }

        if !sievea.is_empty() {
            println!("{:?}", sievea);
            return 1;
        }
        
        for sieve in &mut sieves {
            *sieve = sieve.iter().map(|&x| x + sievesz).collect();
        }
        currs += sievesz;
        println!("{}", currs);
    }

    return 0;
}
*/

fn part2(filename: &str) -> i32 {
    let mut lines: Vec<String> = Vec::new();
    for line in readlines(filename).expect("lines") {
        lines.push(line.expect("line"));
    }
    let sched = lines[1].split(",");
    let mut idx = 0;
    let mut shift: Vec<i64> = Vec::new();
    let mut buses: Vec<i64> = Vec::new();
    for bus in sched {
        match bus {
            "x" => idx -= 1,
            _ => {
                shift.push(idx);
                buses.push(bus.parse::<i64>().unwrap());
                idx -=1;
            }
        }
    }
    println!("{:?}", shift);
    println!("{:?}", buses);
    let N: i64 = buses.iter().copied().reduce(|a, b| a*b).unwrap();
    println!("{}", N);

    return 0;
}
fn main() {
    println!("Part 1: {}", part1("i13.txt"));
    println!("Part 2: {}", part2("i13.txt"));
}
