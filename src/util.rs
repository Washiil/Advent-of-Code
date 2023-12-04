use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::{Instant, Duration};

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn time_function<F, T>(func: F) -> (Duration, T)
where
    F: Fn() -> T,
{
    let start = Instant::now();
    let result = func();
    let len = start.elapsed();
    (len, result)
}