advent_of_code::solution!(11);

/*

If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
If the stone is engraved with a number that has an even number of digits, it is replaced by two stones. The left half of the digits are engraved on the new left stone, and the right half of the digits are engraved on the new right stone. (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)
If none of the other rules apply, the stone is replaced by a new stone; the old stone's number multiplied by 2024 is engraved on the new stone.
*/
fn blink(stones: Vec<u64>) -> Vec<u64> {
    let mut new_vec = Vec::new();

    for stone in stones {
        if stone == 0 {
            new_vec.push(1);
            continue;
        }

        let str_stone = stone.to_string();
        let stone_length = str_stone.len();

        if str_stone.len() % 2 == 0 {
            new_vec.push(str_stone[0..(stone_length / 2)].parse().unwrap());
            new_vec.push(str_stone[(stone_length / 2)..].parse().unwrap());
        }
        else {
            new_vec.push(stone * 2024);
        }
        
    }

    new_vec
}

fn depth_blink(stone: u64, curr_depth: u32, maximum_depth: u32) -> u32 {

}

pub fn part_one(input: &str) -> Option<u32> {
    let mut stones: Vec<u64> = input.split(' ').filter_map(|stone| stone.parse::<u64>().ok()).collect();

    for _ in 0..75 {
        stones = blink(stones);
    }

    Some(stones.len() as u32)
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
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
