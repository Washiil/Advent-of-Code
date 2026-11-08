use std::collections::HashSet;

advent_of_code::solution!(9);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: u64,
    y: u64,
}

impl Point {
    pub fn dist(&self, other: &Point) -> u64 {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
}

impl std::str::FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').map(str::trim).collect();
        let [x, y] = parts[..] else {
            return Err(format!("expected 2 fields, got {}: {s:?}", parts.len()));
        };
        Ok(Point {
            x: x.parse().map_err(|e| format!("bad x: {e}"))?,
            y: y.parse().map_err(|e| format!("bad y: {e}"))?,
        })
    }
}

fn segment_is_valid(src: Point, dest: Point, valid: &HashSet<(u64, u64)>) -> bool {
    let min_x = src.x.min(dest.x);
    let max_x = src.x.max(dest.x);
    let min_y = src.y.min(dest.y);
    let max_y = src.y.max(dest.y);

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if !valid.contains(&(x, y)) {
                return false;
            }
        }
    }

    true
}

fn rect_is_valid(a: Point, b: Point, valid: &HashSet<(u64, u64)>) -> bool {
    // The four corners, going around the rectangle
    let top_left = Point {
        x: a.x.min(b.x),
        y: a.y.min(b.y),
    };
    let top_right = Point {
        x: a.x.max(b.x),
        y: a.y.min(b.y),
    };
    let bottom_right = Point {
        x: a.x.max(b.x),
        y: a.y.max(b.y),
    };
    let bottom_left = Point {
        x: a.x.min(b.x),
        y: a.y.max(b.y),
    };

    segment_is_valid(top_left, top_right, valid)
        && segment_is_valid(top_right, bottom_right, valid)
        && segment_is_valid(bottom_right, bottom_left, valid)
        && segment_is_valid(bottom_left, top_left, valid)
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.lines();
    let points: Vec<Point> = lines.map(|p| p.parse().expect("Invalid Input")).collect();

    let mut max_area = 0;
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            max_area = max_area.max(points[i].dist(&points[j]));
        }
    }

    Some(max_area)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.lines();
    let points: Vec<Point> = lines.map(|p| p.parse().expect("Invalid Input")).collect();

    let mut valid: HashSet<(u64, u64)> = HashSet::new();

    for pair in points.windows(2) {
        let src = pair[0];
        let dest = pair[1];

        // Guaranteed to share a row or column
        let (min_x, max_x) = (src.x.min(dest.x), src.x.max(dest.x));
        let (min_y, max_y) = (src.y.min(dest.y), src.y.max(dest.y));

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                valid.insert((x, y));
            }
        }
    }

    let mut max_area = 0;
    for pair in points.windows(2) {
        let src = pair[0];
        let dest = pair[1];

        if !rect_is_valid(src, dest, &valid) {
            continue;
        }

        max_area = (src.x.abs_diff(dest.y) + 1) * (src.y.abs_diff(dest.y) + 1);
    }

    Some(max_area)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let p1 = Point { x: 2, y: 5 };
        let p2 = Point { x: 11, y: 1 };
        assert_eq!(p1.dist(&p2), 50)
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
