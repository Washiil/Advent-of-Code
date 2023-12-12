advent_of_code::solution!(3);

fn number_from_line(line: &mut Vec<char>, index: usize) -> u32 {
    let mut consecutive = String::new();

    // Check to the left of the index
    for i in (0..index).rev() {
        if line[i].is_numeric() {
            consecutive.insert(0, line[i]);
            line[i] = '.';
        } else {
            break;
        }
    }

    // Add the digit at the current index
    consecutive.push(line[index]);

    // Check to the right of the index
    for i in index + 1..line.len() {
        if line[i].is_numeric() {
            consecutive.push(line[i]);
            line[i] = '.';
        } else {
            break;
        }
    }
    let parsed_number: u32 = consecutive.parse().unwrap();

    return parsed_number;
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0;
    let lines = input.lines();

    let mut data: Vec<Vec<char>> = lines
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();


    let rows = data.len();
    let cols = data[0].len();

    for row in 1..rows {
        for col in 1..cols {
            if data[row][col].is_numeric() || data[row][col] == '.' {
                continue
            }
            // We found a nuumber so symbol so its time to perform a check
            for modified_row in row - 1..=row + 1 {
                for modified_col in col - 1..=col + 1 {
                    if data[modified_row][modified_col].is_numeric() {
                        let line_sum = number_from_line(&mut data[modified_row], modified_col);
                        total += line_sum;
                    }
                }
            }
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total = 0;
    let lines = input.lines();

    let mut data: Vec<Vec<char>> = lines
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();

    let rows = data.len();
    let cols = data[0].len();

    for row in 1..rows {
        for col in 1..cols {
            if data[row][col] != '*' { 
                continue;
            }
            // We found a nuumber so symbol so its time to perform a check
            let mut gears: Vec<u32> = vec![];
            for modified_row in row - 1..=row + 1 {
                for modified_col in col - 1..=col + 1 {
                    if data[modified_row][modified_col].is_numeric() {
                        let line_sum = number_from_line(&mut data[modified_row], modified_col);
                        gears.push(line_sum);
                    }
                }
            }
            if gears.len() == 2 {
                total += gears.iter().fold(1, |acc, x| acc * x);
            }
        }
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
