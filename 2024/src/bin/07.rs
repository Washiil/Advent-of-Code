advent_of_code::solution!(7);

#[derive(Debug, Clone, Copy)]
enum Operator {
    Multiply,
    Add,
    Concatenate,
}

fn evaluate_expression(nums: &[u64], perm: &[Operator], target: u64) -> bool {
    let mut temp = nums.to_vec();
    for &op in perm {
        let n1 = temp.pop().unwrap();
        if n1 > target {
            return false;
        }

        let n2 = temp.pop().unwrap();
        let new_val = match op {
            Operator::Multiply => n1 * n2,
            Operator::Add => n1 + n2,
            Operator::Concatenate => format!("{}{}", n1, n2).parse::<u64>().unwrap(),
        };
        temp.push(new_val);
    }
    temp[0] == target
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
    const OPERATORS: [Operator; 2] = [Operator::Multiply, Operator::Add];

    let output: u64 = input
        .lines()
        .filter_map(|line| {
            let split: Vec<&str> = line.split(": ").collect();
            let target: u64 = split[0].parse().ok()?;
            let nums: Vec<u64> = split[1]
                .split(' ')
                .filter_map(|n| n.parse::<u64>().ok())
                .rev()
                .collect();

            let num_of_operations = nums.len() - 1;

            let permutations = generate_permutations(&OPERATORS, num_of_operations);

            if permutations
                .iter()
                .any(|perm| evaluate_expression(&nums, perm, target))
            {
                return Some(target);
            }
            None
        })
        .sum();

    Some(output)
}

pub fn part_two(input: &str) -> Option<u64> {
    const OPERATORS: [Operator; 3] = [Operator::Add, Operator::Multiply, Operator::Concatenate];

    let output: u64 = input
        .lines()
        .filter_map(|line| {
            let split: Vec<&str> = line.split(": ").collect();
            let target: u64 = split[0].parse().ok()?;
            let nums: Vec<u64> = split[1]
                .split(' ')
                .filter_map(|n| n.parse::<u64>().ok())
                .rev()
                .collect();

            let num_of_operations = nums.len() - 1;

            let permutations = generate_permutations(&OPERATORS, num_of_operations);

            if permutations
                .iter()
                .any(|perm| evaluate_expression(&nums, perm, target))
            {
                return Some(target);
            }
            None
        })
        .sum();

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
