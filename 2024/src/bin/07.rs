use std::{char::TryFromCharError, convert};
use std::collections::VecDeque;

advent_of_code::solution!(7);

#[derive(Debug, Clone, Copy)]
enum Operator {
    Multiply,
    Add,
    Concatination,
}

impl Operator {
    pub fn from_char(c: char) -> Option<Operator> {
        match c {
            '+' => Some(Operator::Add),
            '*' => Some(Operator::Multiply),
            _ => None
        }
    }
}

fn generate_permutations(operators: &[Operator], length: usize) -> Vec<Vec<Operator>> {
    let mut results = Vec::new();
    let mut current = vec![Operator::Multiply; length]; // Default initialization

    fn backtrack(
        operators: &[Operator],
        current: &mut Vec<Operator>,
        depth: usize,
        results: &mut Vec<Vec<Operator>>,
    ) {
        if depth == current.len() {
            results.push(current.clone());
            return;
        }
        for &op in operators {
            current[depth] = op;
            backtrack(operators, current, depth + 1, results);
        }
    }

    backtrack(operators, &mut current, 0, &mut results);
    results
}

pub fn part_one(input: &str) -> Option<u64> {
    let operators = [Operator::Add, Operator::Multiply];

    let output: u64 = input.lines().filter_map(|line| {
        let split: Vec<&str> = line.split(": ").collect();

        let target: u64 = split[0].parse::<u64>().expect("Invalid calibration target");

        let nums: Vec<u64> = split[1]
            .split(' ')
            .filter_map(|n| n.parse::<u64>().ok())
            .rev()
            .collect();

        let num_of_operations = nums.len() - 1;

        let permutations = generate_permutations(&operators, num_of_operations);

        if permutations.iter().any(|perm| {
            let mut temp = nums.clone(); // Avoid borrowing `nums`
            for op in perm {
                let n1 = temp.pop().unwrap();

                if n1 > target {
                    return false;
                }

                let n2 = temp.pop().unwrap();

                let new_val = match op {
                    Operator::Multiply => n1 * n2,
                    Operator::Add => n1 + n2,
                    _ => panic!("Invalid operators for part one!")
                };
                temp.push(new_val);
            }

            temp[0] == target
        }) {
            return Some(target);
        }
        None
    }).sum();

    Some(output)
}

pub fn part_two(input: &str) -> Option<u64> {
    let operators = [Operator::Add, Operator::Multiply, Operator::Concatination];

    let output: u64 = input.lines().filter_map(|line| {
        let split: Vec<&str> = line.split(": ").collect();

        let target: u64 = split[0].parse::<u64>().expect("Invalid calibration target");

        let nums: Vec<u64> = split[1]
            .split(' ')
            .filter_map(|n| n.parse::<u64>().ok())
            .rev()
            .collect();

        let num_of_operations = nums.len() - 1;

        let permutations = generate_permutations(&operators, num_of_operations);

        if permutations.iter().any(|perm| {
            let mut temp = nums.clone(); // Avoid borrowing `nums`
            for op in perm {
                let n1 = temp.pop().unwrap();

                if n1 > target {
                    return false;
                }

                let n2 = temp.pop().unwrap();

                let new_val = match op {
                    Operator::Multiply => n1 * n2,
                    Operator::Add => n1 + n2,
                    Operator::Concatination => {
                        format!("{}{}", n1.to_string(), n2.to_string()).parse::<u64>().unwrap()
                    }
                };
                temp.push(new_val);
            }

            temp[0] == target
        }) {
            return Some(target);
        }
        None
    }).sum();

    Some(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
