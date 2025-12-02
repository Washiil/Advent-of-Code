use core::panic;
use std::{i32, path::PathBuf};

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    /*
    Because the dial is a circle, turning the dial left from 0 one click makes it point at 99. 
    Similarly, turning the dial right from 99 one click makes it point at 0.

    So, if the dial were pointing at 5, a rotation of L10 would cause it to point at 95. 
    After that, a rotation of R5 could cause it to point at 0.

    The number of times the dial is left pointing at 0 after any rotation in the sequence.

    The dial starts by pointing at 50.
     */
    let lines = input.split("\n").map(|l| l.chars());

    let mut dial: i16 = 50;
    let mut zeros = 0;

    for mut input in lines {
        let direction = input.next();

        // 1. Collect the digits into a container that OWNS the data (String)
        let dist_str: String = input
            .take_while(|x| x.is_numeric())
            .collect(); // Rust knows to make this a String based on the next line

        // 2. Parse the owned String
        let dist = dist_str.parse::<i16>().unwrap();
        match direction {
            Some('L') => dial = (dial - dist) % 100,
            Some('R') => dial = (dial + dist) % 100,
            Some(_) => panic!("Unreachable"),
            None => panic!("Unreachable"),
        }

        if dial == 0 {
            zeros += 1;
        }
    }

    Some(zeros)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.split('\n');

    let mut dial: i32 = 50;
    let mut zeros = 0;

    for line in lines {
        let mut chars = line.chars();
        
        // Parse out the direction maybe make an enum later if im feeling fancy
        let direction = match chars.next() {
            Some(c) => c,
            None => continue,
        };
        
        // Parse distance 
        let dist_str: String = chars.take_while(|x| x.is_numeric()).collect();
        if dist_str.is_empty() { 
            panic!("Unreachable / Bad Input!")
        }
        
        let dist = dist_str.parse::<i32>().unwrap();

        match direction {
            'R' => {
                zeros += (dial + dist) as u64 / 100;
                dial = (dial + dist) % 100;
            }
            
            'L' => {
                // I love ternary expressions
                let dist_to_zero = if dial == 0 { 100 } else { dial };

                if dist >= dist_to_zero {
                    let mut count = 1;
                    
                    // We add one more zero for every subsequent hundred "clicks"
                    let remaining_dist = dist - dist_to_zero;
                    count += remaining_dist / 100;
                    
                    zeros += count as u64;
                }

                // Rust % operator does not actually work like a mathematical mod on negative numbers
                dial = (dial - dist).rem_euclid(100);
            }
            _ => panic!("Unknown direction"),
        }
    }

    Some(zeros)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
