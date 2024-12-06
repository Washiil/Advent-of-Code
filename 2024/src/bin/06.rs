use std::collections::HashSet;

advent_of_code::solution!(6);

#[derive(Clone, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    pub fn to_cardinal(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    pub fn turn_right(&mut self) {
        *self = match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        };
    }    
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "Up"),
            Direction::Down => write!(f, "Down"),
            Direction::Left => write!(f, "Left"),
            Direction::Right => write!(f, "Right"),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    let mut position: (i32, i32) = (0, 0);
    let mut direction: Direction = Direction::Up;

    for (row, line) in input.lines().enumerate() {
        if line.contains('^') {
            for (col, c) in line.chars().enumerate() {
                if c == '^' {
                    position = (col as i32, row as i32)
                }
            }
        }
        map.push(line.chars().collect::<Vec<char>>());
    }

    visited.insert(position);

    while let Some(&character) = map.get(position.1 as usize).and_then(|row| row.get(position.0 as usize)) {
        let cardinal = direction.to_cardinal();

        if character == '#' {
            position.0 -= cardinal.0;
            position.1 -= cardinal.1;
            direction.turn_right();
        }
        else {
            visited.insert(position);

            position.0 += cardinal.0;
            position.1 += cardinal.1;
        }
    }

    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut output = 0;
    let mut map: Vec<Vec<char>> = Vec::new();

    let mut starting_position: (i32, i32) = (0, 0);
    let starting_direction: Direction = Direction::Up;

    for (row, line) in input.lines().enumerate() {
        if line.contains('^') {
            for (col, c) in line.chars().enumerate() {
                if c == '^' {
                    starting_position = (col as i32, row as i32)
                }
            }
        }
        map.push(line.chars().collect::<Vec<char>>());
    }

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '^' || map[i][j] == '#' {
                continue;
            }

            map[i][j] = '#';

            let mut visited: HashSet<((i32, i32), (i32, i32))> = HashSet::new();
            let mut direction = starting_direction.clone();
            let mut position = starting_position;

            while let Some(&character) = map.get(position.1 as usize).and_then(|row| row.get(position.0 as usize)) {
                let cardinal = direction.to_cardinal();

                if character == '#' {
                    position.0 -= cardinal.0;
                    position.1 -= cardinal.1;
                    direction.turn_right();
                }
                else {
                    if visited.contains(&(position, cardinal)) {
                        output += 1;
                        break;
                    }
                    else {
                        visited.insert((position, cardinal));
            
                        position.0 += cardinal.0;
                        position.1 += cardinal.1;
                    }
                }
            }

            visited.clear();
            map[i][j] = '.';
        }
    }

    Some(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
