use std::collections::{HashMap, HashSet};

advent_of_code::solution!(10);

fn calculate_trailhead_score(
    map: &Vec<Vec<u32>>,
    y: usize,
    x: usize,
    visited: &mut HashSet<(usize, usize)>,
) -> u32 {
    // Combine the check and insert for better performance
    if !visited.insert((y, x)) {
        return 0;
    }

    if map[y][x] == 9 {
        return 1;
    }

    let height = map.len();
    let width = map[0].len();
    let val = map[y][x];
    let mut score = 0;

    // Pre-check conditions to avoid unnecessary comparisons
    let check_above = y > 0 && map[y - 1][x] == val + 1;
    let check_left = x > 0 && map[y][x - 1] == val + 1;
    let check_right = x + 1 < width && map[y][x + 1] == val + 1;
    let check_below = y + 1 < height && map[y + 1][x] == val + 1;

    // Only recurse if condition is true
    if check_above {
        score += calculate_trailhead_score(map, y - 1, x, visited);
    }
    if check_left {
        score += calculate_trailhead_score(map, y, x - 1, visited);
    }
    if check_right {
        score += calculate_trailhead_score(map, y, x + 1, visited);
    }
    if check_below {
        score += calculate_trailhead_score(map, y + 1, x, visited);
    }

    score
}

fn calculate_unique_trailhead_score(
    map: &Vec<Vec<u32>>,
    y: usize,
    x: usize,
) -> u32 {
    if map[y][x] == 9 {
        return 1;
    }

    let height = map.len();
    let width = map[0].len();
    let val = map[y][x];
    let mut score = 0;

    // Pre-check conditions to avoid unnecessary comparisons
    let check_above = y > 0 && map[y - 1][x] == val + 1;
    let check_left = x > 0 && map[y][x - 1] == val + 1;
    let check_right = x + 1 < width && map[y][x + 1] == val + 1;
    let check_below = y + 1 < height && map[y + 1][x] == val + 1;

    // Only recurse if condition is true
    if check_above {
        score += calculate_unique_trailhead_score(map, y - 1, x);
    }
    if check_left {
        score += calculate_unique_trailhead_score(map, y, x - 1);
    }
    if check_right {
        score += calculate_unique_trailhead_score(map, y, x + 1);
    }
    if check_below {
        score += calculate_unique_trailhead_score(map, y + 1, x);
    }

    score
}

pub fn part_one(input: &str) -> Option<u32> {
    // Each trailheads score is the number of 9's it can reach
    let map: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .collect();

    let mut score = 0;

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 0 {
                score += calculate_trailhead_score(&map, y, x, &mut HashSet::new());
            }
        }
    }

    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
        // Each trailheads score is the number of 9's it can reach
        let map: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .collect();

    let mut score = 0;

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 0 {
                score += calculate_unique_trailhead_score(&map, y, x);
            }
        }
    }

    Some(score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
