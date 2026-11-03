use std::collections::HashMap;

advent_of_code::solution!(8);

struct Point {
    x: u64,
    y: u64,
    z: u64,
}

impl std::str::FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').map(str::trim).collect();
        let [x, y, z] = parts[..] else {
            return Err(format!("expected 3 fields, got {}: {s:?}", parts.len()));
        };
        Ok(Point {
            x: x.parse().map_err(|e| format!("bad x: {e}"))?,
            y: y.parse().map_err(|e| format!("bad y: {e}"))?,
            z: z.parse().map_err(|e| format!("bad z: {e}"))?,
        })
    }
}

impl Point {
    pub fn dist_from(&self, other: &Point) -> u64 {
        let dx = u64::from(self.x.abs_diff(other.x));
        let dy = u64::from(self.y.abs_diff(other.y));
        let dz = u64::from(self.z.abs_diff(other.z));

        dx * dx + dy * dy + dz * dz
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.lines();
    let points: Vec<Point> = lines
        .map(|line| line.parse().expect("Invalid input"))
        .collect();

    let mut distances: Vec<(u64, usize, usize)> = Vec::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            distances.push((points[i].dist_from(&points[j]), i, j))
        }
    }

    const JOINS: usize = 1000;
    distances.select_nth_unstable_by_key(JOINS - 1, |&(dist, _, _)| dist);

    let mut circuits = (0..points.len()).collect::<Vec<usize>>();

    // Now join
    for (_, p1, p2) in distances {
        let shared_circuit = circuits[p1].min(circuits[p2]);
        let c1 = circuits[p1];
        let c2 = circuits[p2];
        for i in 0..circuits.len() {
            if circuits[i] == c1 || circuits[i] == c2 {
                circuits[i] = shared_circuit;
            }
        }
    }

    let mut counts: HashMap<usize, usize> = HashMap::new();
    for c in circuits {
        *counts.entry(c).or_default() += 1;
    }

    let mut res: Vec<&usize> = counts.values().collect();
    res.sort();
    res.reverse();

    Some((res[0] * res[1] * res[2]) as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.lines();

    let points: Vec<Point> = lines
        .map(|line| line.parse().expect("Invalid input"))
        .collect();

    let mut distances: Vec<(u64, usize, usize)> = Vec::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            distances.push((points[i].dist_from(&points[j]), i, j))
        }
    }

    distances.sort_by_key(|&(dist, _, _)| dist);

    let mut circuits = (0..points.len()).collect::<Vec<usize>>();
    let mut unique = points.len();

    // Now join
    for (_, p1, p2) in distances {
        if circuits[p1] == circuits[p2] {
            continue;
        }
        let shared_circuit = circuits[p1].min(circuits[p2]);
        let c1 = circuits[p1];
        let c2 = circuits[p2];
        for i in 0..circuits.len() {
            if circuits[i] == c1 || circuits[i] == c2 {
                circuits[i] = shared_circuit;
            }
        }

        unique -= 1;
        if unique == 1 {
            return Some(points[p1].x * points[p2].x);
        }
    }

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
        assert_eq!(result, Some(25272));
    }
}
