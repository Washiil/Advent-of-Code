use std::collections::HashMap;

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
            
            println!("{}, {}", n1, n2);
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
    println!("{:#?}", ordering_rules);
    println!("{:?}", update_order);

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

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
