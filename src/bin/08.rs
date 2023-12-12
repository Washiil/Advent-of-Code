use std::collections::HashMap;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let directions = lines[0].chars().collect::<Vec<_>>();

    let mut map: HashMap<String, (String, String)> = HashMap::new();
    
    // Build Map
    for &line in &lines[2..] {
        let address = line[0..3].to_string();
        let left = line[7..10].to_string();
        let right = line[12..15].to_string();
        println!("{address}, {left}, {right}");
        map.insert(address, (left, right));
    }

    let mut location = "AAA";
    let mut steps = 0;
    while location != "ZZZ" {
        for c in &directions {
            if let Some(choices) = map.get(location) {
                match c {
                    'R' => {
                        location = &choices.1
                    },
                    'L' => {
                        location = &choices.0
                    },
                    _ => {
                        panic!("Invalid input!");
                    }
                }
                steps += 1;
            }
            
            if location == "ZZZ" {
                break;
            }
        }
    }
    Some(steps)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
