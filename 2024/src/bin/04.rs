advent_of_code::solution!(4);

pub fn check_for_xmas(board: &Vec<Vec<char>>, x: usize, y: usize, direction: (i32, i32)) -> bool {
    let mut index = 0;
    let xmas: [char; 3] = ['M', 'A', 'S'];

    let mut coordinates: (i32, i32) = (x as i32, y as i32);

    while index < xmas.len() {
        coordinates.0 = coordinates.0.checked_add(direction.0).unwrap();
        coordinates.1 = coordinates.1.checked_add(direction.1).unwrap();

        if let Some(row) = board.get(coordinates.0 as usize) {
            if let Some(c) = row.get(coordinates.1 as usize) {
                if *c != xmas[index] {
                    return false
                }
            }
            else {
                return false
            }
        }
        else {
            return false
        }

        index += 1;
    }

    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut output = 0;

    // Horizontal Search
    let mut board: Vec<Vec<char>> = vec![];

    for line in input.lines() {
        board.push(line.chars().collect::<Vec<char>>())
    }

    let directions: [(i32, i32); 9] = [
        (-1, 1), (0, 1), (1, 1),
        (-1, 0), (0, 0), (1, 0),
        (-1, -1), (0, -1), (1, -1)
    ];

    for row in 0..board.len() {
        for col in 0..board[row].len() {
            let c = board[row][col];

            if c == 'X' {
                for dir in directions {
                    if check_for_xmas(&board, row, col, dir) {
                        output += 1;
                    }
                }
            }
        }
    }

    Some(output)
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }
}
