use std::{collections::{HashMap, HashSet}, process::id};

advent_of_code::solution!(12);

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn calculate_region_area(point: (isize, isize), identifier: char, garden: &Vec<Vec(isize, isize)>>) -> u32 {
    let mut output = 0;

    fn dfs(row: isize, col: isize, identifier: char, garden: &Vec<(isize, isize)>, direction: &(isize, isize)) -> u32 {
        let new_row = row + direction.0;
        let new_col = col + direction.1;

        if let Some(c) = garden.get(new_row as usize).and_then(|row| row.get)

        DIRECTIONS.iter().map(|dir| dfs(new_row, new_col, identifier, garden, dir)).sum()
    }

    dfs(point.0, point.1, identifier, garden, &(0, 0))
}

pub fn part_one(input: &str) -> Option<u32> {

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
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
