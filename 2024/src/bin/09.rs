advent_of_code::solution!(9);

pub fn part_one(s: &str) -> Option<u64> {
    let nums: Vec<usize> = s
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    let mut blocks: Vec<Option<usize>> = Vec::new();
    let mut free_spots: Vec<usize> = Vec::new();

    let mut is_file = true;
    let mut file_id = 0;

    for &n in &nums {
        if is_file {
            for _ in 0..n {
                blocks.push(Some(file_id));
            }
            file_id += 1;
        } else {
            for _ in 0..n {
                let idx = blocks.len();
                free_spots.push(idx);
                blocks.push(None);
            }
        }
        is_file = !is_file;
    }

    for &pos in &free_spots {
        if pos >= blocks.len() {
            break;
        }
        if let Some(val) = blocks.pop() {
            if let Some(v) = val {
                blocks[pos] = val;
            }
        }
        while matches!(blocks.last(), Some(None)) {
            blocks.pop();
        }
    }

    let mut answer = 0u64;

    for (pos, &v) in blocks.iter().enumerate() {
        if let Some(file_id) = v {
            answer += pos as u64 * file_id as u64;
        }
    }

    Some(answer)
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
