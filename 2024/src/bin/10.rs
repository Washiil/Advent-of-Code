use std::collections::{HashMap, HashSet};

advent_of_code::solution!(10);

const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
const TARGET: u32 = 9;

fn calculate_trailhead_score(
    map: &Vec<Vec<u32>>,
    y: usize,
    x: usize,
    visited: &mut HashSet<(usize, usize)>,
) -> u32 {
    // Base Case
    if visited.contains(&(y, x)) {
        return 0;
    }
    visited.insert((y, x));

    if map[y][x] == 9 {
        return 1;
    }

    let height = map.len();
    let width = map[0].len();
    let val = map[y][x];

    let mut score = 0;

    // Check above
    if y > 0 {
        if map[y - 1][x] == val + 1 {
            score += calculate_trailhead_score(map, y - 1, x, visited)
        }
    }

    // Check left
    if x > 0 {
        if map[y][x - 1] == val + 1 {
            score += calculate_trailhead_score(map, y, x - 1, visited)
        }
    }

    // Check right
    if x + 1 < width {
        if map[y][x + 1] == val + 1 {
            score += calculate_trailhead_score(map, y, x + 1, visited)
        }
    }

    // Check below
    if y + 1 < height {
        if map[y + 1][x] == val + 1 {
            score += calculate_trailhead_score(map, y + 1, x, visited)
        }
    }

    score
}

fn calculate_unique_trailhead_score(
    map: &Vec<Vec<u32>>,
    y: usize,
    x: usize,
) -> u32 {
    // Base Case

    if map[y][x] == 9 {
        return 1;
    }

    let height = map.len();
    let width = map[0].len();
    let val = map[y][x];

    let mut score = 0;

    // Check above
    if y > 0 {
        if map[y - 1][x] == val + 1 {
            score += calculate_unique_trailhead_score(map, y - 1, x)
        }
    }

    // Check left
    if x > 0 {
        if map[y][x - 1] == val + 1 {
            score += calculate_unique_trailhead_score(map, y, x - 1)
        }
    }

    // Check right
    if x + 1 < width {
        if map[y][x + 1] == val + 1 {
            score += calculate_unique_trailhead_score(map, y, x + 1)
        }
    }

    // Check below
    if y + 1 < height {
        if map[y + 1][x] == val + 1 {
            score += calculate_unique_trailhead_score(map, y + 1, x)
        }
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
