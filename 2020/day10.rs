mod aoc; use aoc::*;

fn part1(filename: &str) -> i32 {
    let mut adps: Vec<i32> = Vec::new();
    for line in readlines(filename).expect("lines") {
        let curr = line.expect("line").parse::<i32>().unwrap();
        adps.push(curr);
    }
    adps.sort();
    let mut ones = 0;
    let mut threes = 1;
    let mut curr = 0;
    for adp in adps {
        match adp - curr {
            3 => threes += 1,
            1 => ones += 1,
            _ => continue,
        }
        curr = adp;
    }
    //println!("{} {}", ones, threes);
    return ones*threes;
}

fn permute(len: i64) -> i64 {
    let n = len - 1;
    return (1 + n) * n / 2 + 1;
}

fn part2(filename: &str) -> i64 {
    let mut adps: Vec<i32> = Vec::new();
    let mut curr = 0;
    adps.push(curr);
    for line in readlines(filename).expect("lines") {
        curr = line.expect("line").parse::<i32>().unwrap();
        adps.push(curr);
    }
    adps.push(curr + 3);
    adps.sort();
    let mut total = 1;
    let mut curr = 0;
    let mut len = 0;
    for adp in adps {
        match adp - curr {
            3 => { 
                total *= permute(len);
                println!("{} {}", permute(len), len);
                len = 0;
            }
            1 => len += 1,
            _ => continue,
        }
        curr = adp;
    }
    return total;
}



fn main() {
    println!("{}", part1("inputday10.test"));
    println!("{}", part2("inputday10.txt"));
}

