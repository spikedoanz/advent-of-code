mod aoc; use aoc::*;

fn part1(filename: &str) -> i32 {
    let mut lines: Vec<String> = Vec::new();
    for line in readlines(filename).expect("lines") {
        lines.push(line.expect("line"));
    }
    let start = lines[0].clone().parse::<i32>().unwrap();
    let sched = lines[1].split(",");
    let mut min = start as f32;
    let mut minbus = 0;
    for bus in sched {
        match bus {
            "x" => continue,
            _ => {
                let curr = start as f32 / bus.parse::<i32>().unwrap() as f32;
                let minutes = (curr.ceil() - curr) * bus.parse::<i32>().unwrap() as f32;
                if minutes < min {
                    min = minutes;
                    minbus = bus.parse::<i32>().unwrap()
                }
            }
        }
    }
    return min.ceil() as i32 * minbus;
}


fn shiftedlcm(a: i64, b: i64, a_s: i64, b_s: i64, shift: i64) -> i64 {
    let mut n = 1;
    loop {
        if (a - b + shift + a_s * n ) % b_s == 0 {
            return a + a_s * n;
        }
        n += 1;
    }
}

fn part2(filename: &str) -> i64 {
    let mut lines: Vec<String> = Vec::new();
    for line in readlines(filename).expect("lines") {
        lines.push(line.expect("line"));
    }
    let sched = lines[1].split(",");
    let mut idx = 0;
    let mut shift: Vec<i64> = Vec::new();
    let mut buses: Vec<i64> = Vec::new();
    for bus in sched {
        match bus {
            "x" => idx += 1,
            _ => {
                shift.push(idx);
                buses.push(bus.parse::<i64>().unwrap());
                idx +=1;
            }
        }
    }
    let mut range = buses[0];
    let mut ret: i64 = 0;
    while buses.len() > 1 {
        shift.remove(0);
        ret = shiftedlcm(buses[0], buses[1], range, buses[1], shift[0]);
        range *= buses[1];
        buses.remove(0);
        buses[0] = ret;
    }
    return ret;
}

fn main() {
    println!("Part 1: {}", part1("i13.txt"));
    println!("Part 2: {}", part2("i13.txt"));
}
