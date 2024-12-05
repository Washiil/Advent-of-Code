use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

fn parse_sequence(line: &str) -> Option<Vec<u32>> {
    if !line.contains(',') {
        return None;
    }

    Some(line
        .split(',')
        .filter_map(|n| n.trim().parse().ok())
        .collect())
}

fn parse_rule(line: &str) -> Option<(u32, u32)> {
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() != 2 {
        return None;
    }

    let first = parts[0].trim().parse().ok()?;
    let second = parts[1].trim().parse().ok()?;
    
    Some((first, second))
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();

    // The key is an integer and the value is the list of all numbers that should be printed before it.
    let mut ordering_rules: HashMap<u32, Vec<u32>> = HashMap::new();

    let output = lines.filter_map(|line| {
        // Check for a rule: "int|int"
        if let Some((n1, n2)) = parse_rule(line) {
            if let Some(v) = ordering_rules.get_mut(&n2) {
                v.push(n1);
            } else {
                ordering_rules.insert(n2, vec![n1]);
            }
        }
        
        // Check for sequence: "int,int,int"
        if let Some(seq) = parse_sequence(line) {
            let mut seen: Vec<u32> = Vec::new();
            for val in &seq {
                seen.push(*val);
                if let Some(orders) = ordering_rules.get(&val) {
                    for ord in orders {
                        if seq.contains(ord) && !seen.contains(ord) {
                            return None;
                        }
                    }
                }
            }

            return Some(seq[seq.len() / 2]);
        }
        None
    }).sum();

    Some(output)
}

fn rebuild_vector(ordering_rules: &HashMap<u32, Vec<u32>>, original_vector: Vec<u32>) -> Vec<u32> {
    let mut visited = HashSet::new();
    let mut result = Vec::new();

    // Helper function for topological sorting
    fn visit(
        node: u32,
        rules: &HashMap<u32, Vec<u32>>,
        visited: &mut HashSet<u32>,
        result: &mut Vec<u32>,
        original: &Vec<u32>,
    ) {
        if visited.contains(&node) {
            return; // Skip if already visited
        }

        if let Some(dependencies) = rules.get(&node) {
            for &dep in dependencies {
                if original.contains(&dep) {
                    visit(dep, rules, visited, result, original); // Visit dependencies first
                }
            }
        }

        visited.insert(node);
        result.push(node); // Add to the result in post-order
    }

    // Rebuild the original vector
    for &node in &original_vector {
        visit(
            node,
            &ordering_rules,
            &mut visited,
            &mut result,
            &original_vector,
        );
    }

    result
}

pub fn part_two(input: &str) -> Option<u32> {
    // Parse input into rules and sequences
    let mut ordering_rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut sequences = Vec::new();

    // SParse the input itself
    for line in input.lines() {
        if let Some((n1, n2)) = parse_rule(line) {
            ordering_rules.entry(n2)
                .or_default()
                .push(n1);
        } else if let Some(nums) = parse_sequence(line) {
            sequences.push(nums);
        }
    }

    let wrong_lines: Vec<Vec<u32>> = sequences
        .iter()
        .filter_map(|line| {
            let mut seen: Vec<u32> = vec![];

            for val in line {
                seen.push(*val);
                if let Some(orders) = ordering_rules.get(&val) {
                    for &ord in orders {
                        if line.contains(&ord) && !seen.contains(&ord) {
                            return Some(line.to_vec());
                        }
                    }
                }
            }

            None
        })
        .collect();

    let out = wrong_lines
        .iter()
        .map(|line| {
            let sorted = rebuild_vector(&ordering_rules, line.to_vec());
            sorted[line.len() / 2]
        })
        .sum();

    Some(out)
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
