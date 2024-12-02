use std::collections::HashMap;

advent_of_code::solution!(8);

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!(),
        }
    }
}

fn gcd(mut a:u64, mut b:u64) -> u64{
    if a==b { return a; }
    if b > a {
        let temp = a;
        a = b;
        b = temp;
    }
    while b>0 {
        let temp = a;
        a = b;
        b = temp%b;
    }
    return a;
}

fn lcm(a:u64, b:u64) -> u64{
    // LCM = a*b / gcd
    return a*(b/gcd(a,b));
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let directions = lines[0].chars().map(Direction::from).collect::<Vec<_>>();

    let mut map: HashMap<String, (String, String)> = HashMap::new();
    
    // Build Map
    for &line in &lines[2..] {
        let address = line[0..3].to_string();
        let left = line[7..10].to_string();
        let right = line[12..15].to_string();
        map.insert(address, (left, right));
    }

    let mut location = "AAA";
    let mut steps = 0;

    while location != "ZZZ" {
        for c in &directions {
            if let Some(choices) = map.get(location) {
                location = match c {
                    Direction::Left => &choices.0,
                    Direction::Right => &choices.1
                };
                steps += 1;
            }
        }
    }
    Some(steps)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();
    let directions = lines[0].chars().map(Direction::from).collect::<Vec<_>>();

    let mut map: HashMap<String, (String, String)> = HashMap::new();

        // Build Map
    for &line in lines.get(2..)?.iter() {
        let address = line[0..3].to_string();
        let left = line[7..10].to_string();
        let right = line[12..15].to_string();
        map.insert(address, (left, right));
    }

    let mut all_steps = Vec::new();

    let start_locations =
        map
            .keys()
            .filter(|i| i.ends_with('A'))
            .cloned()
            .collect::<Vec<String>>();

    for start in start_locations {
        let mut location = start.clone();
        let mut steps: i64 = 0;

        while !location.ends_with('Z') {
            for direction in &directions {
                let (left, right) = map.get(&location).unwrap();
                location = match direction {
                    Direction::Right => right.to_string(),
                    Direction::Left => left.to_string(),
                };
                steps += 1;
            }
        }

        all_steps.push(steps);
    }

    Some(all_steps.iter().fold(1, |acc, s| lcm(acc as u64, *s as u64)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(6));
    }
}