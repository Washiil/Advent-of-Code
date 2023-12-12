use std::collections::HashMap;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0;
    let mut game_number = 1;

    let colours: HashMap<&str, u32> = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);

    let lines = input.lines();
    
    for line in lines {
        let data = line
            .split(": ")
            .skip(1)
            .next()
            .unwrap();

        let mut valid = true;
        let draws: Vec<&str> = data.split(";").collect();
        
        'outer: for draw in draws {
            let dice: Vec<&str> = draw.split(",").collect();
            for die in dice {
                let parts: Vec<&str> = die.trim().split(" ").collect();

                let count: u32 = parts[0].parse().unwrap();
                let colour = parts[1];

                match colours.get(colour) {
                    Some(col) => {
                        if &count > col {
                            valid = false;
                            break 'outer;
                        }
                    },
                    None => panic!("Incorrect Input")
                }
            }
        }
        if valid {
            total += game_number;
        }
        game_number += 1
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total = 0;
    let lines = input.lines();

    for line in lines {
        let x = line.strip_prefix("Game ").unwrap();
        let data: Vec<&str> = x.split(":").collect();

        let mut colours: HashMap<&str, u32> = HashMap::from([
            ("red", 0),
            ("green", 0),
            ("blue", 0),
        ]);

        let draws: Vec<&str> = data[1].split(";").collect();
        
        for draw in draws {
            let dice: Vec<&str> = draw.split(",").collect();
            for die in dice {
                let parts: Vec<&str> = die.trim().split(" ").collect();

                let count: u32 = parts[0].parse().unwrap();
                let colour = parts[1];
                
                match colours.get_mut(colour) {
                    Some(col) => {
                        if &count > col {
                            *col = count
                        }
                    },
                    None => panic!("Incorrect Input")
                }
            }
        }
        let product: u32 = colours.values().fold(1, |acc, x| acc * x);
        total += product;
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
