mod aoc; use aoc::*;

fn count_cube(cube: Vec<Vec<Vec<i32>>>) -> i32 {
    let mut ret = 0;
    for slice in cube {
        for line in slice {
            for state in line {
                ret += state;
            }
        }
    }
    return ret
}

fn atmt(count: i32, state: i32) -> i32 {
    if state == 1 {
        if count == 2 || count == 3 {
            return 1;
        }
        else {
            return 0;
        }
    }
    if count == 3 {
        return 1;
    }
    return 0;
}

fn get_nei(idx: Vec<i32>, dims: Vec<i32>)
    -> Vec<Vec<usize>> {
    let x = idx[0]; let y = idx[1]; let z = idx[2];
    let dx = dims[0]; let dy = dims[1]; let dz = dims[2];
    let vx = vec![x-1, x, x+1];
    let vy = vec![y-1, y, y+1];
    let vz = vec![z-1, z, z+1];
    let mut ret: Vec<Vec<usize>> = Vec::new();
     
    for ix in &vx {
        if *ix < 0 || *ix >= dx { continue; }
        for iy in &vy {
            if *iy < 0 || *iy >= dy { continue; }
            for iz in &vz {
                if *iz < 0 || *iz >= dz { continue; }
                if *ix == x && *iy == y && *iz == z {
                    continue;
                }
                ret.push(vec![*ix as usize, *iy as usize, *iz as usize]);
                //println!("{} {} {}", *ix, *iy, *iz);
            }
        }
    }
                
    return ret;
}

fn outside(idx: Vec<i32>, dims: Vec<i32>) -> bool {
    if idx[0] < 0 || idx[0] >= dims[0] { return true; }
    if idx[1] < 0 || idx[1] >= dims[1] { return true; }
    if idx[2] < 0 || idx[2] >= dims[2] { return true; }
    return false;
}

fn step(flip: Vec<Vec<Vec<i32>>>) 
    -> Vec<Vec<Vec<i32>>> {
    let zdim = flip.len() as i32;
    let ydim = flip[0].len() as i32;
    let xdim = flip[0][0].len() as i32;
    let mut flop: Vec<Vec<Vec<i32>>> = Vec::new();
    
    let mut state;
    let mut nstate;

    for z in -1..zdim+1 {
        let mut cslice = Vec::new();
        for y in -1..ydim+1 {
            let mut cline = Vec::new();
            for x in -1..xdim+1 {
                if outside(vec![x,y,z], vec![xdim, ydim, zdim]) {
                    state = 0;
                }
                else {
                    state = flip[z as usize][y as usize][x as usize];
                }

                let vcs = get_nei(vec![x,y,z], vec![xdim, ydim, zdim]);
                let mut count = 0;
                for c in vcs {
                    if flip[c[2]][c[1]][c[0]] == 1 { count += 1; }
                }
                nstate = atmt(count, state);
                cline.push(nstate);
            }
            cslice.push(cline);
        }
        flop.push(cslice);
    }
    
    return flop;
}

fn part1(filename: &str) -> i32 {
    let mut lines: Vec<String> = Vec::new();
    for line in readlines(filename).expect("lines") {
        lines.push(line.expect("line"));
    }
    // Load in initial state
    let mut flip: Vec<Vec<Vec<i32>>> = Vec::new();
    let mut cslice: Vec<Vec<i32>> = Vec::new();

    for line in lines {
        let mut cline: Vec<i32> = Vec::new();  
        for c in line.chars() {
            if c == '.' {
                cline.push(0);
            }
            else {
                cline.push(1);
            }
        }
        cslice.push(cline);
    }
    flip.push(cslice);

    for _ in 0..6 {
        flip = step(flip);
    }
    return count_cube(flip.clone());
}


fn count_hcube(hcube: Vec<Vec<Vec<Vec<i32>>>>) -> i32 {
    let mut ret = 0;
    for cube in hcube {
        for slice in cube {
            for line in slice {
                for state in line {
                    ret += state;
                }
            }
        }
    }
    return ret
}

