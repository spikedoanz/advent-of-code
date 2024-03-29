use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
  where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part1(filename: &str) -> i32 {   
  let mut count = 0;
  let mut curr = 0;
  let mut down = 0;
  if let Ok(lines) = read_lines(filename) {
    for line in lines {
      if let Ok(line) = line {
        if down % 2 == 1 {
          down += 1;
          continue;
        }
        if line.chars().nth(curr).unwrap() == '#' {
          count += 1;
        }
        //println!("{} {}", line, line.chars().nth(curr).unwrap());
        curr = (curr + 1) % line.len(); 
        down += 1;
      }
    }
  }
  return count;
}

fn main() {
  println!("Part 1: {}", part1("./inputday3.txt"));
}
