advent_of_code::solution!(9);

fn get_differences(input: Vec<i32>) -> Vec<i32> {
    input
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let history = input.lines();
    let mut total = 0;

    for sequence in history {
        let values: Vec<i32> = 
            sequence
                .split(" ")
                .filter_map(|n| n.parse().ok())
                .collect();

        let mut sequences: Vec<Vec<i32>> = vec![values.clone()];
        
        let mut current_differences = get_differences(values);
        while !current_differences.iter().all(|&x| x == 0) {
            sequences.push(current_differences.clone());
            current_differences = get_differences(current_differences);
        }

        for s in sequences {
            total += s.last().unwrap();
        }
    }
    Some(total as u32)
}

pub fn part_two(input: &str) -> Option<i32> {
    let history = input.lines();
    let mut total: i32 = 0;

    for sequence in history {
        let values: Vec<i32> = 
            sequence
                .split(" ")
                .filter_map(|n| n.parse().ok())
                .collect();

        let mut sequences: Vec<Vec<i32>> = vec![values.clone()];
        
        let mut current_differences = get_differences(values);
        while !current_differences.iter().all(|&x| x == 0) {
            sequences.push(current_differences.clone());
            current_differences = get_differences(current_differences);
        }
        
        sequences.push(vec![0; current_differences.len()]);
        sequences.reverse();

        for i in 0..sequences.len() - 1 {
            let x = sequences[i + 1][0] - sequences[i][0];
            sequences[i + 1].insert(0, x);
        }

        total += sequences.last().unwrap().first().unwrap();
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