fn get_nei2(idx: Vec<i32>, dims: Vec<i32>)
    -> Vec<Vec<usize>> {
    let x = idx[0]; let y = idx[1]; let z = idx[2]; let w = idx[3];
    let dx = dims[0]; let dy = dims[1]; let dz = dims[2]; let dw = dims[3];
    let vx = vec![x-1, x, x+1];
    let vy = vec![y-1, y, y+1];
    let vz = vec![z-1, z, z+1];
    let vw = vec![w-1, w, w+1];

    let mut ret: Vec<Vec<usize>> = Vec::new();

    for iw in &vw {
        if *iw < 0 || *iw >= dw { continue; }
        for ix in &vx {
            if *ix < 0 || *ix >= dx { continue; }
            for iy in &vy {
                if *iy < 0 || *iy >= dy { continue; }
                for iz in &vz {
                    if *iz < 0 || *iz >= dz { continue; }
                    if *ix == x && *iy == y && *iz == z && *iw == w {
                        continue;
                    }
                    ret.push(vec![*ix as usize, *iy as usize, *iz as usize, *iw as usize]);
                }
            }
        }
    }
    return ret;
}

fn outside2(idx: Vec<i32>, dims: Vec<i32>) -> bool {
    if idx[0] < 0 || idx[0] >= dims[0] { return true; }
    if idx[1] < 0 || idx[1] >= dims[1] { return true; }
    if idx[2] < 0 || idx[2] >= dims[2] { return true; }
    if idx[3] < 0 || idx[3] >= dims[3] { return true; }
    return false;
}

fn step2(flip: Vec<Vec<Vec<Vec<i32>>>>) 
    -> Vec<Vec<Vec<Vec<i32>>>> {
    let wdim = flip.len() as i32;
    let zdim = flip[0].len() as i32;
    let ydim = flip[0][0].len() as i32;
    let xdim = flip[0][0][0].len() as i32;

    let mut flop: Vec<Vec<Vec<Vec<i32>>>> = Vec::new();
    
    let mut state;
    let mut nstate;

    for w in -1..wdim+1 {
        let mut ccube = Vec::new();
        for z in -1..zdim+1 {
            let mut cslice = Vec::new();
            for y in -1..ydim+1 {
                let mut cline = Vec::new();
                for x in -1..xdim+1 {
                    if outside2(vec![x,y,z,w], vec![xdim, ydim, zdim, wdim]) {
                        state = 0;
                    }
                    else {
                        state = flip[w as usize][z as usize][y as usize][x as usize];
                    }

                    let vcs = get_nei2(vec![x,y,z,w], vec![xdim, ydim, zdim, wdim]);
                    let mut count = 0;
                    for c in vcs {
                        if flip[c[3]][c[2]][c[1]][c[0]] == 1 { count += 1; }
                    }
                    nstate = atmt(count, state);
                    cline.push(nstate);
                }
                cslice.push(cline);
            }
            ccube.push(cslice);
        }
        flop.push(ccube);
    }
    return flop;
}

fn part2(filename: &str) -> i32 {
    let mut lines: Vec<String> = Vec::new();
    for line in readlines(filename).expect("lines") {
        lines.push(line.expect("line"));
    }
    // Load in initial state
    let mut flip: Vec<Vec<Vec<Vec<i32>>>> = Vec::new();
    let mut cube = Vec::new();
    let mut cslice: Vec<Vec<i32>> = Vec::new();

    for line in lines {
        let mut cline: Vec<i32> = Vec::new();  
        for c in line.chars() {
            if c == '.' {
                cline.push(0);
            }
            else {
                cline.push(1);
            }
        }
        cslice.push(cline);
    }
    cube.push(cslice);
    flip.push(cube);

    for _ in 0..6 {
        flip = step2(flip);
    }
    return count_hcube(flip.clone());
}

fn main() {
    println!("Part 1: {}", part1("i17.txt"));
    println!("Part 2: {}", part2("i17.txt"));
}
