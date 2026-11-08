advent_of_code::solution!(2);

fn check_id(id: &str) -> bool {
    let l = id.len();
    if !l.is_multiple_of(2) {
        return false;
    }

    let midpoint = l.div_ceil(2);

    id[..midpoint] == id[midpoint..]
}

pub fn part_one(input: &str) -> Option<u64> {
    /*
    The ranges are separated by commas (,); each range gives its first ID and last ID separated by a dash (-).
    None of the numbers have leading zeroes; 0101 isn't an ID at all. (101 is a valid ID that you would ignore.)
     */
    let id_ranges = input.trim().split(",").collect::<Vec<&str>>();
    let mut output: u64 = 0;

    for r in id_ranges {
        let bounds = r.trim().split('-').collect::<Vec<&str>>();
        if bounds.len() < 2 {
            panic!("Invalid Input Lengths")
        }

        let lower = bounds[0].parse::<u64>().unwrap();
        let upper = bounds[1].parse::<u64>().unwrap();

        for id in lower..upper+1 {
            let str_id = id.to_string();
            if check_id(&str_id) {
                output += id;
            }
        }
    }

    Some(output)
}

fn check_id_2(id: &str) -> bool {
    let len = id.len();

    (1..=(len / 2))
        .filter(|&g| len.is_multiple_of(g))
        .any(|win| {
            let pat = &id[0..win];
            (0..len)
                .step_by(win)
                .all(|x|
                    &id[x..(x+win)] == pat)
            }
        )
}

pub fn part_two(input: &str) -> Option<u64> {
    let id_ranges = input.trim().split(",").collect::<Vec<&str>>();
    let mut output: u64 = 0;

    for r in id_ranges {
        let bounds = r.trim().split('-').collect::<Vec<&str>>();
        if bounds.len() < 2 {
            panic!("Invalid Input Lengths")
        }

        let lower = bounds[0].parse::<u64>().unwrap();
        let upper = bounds[1].parse::<u64>().unwrap();

        for id in lower..upper+1 {
            let str_id = id.to_string();
            if check_id_2(&str_id) {
                output += id;
            }
        }
    }

    Some(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
