advent_of_code::solution!(5);

fn parse_ranges() {}

pub fn part_one(input: &str) -> Option<u64> {
    let mut input_parts = input.split("\n\n");
    let mut result = 0;

    let ranges = input_parts
        .next()
        .expect("Malformed Input")
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let mut parts = line
                .split("-")
                .map(|v| v.parse::<i64>().expect("Invalid number"));
            
            println!("{:?}", line);

            let a = parts.next().expect("expected two numbers");
            let b = parts.next().expect("expected two numbers");
            (a, b)
        })
        .collect::<Vec<(i64, i64)>>();

    let items = input_parts.next().expect("Malformed Input")
        .lines()
        .map(|v| v.parse::<i64>().expect("Invalid Input"))
        .collect::<Vec<i64>>();

    for item in items {
        for (start, end) in &ranges {
            if *start <= item && item <= *end {
                result += 1;
                break;
            }
        }
    }

    Some(result)
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
