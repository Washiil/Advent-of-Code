advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    /*
    Each line of digits in your input corresponds to a single bank of batteries
    Within each bank, you need to turn on exactly two batteries
    The joltage that the bank produces is equal to the number formed by the digits on the batteries you've turned on (2, 4 = 24 not 6)
    You'll need to find the largest possible joltage each bank can produce. In the above example:
     */
    let mut joltage = 0;
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let charges = line
            .chars()
            .map(|c| c.to_digit(10).expect("Invalid Input"))
            .collect::<Vec<u32>>();
        
        if charges.len() < 2 {
            panic!("Invalid Length of Input")
        }

        let maximum = charges[..charges.len() - 1]
            .iter()
            .max()
            .expect("empty slice");
        
        let mut found = false;

        let mut largest = 0;
        for num in &charges {
            if found && num > &largest {
                largest = *num;
            }
            if num == maximum {
                found = true;
            }
        }

        let num = (maximum * 10) + largest;
        joltage += num;
    }

    Some(joltage as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
        let mut joltage = 0;
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let mut charges = line
            .chars()
            .map(|c| c.to_digit(10).expect("Invalid Input"))
            .collect::<Vec<u32>>();

        joltage += num;
    }

    Some(joltage as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
