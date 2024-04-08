mod aoc; use aoc::*;
use std::collections::HashMap;


fn turn(dir: &mut char, hand: char, degree: i32) {
    let right = vec!['N', 'E', 'S', 'W'];
    let left = vec!['N', 'W', 'S', 'E'];
    let turns = (degree / 90) as usize;
    match hand {
        'R' => {
            let pos = right.iter().position(|&r| r == *dir).unwrap();

            *dir = right[(pos+turns) % right.len()];
        }

        'L' => {
            let pos = left.iter().position(|&r| r == *dir).unwrap();
            *dir = left[(pos+turns) % left.len()];
        }
        _   => (),
    }
}

fn part1(filename: &str) -> i32 {
    let mut dir = 'E';
    let mut pos: HashMap<char, i32> = HashMap::new();
    pos.insert('N',0);
    pos.insert('E',0);
    pos.insert('W',0);
    pos.insert('S',0);
    for line in readlines(filename).expect("lines") {
        let line = line.unwrap();
        let instruction = line.chars().next().unwrap();
        let num = line[1..].parse::<i32>().unwrap();

        match instruction {
            'F' => *pos.entry(dir).or_insert(0) += num,
            'N' => *pos.entry('N').or_insert(0) += num,
            'E' => *pos.entry('E').or_insert(0) += num,
            'W' => *pos.entry('W').or_insert(0) += num,
            'S' => *pos.entry('S').or_insert(0) += num,
            'L' => turn(&mut dir, instruction, num),
            'R' => turn(&mut dir, instruction, num),
            _   => (),
        }
    }
    let north_south = pos.get(&'N').unwrap() - pos.get(&'S').unwrap();
    let east_west = pos.get(&'E').unwrap() - pos.get(&'W').unwrap();
    let result = north_south.abs() + east_west.abs();
    return result;
}


fn follow(pos: &mut HashMap<char, i32>, way: &mut HashMap<char, i32>, num: i32) {
    let n = way[&'N'] * num;
    let e = way[&'E'] * num;
    let w = way[&'W'] * num;
    let s = way[&'S'] * num;
    //println!("{} {} {} {}", n,e,w,s);
    *pos.entry('N').or_insert(0) += n;
    *pos.entry('E').or_insert(0) += e;
    *pos.entry('W').or_insert(0) += w;
    *pos.entry('S').or_insert(0) += s;
}

fn turnway(way: &mut HashMap<char, i32>, hand: char, degree: i32) {
    let mut pos = vec![way[&'N'],way[&'E'],way[&'S'],way[&'W']];
    let turns = (degree / 90) as usize;
    match hand {
        'R' => {
            for _ in 0..turns {
                let temp = pos[0];
                pos[0] = pos[3];
                pos[3] = pos[2];
                pos[2] = pos[1];
                pos[1] = temp;
            }
        },
        'L' => {
            for _ in 0..turns {
                let temp = pos[0];
                pos[0] = pos[1];
                pos[1] = pos[2];
                pos[2] = pos[3];
                pos[3] = temp;
            }
        }
        _ => (),
    }
    for (i, dir) in ['N', 'E', 'S', 'W'].iter().enumerate() {
        way.insert(*dir, pos[i]);
    }
}

fn part2(filename: &str) -> i32 {
    let mut dir = 'E';
    let mut pos: HashMap<char, i32> = HashMap::new();
    pos.insert('N',0);
    pos.insert('E',0);
    pos.insert('W',0);
    pos.insert('S',0);

    let mut way: HashMap<char, i32> = HashMap::new();
    way.insert('N',1);
    way.insert('E',10);
    way.insert('W',0);
    way.insert('S',0);

    for line in readlines(filename).expect("lines") {
        let line = line.unwrap();
        let instruction = line.chars().next().unwrap();
        let num = line[1..].parse::<i32>().unwrap();
        
        match instruction {
            'F' => follow(&mut pos, &mut way, num),
            'N' => *way.entry('N').or_insert(0) += num,
            'E' => *way.entry('E').or_insert(0) += num,
            'W' => *way.entry('W').or_insert(0) += num,
            'S' => *way.entry('S').or_insert(0) += num,
            'L' => turnway(&mut way, instruction, num),
            'R' => turnway(&mut way, instruction, num),
            _   => (),
        }
    }
    let north_south = pos.get(&'N').unwrap() - pos.get(&'S').unwrap();
    let east_west = pos.get(&'E').unwrap() - pos.get(&'W').unwrap();
    let result = north_south.abs() + east_west.abs();
    return result;
}

fn main() {
    println!("{}", part1("i12.txt"));
    println!("{}", part2("i12.txt"));
}