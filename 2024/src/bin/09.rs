advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let mut index: i64 = -1;
    let mut is_file = true;

    // Parse the disk map into a `filesystem` representation
    let mut filesystem: Vec<char> = input
        .chars()
        .filter_map(|c| {
            let length = c.to_digit(10)? as usize;
            if is_file {
                index += 1;
                is_file = false;
                Some(format!("{}", index).repeat(length))
            } else {
                is_file = true;
                Some(".".repeat(length))
            }
        })
        .collect::<String>()
        .chars()
        .collect();

    println!("Initial filesystem: {:?}", filesystem);
    let mut left = 0;
    let mut right = filesystem.len() - 1;

    // Move file blocks to compact the filesystem
    while left < right {
        if filesystem[right] == '.' {
            right -= 1;
        } else if filesystem[left] == '.' {
            filesystem[left] = filesystem[right];
            filesystem[right] = '.';
            left += 1;
            right -= 1;
        } else {
            left += 1;
        }
    }

    let mut output: u64 = 0;
    for (i, c) in filesystem.iter().enumerate() {
        if let Some(v) = c.to_digit(10) {
            output += v as u64 * i as u64;
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
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
