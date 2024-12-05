use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();

    let mut ordering_rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut update_order: Vec<Vec<u32>> = vec![];

    for line in lines {
        if line.contains('|') {
            let nums: Vec<&str> = line.split('|').collect();

            let n1 = nums[0].parse::<u32>().unwrap();
            let n2 = nums[1].parse::<u32>().unwrap();
            
            if let Some(v) = ordering_rules.get_mut(&n2) {
                v.push(n1);
            }
            else {
                ordering_rules.insert(n2, vec![n1]);
            }
        }
        else if line.contains(',') {
            let nums = line.split(',');
            update_order.push(
                nums.map(|num| {
                    num.parse::<u32>().unwrap()
                }).collect::<Vec<u32>>()
            )
        }
    }

    

    // Check each update order with our order rules
    let correct_lines: u32 = update_order.iter().filter_map(|line| {
        let mut seen: Vec<u32> = vec![];

        for val in line {
            seen.push(*val);
            if let Some(orders) = ordering_rules.get(&val) {
                for ord in orders {
                    if line.contains(ord) && !seen.contains(ord) {
                        return None;
                    } 
                }
            }
        }

        Some(line[line.len()/ 2])
    }).sum();



    Some(correct_lines)
}

fn rebuild_vector(ordering_rules: &HashMap<u32, Vec<u32>>, original_vector: Vec<u32>) -> Vec<u32> {
    let mut visited = HashSet::new();
    let mut result = Vec::new();

    // Helper function for topological sorting
    fn visit(node: u32, rules: &HashMap<u32, Vec<u32>>, visited: &mut HashSet<u32>, result: &mut Vec<u32>, original: &Vec<u32>) {
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
        result.push(node);    // Add to the result in post-order
    }

    // Rebuild the original vector
    for &node in &original_vector {
        visit(node, &ordering_rules, &mut visited, &mut result, &original_vector);
    }

    result
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();

    let mut ordering_rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut update_order: Vec<Vec<u32>> = vec![];

    for line in lines {
        if line.contains('|') {
            let nums: Vec<&str> = line.split('|').collect();

            let n1 = nums[0].parse::<u32>().unwrap();
            let n2 = nums[1].parse::<u32>().unwrap();
            
            if let Some(v) = ordering_rules.get_mut(&n2) {
                v.push(n1);
            }
            else {
                ordering_rules.insert(n2, vec![n1]);
            }
        }
        else if line.contains(',') {
            let nums = line.split(',');
            update_order.push(
                nums.map(|num| {
                    num.parse::<u32>().unwrap()
                }).collect::<Vec<u32>>()
            )
        }
    }


    let wrong_lines: Vec<Vec<u32>> = update_order.iter().filter_map(|line| {
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
    }).collect();

    let out = wrong_lines.iter().map(|line| {
        let sorted = rebuild_vector(&ordering_rules, line.to_vec());
        sorted[line.len() / 2]
    }).sum();


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
