advent_of_code::solution!(6);

fn reduce_spaces(input: &str) -> String {
    let mut result = String::new();
    let mut last_char_was_space = false;

    for c in input.chars() {
        if c.is_whitespace() {
            if !last_char_was_space {
                result.push(' ');
            }
            last_char_was_space = true;
        } else {
            result.push(c);
            last_char_was_space = false;
        }
    }

    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();

    let mut distances: Vec<u32> = vec![];
    let mut times: Vec<u32> = vec![];

    for line in lines {
        let parts: Vec<&str> = line.split(":").collect();
        match parts[0] {
            "Time" => {
                let y = reduce_spaces(parts[1].trim());
                let parsed_times: Vec<u32> = y
                    .split(" ")
                    .map(|num| num.parse().unwrap())
                    .collect();

                times.extend(parsed_times);
            }
            "Distance" => {
                let y = reduce_spaces(parts[1].trim());
                let parsed_distances: Vec<u32> = y
                    .split(" ")
                    .map(|num| num.parse().unwrap())
                    .collect();

                distances.extend(parsed_distances);
            }
            _ => panic!("Wrong input!")
        }
    }

    let result = times
        .into_iter()
        .zip(distances)
        .map(|(time, target_distance)| {
            (0..time)
                .filter_map(|speed| {
                    let temp_distance = (time - speed) * speed;
                    (temp_distance > target_distance).then_some(temp_distance)
                })
                .count()
        })
        .product::<usize>();

    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();

    let mut distance: u64 = 0;
    let mut time: u64 = 0;

    for line in lines {
        let parts: Vec<&str> = line.split(":").collect();
        match parts[0] {
            "Time" => {
                let y = parts[1].trim().replace(" ", "");
                time = y.parse().unwrap();
            }
            "Distance" => {
                let y = parts[1].trim().replace(" ", "");
                distance = y.parse().unwrap();
            }
            _ => panic!("Wrong input!")
        }
    }

    let total = (0..time).filter_map(|speed| {
        let temp_distance = (time - speed) * speed;
        (temp_distance > distance).then_some(temp_distance)
    }).count();

    Some(total as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
