pub use std::fs::File;
pub use std::path::Path;
pub use std::io::{self, BufRead};

pub fn readlines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where 
  P: AsRef<Path>,
{
  Ok(io::BufReader::new(File::open(filename)?).lines())
}

#[allow(dead_code)]
pub fn printvar<T: std::fmt::Debug>(value: T) {
  println!("{:?}", value);
}

#[allow(dead_code)]
pub fn printlines(lines: Vec<String>) {
  for line in lines {
    println!("{}", line);
  }
}