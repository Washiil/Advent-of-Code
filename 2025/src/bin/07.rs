advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let row_size = input.lines().next()?.len();
    let mut splits: u64 = 0;

    // This will contain values for currently active beams empty by default
    // Capacity can likely be shrunk need to verify
    let mut beams_curr = vec![false; row_size + 2];
    let mut beams_next = vec![false; row_size + 2];

    for line in input.lines().step_by(2) {
        for (idx, c) in line.chars().enumerate() {
            if c == 'S' {
                beams_next[idx + 1] = true;
                continue;
            }
            if c == '.' || !beams_curr[idx + 1] {
                continue;
            };
            // We now have both a beam and a ^

            // Split and Mark these as they need to be added
            splits += 1;
            beams_next[idx] = true;
            beams_next[idx + 1] = false;
            beams_next[idx + 2] = true;
        }
        beams_curr.copy_from_slice(&beams_next);
    }

    Some(splits)
}

pub fn part_two(input: &str) -> Option<u64> {
    let row_size = input.lines().next()?.len();
    let mut worlds = vec![0; row_size + 2];

    for line in input.lines().step_by(2) {
        for (idx, c) in line.chars().enumerate() {
            if c == 'S' {
                worlds[idx + 1] = 1;
                continue;
            }
            if c == '.' || worlds[idx + 1] == 0 {
                continue;
            };

            // Count by the number of worlds going in
            worlds[idx] += worlds[idx + 1];
            worlds[idx + 2] += worlds[idx + 1];

            worlds[idx + 1] = 0;
        }
        println!("{:?}", worlds);
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
