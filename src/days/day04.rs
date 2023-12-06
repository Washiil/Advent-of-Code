use std::collections::HashMap;

use crate::{runner::Runner, utility::read_lines};

pub struct Day04;

impl Runner for Day04 {
    fn part_one(&self) -> u32 {
        let mut total = 0;
        if let Ok(lines) = read_lines("./input/day04.txt") {
            for line in lines {
                if let Ok(value) = line {
                    let mut points = 0;

                    let data: Vec<&str> = value.split(": ").collect();
                    let nums = data[1].replace("  ", " ");

                    let nums: Vec<&str> = nums
                        .split(" | ")
                        .collect();
                    
                    let winners: Vec<&str> = nums[0]
                        .split(" ")
                        .collect();

                    let numbers: Vec<&str> = nums[1]
                        .split(" ")
                        .collect();

                    for num in &numbers {
                        if winners.contains(num) {
                            if points == 0 {
                                points = 1;
                            }
                            else {
                                points *= 2;
                            }
                        }
                    }
                    total += points;
                } 
            }
        }
        return total;
    }

    fn part_two(&self) -> u32 {
        return 0;
        let mut total = 0;
        
        let mut data: HashMap<String, u32> = HashMap::new();
        if let Ok(lines) = read_lines("./input/day04.txt") {
            for line in lines {
                if let Ok(value) = line {
                    let mut count = 0;

                    let card: Vec<&str> = value.split(": ").collect();
                    let first_half = card[0].replace("  ", " ");
                    let game_number: Vec<&str> = first_half.split(" ").collect();
                    let game_number: u32 = game_number[1].parse().unwrap();
                    println!("{}", game_number);
                    let nums = card[1].replace("  ", " ");

                    let nums: Vec<&str> = nums
                        .split(" | ")
                        .collect();
                    
                    let winners: Vec<&str> = nums[0]
                        .split(" ")
                        .collect();

                    let numbers: Vec<&str> = nums[1]
                        .split(" ")
                        .collect();

                    for num in &numbers {
                        if winners.contains(num) {
                                count += 1;
                            }
                        }
                    data.insert(value, count);
                }
            }
        }

        fn score_card() -> u32 {
            0
        }

        total
    }
}