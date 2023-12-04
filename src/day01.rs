use std::collections::btree_map::VacantEntry;

use crate::util::read_lines;

pub fn part_1() -> u32 {
	let mut total = 0;
	if let Ok(lines)  = read_lines("input/day01.txt"){
		for line in lines {
			if let Ok(value) = line {
				let x: Vec<u32> = value
					.chars()
					.filter_map(|c| c.to_digit(10))
					.collect();
				
				if let (Some(first), Some(last)) = (x.first(), x.last()) {
					let combined = (first * 10) + last;
					total += combined
				}
			}
		}
		total
	}
	else {
		panic!("Incorrect input!")
	}
}

pub fn part_2() -> u32 {
	// TODO: EWWWWWW Please fix <3
	let mut total = 0;
	let num_words = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
	if let Ok(lines)  = read_lines("input/day01.txt"){
		for line in lines {
			if let Ok(value) = line {
				let mut parsed = value;
				println!("{}", parsed);
				parsed = parsed.replace("one", "1");
				parsed = parsed.replace("two", "2");
				parsed = parsed.replace("three", "3");
				parsed = parsed.replace("four", "4");
				parsed = parsed.replace("five", "5");
				parsed = parsed.replace("six", "6");
				parsed = parsed.replace("seven", "7");
				parsed = parsed.replace("eight", "8");
				parsed = parsed.replace("nine", "9");
				
				println!("{}", parsed);

				let x: Vec<u32> = parsed
					.chars()
					.filter_map(|c| c.to_digit(10))
					.collect();

				if let (Some(first), Some(last)) = (x.first(), x.last()) {
					let combined = (first * 10) + last;
					total += combined
				}
			}
		}
		total
	}
	else {
		panic!("Incorrect input!")
	}
}