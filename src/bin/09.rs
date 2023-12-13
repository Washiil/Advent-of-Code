advent_of_code::solution!(9);

fn get_differences(input: Vec<u32>) -> Vec<u32> {
    let differences: Vec<u32> = 
            input
                .windows(2)
                .map(|pair| pair[0] - pair[1])
                .collect();
}

pub fn part_one(input: &str) -> Option<u32> {
    let history = input.lines();

    for sequence in history {
        let values: Vec<u32> = 
            sequence
                .split(" ")
                .filter_map(|n| n.parse().ok())
                .collect();
        
        let mut sequences: Vec<Vec<u32>> = Vec::new();

        let differences: Vec<u32> = 
            values
                .windows(2)
                .map(|pair| pair[0] - pair[1])
                .collect();
        
        while differences.iter().all(|&x| x == values[0]) {
            sequences.push(differences.clone())
        }
    }

    None
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
