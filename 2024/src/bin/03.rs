advent_of_code::solution!(3);

use std::f32::consts::PI;

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut output = 0;
    for cap in re.captures_iter(input) {
        let arg1 = &cap[1];
        let arg2 = &cap[2];
        output += arg1.parse::<u32>().unwrap() * arg2.parse::<u32>().unwrap()
    }

    Some(output)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let x: Vec<&str> = input.split("don't()").collect();

    let y: u32 = x
        .iter()
        .enumerate()
        .filter_map(|(i, part)| {
            if i == 0 {
                return Some(part.to_string())
            }
            if part.contains("do()") {
                // Split the string
                let split = part.split("do()");
                // Skip the first element and collect the rest
                Some(split.skip(1).collect::<Vec<_>>().join(""))
            } else {
                None
            }
        })
        .map(|part| {
            let mut out = 0;
            for cap in re.captures_iter(&part) {
                let arg1 = &cap[1];
                let arg2 = &cap[2];
                out += arg1.parse::<u32>().unwrap() * arg2.parse::<u32>().unwrap()
            }

            out
        })
        .sum();

    Some(y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
