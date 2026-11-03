use std::collections::HashMap;

advent_of_code::solution!(8);

struct Point {
    x: u32,
    y: u32,
    z: u32,
}

type Rank = usize;

fn distance_squared(p1: &Point, p2: &Point) -> u64 {
    let dx = u64::from(p1.x.abs_diff(p2.x));
    let dy = u64::from(p1.y.abs_diff(p2.y));
    let dz = u64::from(p1.z.abs_diff(p2.z));

    dx * dx + dy * dy + dz * dz
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.lines();
    let mut points: Vec<Point> = Vec::new();

    // Parse all of the points
    for line in lines {
        let mut iter = line.split(',');
        let x = iter.next()?.parse().expect("Invalid Input");
        let y = iter.next()?.parse().expect("Invalid Input");
        let z = iter.next()?.parse().expect("Invalid Input");
        points.push(Point { x, y, z })
    }

    let mut distances: Vec<(u64, usize, usize)> = Vec::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            distances.push((distance_squared(&points[i], &points[j]), i, j))
        }
    }

    const JOINS: usize = 1000;
    distances.select_nth_unstable_by_key(JOINS - 1, |&(dist, _, _)| dist);

    let mut circuits = (0..points.len()).collect::<Vec<Rank>>();

    // Now join
    for i in 0..JOINS {
        let (_, p1, p2) = distances[i];
        let shared_circuit = circuits[p1].min(circuits[p2]);
        let c1 = circuits[p1];
        let c2 = circuits[p2];
        for i in 0..circuits.len() {
            if circuits[i] == c1 || circuits[i] == c2 {
                circuits[i] = shared_circuit;
            }
        }
    }

    let mut counts: HashMap<Rank, usize> = HashMap::new();
    for c in circuits {
        *counts.entry(c).or_default() += 1;
    }

    let mut res: Vec<&usize> = counts.values().collect();
    res.sort();
    res.reverse();

    Some((res[0] * res[1] * res[2]) as u64)
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
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
