use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

fn two_sum(filename: &str) -> i32 {
    if let Ok(lines) = read_lines(filename) {
        let mut set = HashSet::new();
            for line in lines {
                    if let Ok(ip) = line {
                        let num = ip.parse::<i32>().unwrap();            
                        if !set.contains(&num) {
                            set.insert(num);
                        }
                        if set.contains(&(2020-num)) {
                            return num*(2020-num)
                        }
                    }
                }
    }
    return -1;
}

fn three_sum(filename: &str) -> i32 {
    let mut sums = Vec::new();
    let mut nums = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines { // Make all the sums that can be make with 1 fewer of the numbers
            if let Ok(ip) = line {
                let num = ip.parse::<i32>().unwrap();            
                sums.push(2020-num);
                nums.push(num);
            }
        }
    }
    for sum in sums {
        let mut set = HashSet::new();
        for num in &nums {
            if !set.contains(&num) {
                set.insert(num);
            }
            if set.contains(&(sum-num)) {
                return num*(sum-num)*(2020-sum)
            }
        }
    }
    
    return -1;
}

fn main() {
    println!("Part 1: {}", two_sum("./inputday1.txt"));
    println!("Part 2: {}", three_sum("./inputday1.txt"));

}