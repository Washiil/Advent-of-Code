use crate::runner::Runner;
use crate::utility::read_lines;

pub struct Day01;

impl Runner for Day01 {
    fn part_one(&self) -> u32 {
        let mut total = 0;
        if let Ok(lines) = read_lines("./input/day01.txt") {
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                if let Ok(value) = line {
                    let nums: Vec<u32> = value
                        .chars()
                        .filter_map(|n| n.to_digit(10))
                        .collect();
                    
                    let f = nums.first().unwrap();
                    let l = nums.last().unwrap();

                    total += (f * 10) + l
                }
            }
        }
        return total;
    }

    fn part_two(&self) -> u32 {
        let mut total = 0;
        if let Ok(lines) = read_lines("./input/day01.txt") {
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                if let Ok(value) = line {
                    let mut output = String::from("");
                    let mut left = 0;

                    while left < value.len() {
                        let part = &value[left..];
                        if part.starts_with("one") {
                            output += "1";
                            left += 2;
                        }
                        else if part.starts_with("two") {
                            output += "2";
                            left += 2;
                        }
                        else if part.starts_with("three") {
                            output += "3";
                            left += 4;
                        }
                        else if part.starts_with("four") {
                            output += "4";
                            left += 3;
                        }
                        else if part.starts_with("five") {
                            output += "5";
                            left += 3;
                        }
                        else if part.starts_with("six") {
                            output += "6";
                            left += 2;
                        }
                        else if part.starts_with("seven") {
                            output += "7";
                            left += 4;
                        }
                        else if part.starts_with("eight") {
                            output += "8";
                            left += 4;
                        }
                        else if part.starts_with("nine") {
                            output += "9";
                            left += 3;
                        }
                        else {
                            output += &part[0..1].to_string();
                            left += 1;
                        }
                    }

                    let nums: Vec<u32> = output
                        .chars()
                        .filter_map(|n| n.to_digit(10))
                        .collect();
                    
                    let f = nums.first().unwrap();
                    let l = nums.last().unwrap();

                    total += (f * 10) + l
                }
            }
        }
        return total;
    }
}