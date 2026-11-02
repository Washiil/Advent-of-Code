advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines().step_by(2);
    let first_line = lines.next()?;
    let start = first_line.find('S')?;
    let mut splits: u64 = 0;

    let mut beams = vec![false; first_line.len() + 2];
    beams[start + 1] = true;

    for line in lines {
        for (col, _) in line.match_indices('^') {
            let i = col + 1;
            if !beams[i] {
                continue;
            }
            splits += 1;
            beams[i - 1] = true;
            beams[i] = false;
            beams[i + 1] = true;
        }
    }

    Some(splits)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines().step_by(2);
    let first_line = lines.next()?;
    let start = first_line.find('S')?;

    // Padded to avoid bounds checking
    let mut worlds = vec![0u64; first_line.len() + 2];
    worlds[start + 1] = 1;

    for line in lines {
        for (col, _) in line.match_indices('^') {
            let i = col + 1;
            let count = std::mem::take(&mut worlds[i]);
            worlds[i - 1] += count;
            worlds[i + 1] += count;
        }
    }

    Some(worlds.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
