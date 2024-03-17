use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
  where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part1(filename: &str) -> i32 {

  fn parse_rule(rule: &str, counts: Vec<i32>) -> i32 {
    let rule_parts: Vec<&str> = rule.split(' ').collect();
    let range: Vec<&str> = rule_parts[0].split('-').collect();
    let low: i32 = range[0].parse().unwrap();
    let high: i32 = range[1].parse().unwrap();
    let character = rule_parts[1].chars().next().unwrap();
    let position = character as i32 - 'a' as i32;
    if counts[position as usize] < low || counts[position as usize] > high {
      return 0;
    }
    return 1;
  }

  fn parse_password(password: &str) -> Vec<i32> {
    let mut counts:Vec<i32>  = vec![0;26]; 
    for character in password.chars() {
      let position = character as i32 - 'a' as i32;
        counts[position as usize] += 1;
    }
    return counts
  }

  let mut valid = 0;
  if let Ok(lines) = read_lines(filename) {
    for line in lines {
      if let Ok(line) = line {
        let parts: Vec<&str> = line.split(':').collect();
        let password = &parts[1][1..];
        let counts = parse_password(password);
        let rule = &parts[0];
        valid += parse_rule(rule, counts);
      }
    }
  }
  return valid;
}
fn part2(filename: &str) -> i32 {

  fn parse_rule(rule: &str, password: &str) -> i32 {
    let rule_parts: Vec<&str> = rule.split(' ').collect();
    let range: Vec<&str> = rule_parts[0].split('-').collect();
    let mut low: i32 = range[0].parse().unwrap(); low -= 1;
    let mut high: i32 = range[1].parse().unwrap(); high -= 1;
    let character = rule_parts[1].chars().next().unwrap();
    let password_chars: Vec<char> = password.chars().collect();
    if high < password_chars.len() as i32 {
      if (password_chars[low as usize] == character) ^ (password_chars[high as usize] == character) {
        return 1;
      }
    }
    return 0;
  }

  let mut valid = 0;
  if let Ok(lines) = read_lines(filename) {
    for line in lines {
      if let Ok(line) = line {
        let parts: Vec<&str> = line.split(':').collect();
        let password = &parts[1][1..];
        let rule = &parts[0];
        valid += parse_rule(rule, password);
      }
    }
  }
  return valid;
}

fn main() {
  println!("Part 1: {}", part1("./inputday2.txt"));
  println!("Part 2: {}", part2("./inputday2.txt"));
}