advent_of_code::solution!(3);

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

    Some(
        input
            .split("don't()")
            .enumerate()
            .filter_map(|(i, part)| {
                // Because we start with a do we need to skip the first element while keeping it in the iterator
                if i == 0 {
                    return Some(part.to_string());
                }

                if part.contains("do()") {
                    let split = part.split("do()");
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
            .sum(),
    )
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
