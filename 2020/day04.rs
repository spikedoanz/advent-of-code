use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use std::collections::HashSet;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
  where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn verify_title(cont: HashSet<String>) -> bool {
  
  let mut spec: HashSet<String> = HashSet::from([]);
  for thing in ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"] {
    spec.insert(thing.to_string()); 
  }

  //println!("{:?}", spec);
  //println!("{:?} {}", cont, spec.is_subset(&cont));
  return spec.is_subset(&cont)
}

fn verify_byr(cont: String) -> bool {
  match cont.parse::<i32>() {
    Err(_) => return false,
    Ok(num) => return num >= 1920 && num <= 2002,
  }
}

fn verify_iyr(cont: String) -> bool {
  match cont.parse::<i32>() {
    Err(_) => return false,
    Ok(num) => return num >= 2010 && num <= 2020,
  }
}

fn verify_eyr(cont: String) -> bool {
  match cont.parse::<i32>() {
    Err(_) => return false,
    Ok(num) => return num >= 2020 && num <= 2030,
  }
}

fn verify_hgt(cont: String) -> bool {
  let num_str = &cont[..cont.len() -2];
  let unit = &cont[cont.len() -2..]; 
  if unit == "cm" { // 150..193
    match num_str.parse::<i32>() {
      Err(_) => return false,
      Ok(num) => return num >= 150 && num <= 193,
    }
  } 
  else if unit == "in" { // 59..76  
    match num_str.parse::<i32>() {
      Err(_) => return false,
      Ok(num) => return num >= 59 && num <= 76,
    }
  } 
  return false;
}

fn verify_hcl(cont: String) -> bool {
  fn is_hex(color: String) -> bool {
    color.chars().all(|c| match c {
      '0'..='9' | 'a'..='f' => true,
      _ => false,
    })
  }
      
  if cont.chars().next().unwrap() == '#' { 
    let color = &cont[1..];
    if color.len() == 6 {
      return is_hex(color.to_string());
    }
  }
  return false;
}

fn verify_ecl(cont: String) -> bool {
  let mut spec: HashSet<String> = HashSet::from([]);
  for thing in ["amb", "blu", "brn","gry", "grn", "hzl", "oth"] {
    spec.insert(thing.to_string()); 
  }
  return spec.contains(&cont)
}

fn verify_pid(cont: String) -> bool {
  if cont.len() == 9 {
    return cont.chars().all(|c| match c {
      '0'..='9' => true,
      _ => false,        
    })
  }
  return false;
}
  
fn verify_content(field: String) -> bool {
  if &field[..3] == "byr" {
    return verify_byr(field[4..].to_string())
  } else if &field[..3] == "iyr" {
    return verify_iyr(field[4..].to_string()) 
  } else if &field[..3] == "eyr" {
    return verify_eyr(field[4..].to_string()) 
  } else if &field[..3] == "hgt" {
    //println!("{} {}", field, verify_hgt(field[4..].to_string()));
    return verify_hgt(field[4..].to_string()) 
  } else if &field[..3] == "hcl" {
    //println!("{} {}", field, verify_hcl(field[4..].to_string()));
    return verify_hcl(field[4..].to_string()) 
  } else if &field[..3] == "ecl" {
    //println!("{} {}", field, verify_ecl(field[4..].to_string()));
    return verify_ecl(field[4..].to_string()) 
  } else if &field[..3] == "pid" {
    //println!("{} {}", field, verify_pid(field[4..].to_string()));
    return verify_pid(field[4..].to_string()) 
  } else if &field[0..3] == "cid" {
    return true;
  }
  return false;
}

fn part1(filename: &str) -> i32 {
  let mut count = 0;
  let mut cont: HashSet<String> = HashSet::from([]);

  let lines = read_lines(filename).expect("lines");
  for line in lines {
    let line = line.expect("line"); 
    if line == "" {
      count += verify_title(cont.clone()) as i32;
      cont.clear();
      continue;
    }
    let fields = line.split(' ');
    for field in fields { 
      cont.insert(field[..3].to_string());
    }
  }  
  return count;
}

fn part2(filename: &str) -> i32 {
  let mut count = 0;
  let mut cont: HashSet<String> = HashSet::from([]);

  let lines = read_lines(filename).expect("lines");
  for line in lines {
    let line = line.expect("line"); 
    if line == "" {
      count += verify_title(cont.clone()) as i32;
      cont.clear();
      continue;
    }
    let fields = line.split(' ');
    for field in fields { 
      if verify_content(field.to_string()) {
        cont.insert(field[..3].to_string());
      }
    }
  }  
  return count;
}
fn main() {
  println!("Part 1: {}", part1("./inputday4.txt"));
  println!("Part 2: {}", part2("./inputday4.txt"));
}
