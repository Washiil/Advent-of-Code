use core::panic;
use std::i32;

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

        let prev_dial = dial;

        match direction {
            Some('L') => dial = (dial + dist) % 100,
            Some('R') => dial = (dial - dist) % 100,
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
    None
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
        assert_eq!(result, None);
    }
}
