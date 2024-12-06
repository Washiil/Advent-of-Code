use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

type OrderingRules = HashMap<u32, Vec<u32>>;
type UpdateSequence = Vec<u32>;

#[derive(Default)]
struct InputData {
    ordering_rules: OrderingRules,
    sequences: Vec<UpdateSequence>,
}

impl InputData {
    fn parse(input: &str) -> Self {
        let mut data = InputData::default();

        for line in input.lines() {
            if let Some((n1, n2)) = parse_rule(line) {
                data.ordering_rules.entry(n2).or_default().push(n1);
            } else if let Some(nums) = parse_sequence(line) {
                data.sequences.push(nums);
            }
        }

        data
    }
}

// Parser functions
fn parse_sequence(line: &str) -> Option<UpdateSequence> {
    if !line.contains(',') {
        return None;
    }

    Some(
        line.split(',')
            .filter_map(|n| n.trim().parse().ok())
            .collect(),
    )
}

fn parse_rule(line: &str) -> Option<(u32, u32)> {
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() != 2 {
        return None;
    }

    Some((parts[0].trim().parse().ok()?, parts[1].trim().parse().ok()?))
}

// Validation functions
fn is_valid_sequence(seq: &[u32], rules: &OrderingRules, seen: &mut HashSet<u32>) -> bool {
    for &val in seq {
        seen.insert(val);

        if let Some(required_nums) = rules.get(&val) {
            if required_nums
                .iter()
                .any(|&req| seq.contains(&req) && !seen.contains(&req))
            {
                return false;
            }
        }
    }
    true
}

fn get_median(seq: &[u32]) -> u32 {
    seq[seq.len() / 2]
}

// Topological sort implementation
fn rebuild_vector(rules: &OrderingRules, original: Vec<u32>) -> Vec<u32> {
    let mut visited = HashSet::new();
    let mut result = Vec::new();

    fn visit(
        node: u32,
        rules: &OrderingRules,
        visited: &mut HashSet<u32>,
        result: &mut Vec<u32>,
        original: &[u32],
    ) {
        if visited.contains(&node) {
            return;
        }

        if let Some(deps) = rules.get(&node) {
            for &dep in deps {
                if original.contains(&dep) {
                    visit(dep, rules, visited, result, original);
                }
            }
        }

        visited.insert(node);
        result.push(node);
    }

    for &node in &original {
        visit(node, rules, &mut visited, &mut result, &original);
    }

    result
}

// Solution implementations
pub fn part_one(input: &str) -> Option<u32> {
    let data = InputData::parse(input);

    let sum = data
        .sequences
        .iter()
        .filter_map(|seq| {
            let mut seen = HashSet::new();
            is_valid_sequence(seq, &data.ordering_rules, &mut seen).then(|| get_median(seq))
        })
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = InputData::parse(input);

    let wrong_sequences: Vec<UpdateSequence> = data
        .sequences
        .iter()
        .filter_map(|seq| {
            let mut seen = HashSet::new();
            (!is_valid_sequence(seq, &data.ordering_rules, &mut seen)).then(|| seq.clone())
        })
        .collect();

    let sum = wrong_sequences
        .iter()
        .map(|seq| {
            let sorted = rebuild_vector(&data.ordering_rules, seq.clone());
            get_median(&sorted)
        })
        .sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
