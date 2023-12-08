use crate::{runner::Runner, utility::read_lines};

pub struct Day03;

impl Day03 {
    // This function must remove all letters it uses to avoid double counting hence the mutable reference
    fn number_from_line(&self, line: &mut Vec<char>, index: usize) -> u32 {
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
}

impl Runner for Day03 {
    fn part_one(&self) -> u32 {
        let mut total = 0;
        let lines = read_lines("./input/day03.txt");

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
                            let line_sum = self.number_from_line(&mut data[modified_row], modified_col);
                            total += line_sum;
                        }
                    }
                }
            }
        }
        return total;
    }

    fn part_two(&self) -> u32 {
        let mut total = 0;
        let lines = read_lines("./input/day03.txt");

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
                            let line_sum = self.number_from_line(&mut data[modified_row], modified_col);
                            gears.push(line_sum);
                        }
                    }
                }
                if gears.len() == 2 {
                    total += gears.iter().fold(1, |acc, x| acc * x);
                }
            }
        }
        return total;
    }
}