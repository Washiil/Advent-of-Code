use std::collections::HashMap;

use crate::runner::Runner;
use crate::utility::read_lines;

pub struct Day02;
// Game 1: 2 blue, 4 green; 7 blue, 1 red, 14 green; 5 blue, 13 green, 1 red; 1 red, 7 blue, 11 green
impl Runner for Day02 {
    fn part_one(&self) -> u32 {
        let mut total = 0;

        let colours: HashMap<&str, u32> = HashMap::from([
            ("red", 12),
            ("green", 13),
            ("blue", 14),
        ]);
        
        if let Ok(lines) = read_lines("./input/day02.txt") {
            for line in lines {
                if let Ok(value) = line {
                    let x = value.strip_prefix("Game ").unwrap();
                    let data: Vec<&str> = x.split(":").collect();

                    let match_num: u32 = data[0].parse().unwrap();
                    let mut valid = true;

                    let draws: Vec<&str> = data[1].split(";").collect();
                    
                    'outer: for draw in draws {
                        let dice: Vec<&str> = draw.split(",").collect();
                        for die in dice {
                            let parts: Vec<&str> = die.trim().split(" ").collect();

                            let count: u32 = parts[0].parse().unwrap();
                            let colour = parts[1];

                            match colours.get(colour) {
                                Some(col) => {
                                    if &count > col {
                                        valid = false;
                                        break 'outer;
                                    }
                                },
                                None => panic!("Incorrect Input")
                            }
                        }
                    }
                    if valid {
                        total += match_num;
                    }
                }
            }
        }
        return total;
    }

    fn part_two(&self) -> u32 {
        let mut total = 0;
        
        if let Ok(lines) = read_lines("./input/day02.txt") {
            for line in lines {
                if let Ok(value) = line {
                    let x = value.strip_prefix("Game ").unwrap();
                    let data: Vec<&str> = x.split(":").collect();

                    let mut colours: HashMap<&str, u32> = HashMap::from([
                        ("red", 0),
                        ("green", 0),
                        ("blue", 0),
                    ]);

                    let draws: Vec<&str> = data[1].split(";").collect();
                    
                    for draw in draws {
                        let dice: Vec<&str> = draw.split(",").collect();
                        for die in dice {
                            let parts: Vec<&str> = die.trim().split(" ").collect();

                            let count: u32 = parts[0].parse().unwrap();
                            let colour = parts[1];
                            
                            match colours.get_mut(colour) {
                                Some(col) => {
                                    if &count > col {
                                        *col = count
                                    }
                                },
                                None => panic!("Incorrect Input")
                            }
                        }
                    }
                    let product: u32 = colours.values().fold(1, |acc, x| acc * x);
                    total += product;
                }
            }
        }
        return total;
    }
}