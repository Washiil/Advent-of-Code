use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

const BLANK_SPACE: u32 = '.' as u32;

pub fn part_one(input: &str) -> Option<u32> {
    // Stores coordinates in (x, y) format
    let mut frequency_antennas: HashMap<u32, Vec<(i32, i32)>> = HashMap::new();
    let mut unique_antinodes: HashSet<(i32, i32)> = HashSet::new();

    let rows: Vec<&str> = input.lines().collect();
    let height = rows.len() as i32;
    let width = rows[0].len() as i32;

    for (y, row) in rows.iter().enumerate() {
        for (x, val) in row.chars().map(|c| c as u32).enumerate() {
            if val != BLANK_SPACE {
                frequency_antennas
                    .entry(val)
                    .or_insert_with(Vec::new)
                    .push((x as i32, y as i32));
            }
        }
    }

    for (_, stations) in frequency_antennas {
        let n = stations.len();

        for i in 0..n {
            for j in i + 1..n {
                let (x1, y1) = stations[i];
                let (x2, y2) = stations[j];

                let dx = x2 - x1;
                let dy = y2 - y1;

                let antinodes: [(i32, i32); 2] = [(x1 - dx, y1 - dy), (x2 + dx, y2 + dy)];

                for node in antinodes {
                    if node.0 >= 0 && node.0 < width && node.1 >= 0 && node.1 < height {
                        unique_antinodes.insert(node);
                    }
                }
            }
        }
    }

    Some(unique_antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    // Stores coordinates in (x, y) format
    let mut frequency_antennas: HashMap<u32, Vec<(i32, i32)>> = HashMap::new();
    let mut unique_antinodes: HashSet<(i32, i32)> = HashSet::new();

    let rows: Vec<&str> = input.lines().collect();
    let height = rows.len() as i32;
    let width = rows[0].len() as i32;

    for (y, row) in rows.iter().enumerate() {
        for (x, val) in row.chars().map(|c| c as u32).enumerate() {
            if val != BLANK_SPACE {
                frequency_antennas
                    .entry(val)
                    .or_insert_with(Vec::new)
                    .push((x as i32, y as i32));
            }
        }
    }

    for (_, stations) in frequency_antennas {
        let n = stations.len();

        for i in 0..n {
            for j in i + 1..n {
                let (x1, y1) = stations[i];
                let (x2, y2) = stations[j];
                unique_antinodes.insert((x1, y1));
                unique_antinodes.insert((x2, y2));

                let dx = x2 - x1;
                let dy = y2 - y1;

                // Direction 1
                let mut node = (x1 - dx, y1 - dy);
                while node.0 >= 0 && node.0 < width && node.1 >= 0 && node.1 < height {
                    unique_antinodes.insert(node);
                    node = (node.0 - dx, node.1 - dy);
                }

                // Direction 2
                let mut node = (x2 + dx, y2 + dy);
                while node.0 >= 0 && node.0 < width && node.1 >= 0 && node.1 < height {
                    unique_antinodes.insert(node);
                    node = (node.0 + dx, node.1 + dy);
                }
            }
        }
    }
    Some(unique_antinodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
