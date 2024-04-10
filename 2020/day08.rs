mod aoc; use aoc::*;

fn parse_lines(mut lines: Vec<String>, check_complete: bool) -> i32 {
  let mut curr = 0;
  let count = lines.len() - 1;
  let mut ret = 0;

  while lines[curr] != "seen" && curr <= count { 
    let mut content = lines[curr].split(" ");
    let op = content.next();
    let num = content.next().expect("number");
    let tar = num[1..].parse::<i32>().unwrap();
    match op {
      Some("acc") => ret = if num.contains("+")  { ret + tar } else { ret - tar },
      Some("jmp") => curr = if num.contains("+") {curr + (tar - 1) as usize} else {curr - (tar + 1) as usize},
      _ => ret = ret,
    }
    if curr > count {
      return 0;
    }
    lines[curr] = "seen".to_string();
    curr += 1;
    if curr > count {
      return ret
    }
  }
  if curr != count && check_complete {
    return 0;
  } else {
    return ret;
  }
}

fn swap (idx: usize, lines: &mut Vec<String>) {
  if lines[idx].contains("nop") {
    lines[idx] = lines[idx].replace("nop", "jmp");
  }
  else if lines[idx].contains("jmp") {
    lines[idx] = lines[idx].replace("jmp", "nop");
  }
}

fn part1(filename: &str) -> i32 {
  let buffer = readlines(filename).expect("lines");
  let mut lines: Vec<String> = Vec::new();
  for line in buffer {
    lines.push(line.expect("line"));
  }
  return parse_lines(lines, false);
}

fn part2(filename: &str) -> i32 {
  let buffer = readlines(filename).expect("lines");
  let mut lines: Vec<String> = Vec::new();
  for line in buffer {
    lines.push(line.expect("line"));
  }
  let line_count = lines.len() - 1;
  let orig = lines.clone();

  let mut ret = 0;
  for idx in 0..line_count {
    swap(idx, &mut lines);
    ret = parse_lines(lines.clone(), true); 
    if ret != 0 {
      return ret;
    }
    else {
      lines = orig.clone();

    }
  }
  return ret;
}

fn main() {
  println!("Part 1: {}", part1("inputday8.txt"));
  println!("Part 2: {}", part2("inputday8.txt"));
}