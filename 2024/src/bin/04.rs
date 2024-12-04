advent_of_code::solution!(4);

pub fn check_for_xmas(
    board: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    direction: (i32, i32),
) -> bool {
    const MAS: [char; 3] = ['M', 'A', 'S'];
    let (dx, dy) = direction;

    for (i, target) in MAS.iter().enumerate() {
        let new_row = (row as i32 + dx * (i as i32 + 1)) as usize;
        let new_col = (col as i32 + dy * (i as i32 + 1)) as usize;

        match board.get(new_row).and_then(|r| r.get(new_col)) {
            Some(&c) if c == *target => continue,
            _ => return false,
        }
    }
    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut output = 0;

    let board = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    const DIRECTIONS: [(i32, i32); 8] = [
        (-1, 1),
        (0, 1),
        (1, 1),
        (-1, 0),
        (1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];

    for row in 0..board.len() {
        for col in 0..board[row].len() {
            let c = board[row][col];

            if c == 'X' {
                for dir in DIRECTIONS {
                    if check_for_xmas(&board, row, col, dir) {
                        output += 1;
                    }
                }
            }
        }
    }

    Some(output)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut output: u64 = 0;

    let board = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    const DIRECTIONS: [(i32, i32); 5] = [(0, 0), (-1, 1), (1, -1), (-1, -1), (1, 1)];
    const PATTERNS: [&str; 4] = ["AMSMS", "AMSSM", "ASMMS", "ASMSM"];

    for row in 1..board.len() - 1 {
        for col in 1..board[row].len() - 1 {
            let mut s = String::new();

            for dir in DIRECTIONS {
                let x = row as i32 + dir.0;
                let y = col as i32 + dir.1;

                s.push(board[x as usize][y as usize]);
            }

            if PATTERNS.contains(&s.as_str()) {
                output += 1;
            }
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
