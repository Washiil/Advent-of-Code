advent_of_code::solution!(4);

/*
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
*/
fn count_adjacent(board: &Vec<Vec<char>>, r: usize, c: usize) -> u8 {
let rows = board.len() as isize;
    let cols = board[0].len() as isize;
    let mut adjacent = 0;

    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }

            let new_r = (r as isize) + i;
            let new_c = (c as isize) + j;

            if new_r >= 0 && new_r < rows && new_c >= 0 && new_c < cols {
                if board[new_r as usize][new_c as usize] == '@' {
                    adjacent += 1;
                }
            }
        }
    }

    adjacent
}

pub fn part_one(input: &str) -> Option<u64> {
    /*
    The forklifts can only access a roll of paper if there are fewer than four rolls of paper in the eight adjacent positions. 
     */
    let rolls = input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let rows = rolls.len();
    let cols = rolls[0].len();

    let mut valid_rolls = 0;

    for r in 0..rows {
        for c in 0..cols {
            if rolls[r][c] != '@' {
                continue;
            }

            let adj = count_adjacent(&rolls, r, c);
            if adj >= 4 {
                continue;
            }

            valid_rolls += 1;
        }
    }
    Some(valid_rolls)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut rolls = input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let rows = rolls.len();
    let cols = rolls[0].len();

    let squares = rows * cols;

    let mut valid_rolls = 0;

    for _ in 0..squares {
        for r in 0..rows {
            for c in 0..cols {
                if rolls[r][c] != '@' {
                    continue;
                }
    
                let adj = count_adjacent(&rolls, r, c);
                if adj >= 4 {
                    continue;
                }
    
                rolls[r][c] = '.';
    
                valid_rolls += 1;
            }
        }
    }
    Some(valid_rolls)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
