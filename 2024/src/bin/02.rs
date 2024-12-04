advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut output = 0;
    for line in input.lines() {
        let nums: Vec<u32> = line
            .split_whitespace()
            .map(|c| c.parse::<u32>().unwrap())
            .collect();

        let ascending = nums.windows(2).all(|w| w[1] >= w[0] && w[1] - w[0] <= 3);
        let descending = nums.windows(2).all(|w| w[1] <= w[0] && w[0] - w[1] <= 3);
        let differ = nums
            .windows(2)
            .all(|w| w[0].abs_diff(w[1]) > 0 && w[0].abs_diff(w[1]) < 4);

        if (ascending || descending) && differ {
            output += 1;
        }
    }
    Some(output)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut output = 0;

    for line in input.lines() {
        let nums: Vec<u32> = line
            .split_whitespace()
            .map(|c| c.parse::<u32>().unwrap())
            .collect();

        for i in 0..nums.len() {
            let filtered: Vec<&u32> = nums
                .iter()
                .enumerate()
                .filter(|&(j, _)| i != j)
                .map(|(_, v)| v)
                .collect();

            let ascending = filtered
                .windows(2)
                .all(|w| w[1] >= w[0] && w[1] - w[0] <= 3);
            let descending = filtered
                .windows(2)
                .all(|w| w[1] <= w[0] && w[0] - w[1] <= 3);
            let differ = filtered
                .windows(2)
                .all(|w| w[0].abs_diff(*w[1]) > 0 && w[0].abs_diff(*w[1]) < 4);
            if (ascending || descending) && differ {
                output += 1;
                break;
            }
        }
    }

    Some(output)
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
