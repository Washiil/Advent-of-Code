advent_of_code::solution!(11);


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}

fn find_galaxies(universe: &Vec<Vec<char>>) -> Vec<Point> {
    let mut galaxies: Vec<Point> = Vec::new();

    for row in 0..universe.len() {
        for col in 0..universe[0].len() {
            match universe[row][col] {
                '#' => {
                    galaxies.push(Point::new(col, row))
                }
                '.' => {}
                _ => unreachable!()
            }
        }
    }
    galaxies
}

fn x_expansion(galaxies: &mut Vec<Point>, scalar: usize) {
    let mut sum_expansion = 0;
    let mut last_x = 0;
    for galaxy in galaxies.iter_mut() {
        if galaxy.x != last_x {
            sum_expansion += (galaxy.x - last_x - 1) * (scalar - 1);
            last_x = galaxy.x;
        }

        galaxy.x += sum_expansion;
    }
}

fn y_expansion(galaxies: &mut Vec<Point>, scalar: usize) {
    let mut sum_expansion = 0;
    let mut last_y = 0;
    for galaxy in galaxies.iter_mut() {
        if galaxy.y != last_y {
            sum_expansion += (galaxy.y - last_y - 1) * (scalar - 1);
            last_y = galaxy.y;
        }
        galaxy.y += sum_expansion;
    }
}

fn parse(input: &str, expansion: usize) -> Vec<Point> {
    let universe: Vec<Vec<char>> = 
        input
            .lines()
            .map(|line| line.chars().collect())
            .collect();

    let mut galaxies = find_galaxies(&universe);

    y_expansion(&mut galaxies, expansion);

    galaxies.sort_by_key(|galaxy| galaxy.x);

    x_expansion(&mut galaxies, expansion);

    galaxies
}

pub fn part_one(input: &str) -> Option<u64> {
    let galaxies = parse(input, 2);

    let mut sum: usize = 0;

    for i in 0..galaxies.len() {
        let a = &galaxies[i];
        for b in &galaxies[i + 1..] {
            sum += a.x.abs_diff(b.x) + a.y.abs_diff(b.y);
        }
    }

    Some(sum as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let galaxies = parse(input, 1_000_000);

    let mut sum: usize = 0;

    for i in 0..galaxies.len() {
        let a = &galaxies[i];
        for b in &galaxies[i + 1..] {
            sum += a.x.abs_diff(b.x) + a.y.abs_diff(b.y);
        }
    }

    Some(sum as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8410));
    }
}