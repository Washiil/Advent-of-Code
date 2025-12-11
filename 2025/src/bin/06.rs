use std::panic::panic_any;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.lines().collect::<Vec<&str>>();

    let operator_indicies = lines
        .last()?
        .chars()
        .enumerate()
        .filter(|(_, c)| *c != ' ')
        .collect::<Vec<(usize, char)>>();


    let mut equation_results: Vec<u64> = vec![0; operator_indicies.len()];

    lines
        .iter()
        .take_while(|l| !l.contains("*") && !l.contains("+"))
        .for_each(|l| {
            l
                .split_ascii_whitespace()
                .enumerate()
                .for_each(|(i, v)| {
                    match operator_indicies[i].1 {
                        '*' => {
                            if equation_results[i] == 0 {
                                equation_results[i] = 1;
                            }
                            equation_results[i] *= v.parse::<u64>().expect("Invalid Input must be type decimal")
                        }
                        '+' => equation_results[i] += v.parse::<u64>().expect("Invalid Input must be type decimal"),
                        _ => panic!()
                    }
                });
        });


    let sum = equation_results.iter().sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
