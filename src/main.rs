mod runner;
mod days;
mod utility;

use std::io::{Read, stdin};

use crate::runner::Runner;

fn main() {
    let days: Vec<Box<dyn Runner>> = vec![
        Box::new(days::day01::Day01),
        Box::new(days::day02::Day02),
        Box::new(days::day03::Day03),
        Box::new(days::day04::Day04)
    ];

    println!("------------------------------------------------------------------------------");
    println!("│ Day    │ Part 1         │ Duration 1     │ Part 2         │ Duration 2     │");
    println!("------------------------------------------------------------------------------");
    for (i, day) in days.iter().enumerate() {
        let benchmark = day.benchmark();
        println!("│ Day {:0>2} │ {: <14} │ {: <14?} │ {: <14} │ {: <14?} │", i + 1, &benchmark.0, &benchmark.2, &benchmark.1, &benchmark.3);
    }
    println!("------------------------------------------------------------------------------");

    println!("Press Enter to exit...");
    // let mut input = String::new();
    // stdin().read_line(&mut input).expect("Failed to read line");
}