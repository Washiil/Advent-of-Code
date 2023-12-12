advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0;
    let lines = input.lines();
    
    for line in lines {
        let nums: Vec<u32> = line
            .chars()
            .filter_map(|n| n.to_digit(10))
            .collect();
    
        let f = nums.first().unwrap();
        let l = nums.last().unwrap();
        total += (f * 10) + l
    }
    return Some(total);
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total = 0;
    
    let lines = input.lines();

    for line in lines {
        let mut output = String::from("");
        let mut left = 0;

        while left < line.len() {
            let part = &line[left..];
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
    
    return Some(total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(281));
    }
}
