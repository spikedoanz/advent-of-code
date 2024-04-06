mod aoc; use aoc::*;

fn getnei(x: i32, y:i32, sx: i32, sy: i32) -> Vec<(i32,i32)> {
    let vx = vec![x-1, x, x+1];
    let vy = vec![y-1, y, y+1];
    let mut ret: Vec<(i32,i32)> = Vec::new();
    for ix in vx {
        if ix < 0 || ix >= sx { continue; }
        for iy in &vy {
            if *iy < 0 || *iy >= sy { continue; }
            if ix == x && *iy == y { continue; }
            ret.push( (ix,*iy) );
        }
    }
    return ret;
}

fn atmt(x: i32, y: i32, state: &Vec<Vec<i32>>) -> i32 {
    let sy = state.len() as i32;
    let sx = state[0].len() as i32;
    let nei = getnei(x,y,sx,sy);
    let mut count = 0;
    let cs = state[y as usize][x as usize];
    if cs == 0 { return cs; }
    for n in nei {
        match state[n.1 as usize][n.0 as usize] {
            2 => count += 1,
            _ => (),
        }
    }
    if cs == 1 && count == 0 { return 2 } 
    if cs == 2 && count >= 4 { return 1 }
    return cs;
}

fn prints(state: &Vec<Vec<i32>>) {
    for arr in state {
        println!("{:?}", arr);
    }
    println!("------");
}

fn part1(filename: &str) -> i32 {
    let mut flip: Vec<Vec<i32>> = Vec::new();
    for (i, line) in readlines(filename).expect("lines").enumerate() {
        flip.push(Vec::new());
        for c in line.expect("line").chars() {
            match c {
                'L' => flip[i].push(1),
                '#' => flip[i].push(2),
                _ => flip[i].push(0),
            }
        }
    }
    let mut flop = flip.clone();
    let sy = flip.len() as i32;
    let sx = flip[0].len() as i32;
    println!("{} {}", sx, sy);
    let mut changed = true;
    while changed {
        changed = false;
        for y in 0..sy {
            for x in 0..sx {
                let new = atmt(x,y, &flip);
                if new != flop[y as usize][x as usize] { 
                    changed = true;
                }
                flop[y as usize][x as usize] = new;
            }
        }
        flip = flop.clone();
        prints(&flip);
    }

    let mut count = 0;
    for arr in flip {
        for num in arr {
            if num == 2 { count += 1; }
        }
    }
    return count;
}

fn main() {
    println!("{}", part1("i11.txt"));
}

