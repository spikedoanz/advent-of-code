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

fn getlos(x: i32, y:i32, sx:i32, sy:i32) -> Vec<Vec<(i32,i32)>> {
    // Highly unoptimized, can form the arrays in order explicitly
    // Can also just not form these arrays in the first place and 
    //     verify states directly
    let mut horil: Vec<(i32,i32)> = Vec::new(); 
    let mut horir: Vec<(i32,i32)> = Vec::new(); 
    let mut vertl: Vec<(i32,i32)> = Vec::new(); 
    let mut vertr: Vec<(i32,i32)> = Vec::new(); 
    let mut swdnl: Vec<(i32,i32)> = Vec::new(); 
    let mut swdnr: Vec<(i32,i32)> = Vec::new(); 
    let mut swupl: Vec<(i32,i32)> = Vec::new(); 
    let mut swupr: Vec<(i32,i32)> = Vec::new(); 
    for ix in 0..sx {
        for iy in 0..sy { 
            if ix == x || iy == y { continue; }
            if iy < 0 || iy >= sy { continue; }
            if ix - x == iy - y {
                if ix < x   { swdnl.push( (ix, iy) ) }
                else        { swdnr.push( (ix, iy) ) } 
            }
            if x - ix == iy - y  {
                if ix < x   { swupl.push( (ix, iy) ) }
                else        { swupr.push( (ix, iy) ) } 
            }
        }
        if ix != x { 
            if ix < x   { horil.push( (ix , y) ) }
            else        { horir.push( (ix , y) ) } 
        }
        if ix != y { 
            if ix < y   { vertl.push( (x, ix) ) }
            else        { vertr.push( (x, ix) ) }
        }
    }

    let mut ret = vec![horil, horir, vertl, vertr, swdnl, swdnr, swupl, swupr];
    for vec in &mut ret {
        vec.sort_by_key(|coord| sqdist(coord, &(x,y)));
    }
    return ret;
}

fn sqdist(a: &(i32, i32), b: &(i32, i32)) -> i32 {
    let (xa, ya) = *a;
    let (xb, yb) = *b;
    return (xa - xb) * (xa - xb) + (ya - yb) * (ya - yb);
}


fn atmt2(x: usize, y: usize, state: &Vec<Vec<i32>>, losses: &Vec<Vec<Vec<Vec<(i32,i32)>>>>) -> i32 {
    let mut count = 0;
    let cs = state[y][x];
    if cs == 0 { return cs; }

    let los = &losses[x][y];

    for vec in los {
        for n in vec {
            match state[n.1 as usize][n.0 as usize] {
                2 => { count += 1; break }
                1 => { break; }
                _ => (),
            }
        }
    }
    if cs == 1 && count == 0 { return 2 } 
    if cs == 2 && count >= 5 { return 1 }
    return cs;
}

#[allow(dead_code)]
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
    }

    let mut count = 0;
    for arr in flip {
        for num in arr {
            if num == 2 { count += 1; }
        }
    }
    return count;
}

fn part2(filename: &str) -> i32 {
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
    let mut changed = true;
    let mut losses: Vec<Vec<Vec<Vec<(i32,i32)>>>> = Vec::new();
    for x in 0..sx {
        let mut inner: Vec<Vec<Vec<(i32,i32)>>> = Vec::new();
        for y in 0..sy {
            inner.push(getlos(x,y,sx,sy));
        }
        losses.push(inner);
    }

    while changed {
        changed = false;
        for y in 0..sy {
            for x in 0..sx {
                let new = atmt2(x as usize,y as usize, &flip, &losses);
                if new != flop[y as usize][x as usize] { 
                    changed = true;
                }
                flop[y as usize][x as usize] = new;
            }
        }
        flip = flop.clone();
        //prints(&flip);
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
    println!("{}", part2("i11.txt"));
}