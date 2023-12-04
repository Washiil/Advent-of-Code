mod day01;
mod day02;
mod util;

use std::time::{Duration, Instant};
use util::{time_function};

fn main() {
    println!("Welcome to Advent of Code 2023! Happy Holidays!");

    let day1_part_1 = time_function(day01::part_1);
    println!("Day01_1 | {:?} | {}", day1_part_1.0, day1_part_1.1);

    let day1_part_2 = time_function(day01::part_2);
    println!("Day01_2 | {:?} | {}", day1_part_2.0, day1_part_2.1);
}
