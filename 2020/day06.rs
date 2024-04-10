use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use std::collections::HashSet;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where 
    P: AsRef<Path>,
{
    Ok(io::BufReader::new(File::open(filename)?).lines())
}

fn part1(filename: &str) -> i32 {
    let mut count = 0;
    let mut answers: HashSet<char> = HashSet::new();
    let lines = read_lines(filename).expect("lines");
    for line in lines {
        let line = line.expect("line");
        if line == "" {
            //println!("{:?} {}", answers, answers.len());
            count += answers.len() as i32;
            answers.clear();
        }
        for c in line.chars() {
            answers.insert(c);
        }
    }
    return count;
}

fn part2(filename: &str) -> i32 {
    let mut count = 0;
    let mut answers: HashSet<char> = ('a'..='z').collect();
    let mut curr: HashSet<char> = HashSet::new();
    let lines = read_lines(filename).expect("lines");
    for line in lines {
        let line = line.expect("line");
        if line == "" {
            count += answers.len() as i32;
            //println!("{:?} {}", answers, answers.len());
            answers = ('a'..='z').collect();
            continue;
        }
        for c in line.chars() {
            curr.insert(c);
        }
        answers = answers.intersection(&curr).copied().collect();
        curr.clear();
    }
    return count;
}

fn main() {
    println!("Part 1: {}", part1("inputday6.txt"));
    println!("Part 2: {}", part2("inputday6.txt"));
}
