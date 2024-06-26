mod aoc; use aoc::*;

fn apply(a: i64 , b: i64, op: i64) -> i64 {
    (1 - op) * (a + b) + op * (a * b) 
}

fn parse(line: &[char], i: &mut usize) -> i64 {
    let mut ret = 0;
    let mut op = 0; // 0 means +, 1 means *
    while *i < line.len() {
        let c = line[*i];
        match c {
            '0'..='9' => {
                ret = apply(ret, c as i64 - '0' as i64, op);
            },
            '*' => { op = 1 },
            '+' => { op = 0 },
            '(' => { 
                *i += 1;
                ret = apply(ret, parse(line, i), op);
            },
            ')' => {
                return ret;
            }
            _ => {},
        }
        *i += 1;
    }
    ret
}

fn part1(filename: &str) -> i64 {
    let mut ret: i64 = 0;
    for line in readlines(filename).expect("lines") {
        ret += parse(&line.expect("").chars().collect::<Vec<_>>(), &mut 0);
    }
    ret
}

fn parse2(line: &[char], i: &mut usize) -> i64 {
    let mut op = 0;
    let mut stack: Vec<i64> = vec![0];
    while *i < line.len() {
        let c = line[*i];
        match c {
            '0'..='9' => {
                let num = c as i64 - '0' as i64;
                if op == 0 {
                    let a = stack.pop().unwrap();
                    stack.push(num + a);
                } else {
                    stack.push(num);
                }
            },
            '*' => { op = 1 },
            '+' => { op = 0 },
            '(' => { 
                *i += 1;
                let num = parse2(line, i);
                if op == 0 {
                    let a = stack.pop().unwrap();
                    stack.push(num + a);
                } else {
                    stack.push(num);
                }
            },
            ')' => { break; }
            _ => {},
        }
        *i += 1;
    }

    let mut ret = 1;
    for num in stack {
        ret *= num;
    }
    ret
}

fn part2(filename: &str) -> i64 {
    let mut ret: i64 = 0;
    for line in readlines(filename).expect("lines") {
        let curr = parse2(&line.expect("").chars().collect::<Vec<_>>(), &mut 0);
        ret += curr;
    }
    ret
}


fn main() {
  println!("Part 1: {}", part1("i18.txt"));
  println!("Part 2: {}", part2("i18.txt"));
}
