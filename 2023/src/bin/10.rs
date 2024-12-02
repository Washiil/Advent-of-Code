advent_of_code::solution!(10);

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum PipeType {
    Verticle,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Animal
}

impl From<char> for PipeType {
    fn from(value: char) -> Self {
        match value {
            '|' => PipeType::Verticle,
            '-' => PipeType::Horizontal,
            'L' => PipeType::NorthEast,
            'J' => PipeType::NorthWest,
            '7' => PipeType::SouthWest,
            'F' => PipeType::SouthEast,
            '.' => PipeType::Ground,
            'S' => PipeType::Animal,
            _ => unreachable!("Invalid Input!")
        }
    }
}

impl PipeType {
    fn can_connect(&self, other: &PipeType) -> bool {
        match self {
            PipeType::Verticle => {
                match other {
                    PipeType::Verticle|PipeType::NorthEast|PipeType::NorthWest => true,
                    _ => false
                }
            },
            PipeType::Horizontal => todo!(),
            PipeType::NorthEast => todo!(),
            PipeType::NorthWest => todo!(),
            PipeType::SouthWest => todo!(),
            PipeType::SouthEast => todo!(),
            PipeType::Ground => todo!(),
            PipeType::Animal => todo!(),
        }
    }
}

fn find_path(maze: Vec<Vec<PipeType>>, row: usize, col: usize) -> (u32, u32) {
    let current = &maze[row][col];

    let directions = vec![
        &maze[row - 1][col],
        &maze[row][col - 1],
        &maze[row][col + 1],
        &maze[row + 1][col]
    ];



    (0, 0)
}

pub fn part_one(input: &str) -> Option<u32> {
    let maze: Vec<Vec<PipeType>> = input.lines().map(|line| line.chars().map(|c| PipeType::from(c)).collect()).collect();
    let rows = maze.len();
    let cols = maze[0].len();

    let mut start_index: (usize, usize) ;

    // Finding start index
    'outer: for row in 0..rows {
        for col in 0..cols {
            if maze[row][col] == PipeType::Animal {
                start_index = (row, col);
                break 'outer
            }
        }
    }

    None
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
