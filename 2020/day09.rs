mod aoc; use aoc::*;

fn two_sum(nums: &[i64], curr: i64) -> i64{
  for num in nums {
    let check = curr - num;
    if nums.contains(&check) {
      return *num; 
    }
  }
  return -1;
}

fn part1(filename: &str) -> i64{
  let sep = 25;
  let lines = readlines(filename).expect("lines");
  let mut nums: Vec<i64> = Vec::new();
  for line in lines {
    let curr = line.expect("line").parse::<i64>().unwrap();
    nums.push(curr);
  }

  for num in sep..nums.len()  {
    let currnums = &nums[num-sep as usize .. num as usize];
    let curr = two_sum(currnums, nums[num]);
    if curr == -1 {
      return nums[num];
    }
  }
  return 0;
}

fn part2(ret: i64, filename: &str) -> i64{
  let lines = readlines(filename).expect("lines");
  let mut nums: Vec<i64> = Vec::new();
  for line in lines {
    let curr = line.expect("line").parse::<i64>().unwrap();
    nums.push(curr);
  }

  for size in 2..nums.len()  {
    for i in 0..nums.len() - size {
      let currnums = &nums[i as usize .. i+size as usize];
      let sum: i64 = currnums.iter().sum();
      if sum == ret {
        return currnums.iter().min().unwrap() + currnums.iter().max().unwrap();
      }
      continue;
    }
  }
  return 0;
}



fn main() {
  let file = "inputday9.txt";
  let p1 = part1(file);
  println!("Part 1: {}", p1);
  println!("Part 2: {}", part2(p1, file));
}