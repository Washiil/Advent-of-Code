advent_of_code::solution!(1);

use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut list1: Vec<i32> = vec![];
    let mut list2: Vec<i32> = vec![];

    for pair in lines {
        let nums: Vec<&str> = pair.split("   ").collect();
        if nums.len() != 2 {
            println!("Error parsing input");
            return None;
        }

        if let Ok(num1) = nums[0].parse::<i32>() {
            list1.push(num1);
        } else {
            panic!("Error parsing")
        }

        if let Ok(num2) = nums[1].parse::<i32>() {
            list2.push(num2);
        } else {
            panic!("Error parsing")
        }
    }

    list1.sort();
    list2.sort();

    if list1.len() != list2.len() {
        println!("Lengths of inputs do not match!");
    }

    let mut output: i32 = 0;
    for i in 0..list1.len() {
        output += (list1[i] - list2[i]).abs();
    }
    Some(output as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut similarity_score = 0;

    let mut list_similarity: HashMap<u32, u32> = HashMap::new();

    let lines = input.lines();
    let mut list1: Vec<u32> = vec![];
    let mut list2: Vec<u32> = vec![];

    for pair in lines {
        let nums: Vec<&str> = pair.split("   ").collect();
        if nums.len() != 2 {
            println!("Error parsing input");
            return None;
        }

        if let Ok(num1) = nums[0].parse::<u32>() {
            list1.push(num1);
        } else {
            panic!("Error parsing")
        }

        if let Ok(num2) = nums[1].parse::<u32>() {
            list2.push(num2);
        } else {
            panic!("Error parsing")
        }
    }

    for i in list2 {
        if let Some(val) = list_similarity.get_mut(&i) {
            *val += 1;
        } else {
            list_similarity.insert(i, 1);
        }
    }

    for i in list1 {
        if let Some(val) = list_similarity.get(&i) {
            similarity_score += i * val;
        }
    }

    Some(similarity_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
