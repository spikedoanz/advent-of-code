fn backward(pkey: u64) -> u64 {
    let mut val: u64 = 1;
    let mut loop_count = 0;
    loop {
        val = (val * 7) % 20201227;
        loop_count += 1;
        if val == pkey {
            return loop_count; 
        }
    }
}

fn encrypt(pkey: u64, loop_count: u64) -> u64 {
    let mut val = 1;
    for _ in 0..loop_count {
        val = (val * pkey) % 20201227;
    }
    return val;
}
    
fn main() {
    //let a:u64 = 5764801; let b:u64 = 17807724;
    let a:u64 = 10943862; let b:u64 = 12721030;
    println!("{}", encrypt(a, backward(b)));
}