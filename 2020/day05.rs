use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where 
  P: AsRef<Path>,
{
  Ok(io::BufReader::new(File::open(filename)?).lines())
}

fn partition(line: &str) -> i32 {
  let mut top     = 0; 
  let mut bottom  = 127;
  let mut right   = 0;
  let mut left    = 7;
  for c in line.chars() {
    match c {
      'F' => {bottom  = bottom - (bottom - top + 1) / 2;}
      'B' => {top     = top + (bottom - top + 1) / 2;}
      'L' => {left    = left - (left - right + 1) /2;}
      'R' => {right   = right + (left - right + 1) /2;}
      _   => continue,
    }
    //println!("{} ({} {}) ({} {})", c, top, bottom, right, left);
  }
  return top * 8 + right;
}


fn part1(filename: &str) -> i32 {
  let mut highest = 0;
  let lines = read_lines(filename).expect("lines");
  for line in lines {
    let line = line.expect("line");
    let curr = partition(&line);
    if curr > highest {
      highest = curr;
    }
    //println!("{} {}", line, curr);
  }
  return highest;
}

fn part2(filename: &str) -> i32 {
  let mut id = 0;
  let lines = read_lines(filename).expect("lines");
  let numseats = read_lines(filename).expect("lines").count();
  let mut seats:Vec<i32> = Vec::with_capacity(numseats);
  for line in lines {
    let line = line.expect("line");
    let curr = partition(&line);
    seats.push(curr);
  }
  seats.sort();
  let mut prev = seats[0];
  for seat in seats.into_iter() {
    if seat != prev + 1 && seat != prev {
      id = seat - 1;
    }
    prev = seat;
  }

  return id;
}

fn main() {
  println!("Part 1: {}", part1("inputday5.txt"));
  println!("Part 2: {}", part2("inputday5.txt"));
}
