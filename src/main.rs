mod runner;
mod days;
mod utility;

use crate::runner::Runner;

fn main() {

    let days: Vec<Box<dyn Runner>> = vec![
        Box::new(days::day01::Day01),
        Box::new(days::day02::Day02),
        Box::new(days::day03::Day03)
    ];

    for (i, day) in days.iter().enumerate() {
        let duration = day.benchmark();
        println!("Day {} | {:?}", i + 1, duration);
    }
}