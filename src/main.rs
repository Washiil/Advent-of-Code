mod runner;
mod days;
mod utility;

use crate::runner::Runner;

fn main() {
    let days: Vec<Box<dyn Runner>> = vec![
        Box::new(days::day01::Day01),
        Box::new(days::day02::Day02),
        Box::new(days::day03::Day03),
        Box::new(days::day04::Day04)
    ];

    println!("---------------------------------------------------");
    println!("Day    | Value 1        | Value 2        | Duration");
    println!("---------------------------------------------------");
    for (i, day) in days.iter().enumerate() {
        let duration = day.benchmark();
        println!("Day {:0>2} | {: <14} | {: <14} | {:?}", i + 1, &duration.0, &duration.1, &duration.2);
    }
}