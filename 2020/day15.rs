use std::collections::HashMap;

fn game(mut list: Vec<i64>, ith: i64) -> i64 {
    let mut curr = list.len() as i64 - 1;
    let mut seen: HashMap<i64, i64> = HashMap::new();
    for (i, num) in list[..list.len()-1].iter().enumerate() {
        seen.insert(*num, i as i64);
    }
    let turns = ith-curr-1;
    let mut latest: i64 = *list.last().unwrap();
    for _i in 0..turns {
        if !seen.contains_key(&latest) {
            seen.insert(latest, curr);
            latest = 0;
        }
        else {
            let last = curr - seen[&latest];
            seen.insert(latest, curr);
            latest = last;
        }
        curr += 1;
    }
    return latest;
}

    
fn main() {
    let list: Vec<i64> = vec![1,0,16,5,17,4];
    println!("{}", game(list.clone(), 2020));
    println!("{}", game(list.clone(), 30000000));
}