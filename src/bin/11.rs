advent_of_code::solution!(11);

#[derive(Clone)]
struct Point {
    row: u32,
    col: u32
}

impl Point {
    fn new(row: u32, col: u32) -> Point {
        Point { row, col }
    }

    fn manhattan_distance(&self, other: &Point) -> u32 {
        self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
    }
}

fn further_distance(map: &Vec<Vec<char>>, start: &Point, end: &Point) -> usize {
    let y_total = 
        map[0][start.y.min(end.y)..=start.y.max(end.y)]
            .iter()
            .fold(0, |total, val| {
                total + match val {
                    '*' => 100,
                    _ => 1
                }
            });
        
    let x_total =
        map[start.x.min(end.x)..=start.x.max(end.x)]
            .iter()
            .fold(0, |total, val| {
                total + match val[0] {
                    '*' => 100,
                    _ => 1
                }
            });
        
    x_total + y_total
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines: Vec<Vec<char>> = 
        input
            .lines()
            .map(|line| line.chars().collect())
            .collect();
    
    let rows = lines.len();
    let cols = lines[0].len();

    // TODO: Expand Rows
    let mut expansion_rows: Vec<usize> = Vec::new();
    for row in 0..rows {
        if lines[row].iter().all(|x| *x == '.') {
            expansion_rows.push(row);
        }
    }
    // Keep a watch on that index

    let mut index = 0 ;
    for i in expansion_rows {
        lines.insert(i + index, vec!['.'; cols]);
        index += 1;
    }

    let rows = lines.len();
    let cols = lines[0].len();

    // TODO: Expand Coloumns we need to update the index as we add rows
    let mut expansion_cols: Vec<usize> = Vec::new();
    'outer: for col in 0..cols {
        for row in 0..rows {
            if lines[row][col] != '.' {
                continue 'outer;
            }
        }
        expansion_cols.push(col);
    }

    index = 0;
    for i in expansion_cols {
        for row in 0..rows {
            lines[row].insert(i + index, '.');
        }
        index += 1;
    }

    let rows = lines.len();
    let cols = lines[0].len();

    // Get coordinates for every galaxy
    let mut galaxies: Vec<Point> = Vec::new();
    for r in 0..rows {
        for c in 0..cols {
            if lines[r][c] == '#' {
                galaxies.push(Point {
                    x: c,
                    y: r
                });
            }
        }
    }

    let total_galaxies = galaxies.len();
    let mut total_distance = 0;

    for i in 0..total_galaxies {
        for j in i + 1..total_galaxies {
            total_distance += distance(&galaxies[i], &galaxies[j]);
        }
    }

    // TODO: Calculate shortest Path between all galaxy combinations
    Some(total_distance as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines: Vec<Vec<char>> = 
        input
            .lines()
            .map(|line| line.chars().collect())
            .collect();
    
    let rows = lines.len();
    let cols = lines[0].len();

    // TODO: Expand Rows
    let mut expansion_rows: Vec<usize> = Vec::new();
    for row in 0..rows {
        if lines[row].iter().all(|x| *x == '.') {
            expansion_rows.push(row);
        }
    }
    // Keep a watch on that index

    let mut index = 0 ;
    for i in expansion_rows {
        lines.insert(i + index, vec!['*'; cols]);
        index += 1;
    }

    let rows = lines.len();
    let cols = lines[0].len();

    // TODO: Expand Coloumns we need to update the index as we add rows
    let mut expansion_cols: Vec<usize> = Vec::new();
    'outer: for col in 0..cols {
        for row in 0..rows {
            if lines[row][col] == '#' {
                continue 'outer;
            }
        }
        expansion_cols.push(col);
    }

    index = 0;
    for i in expansion_cols {
        for row in 0..rows {
            lines[row].insert(i + index, '*');
        }
        index += 1;
    }

    for l in &lines {
        for c in l {
            print!("{}", c);
        }
        println!();
    }

    let rows = lines.len();
    let cols = lines[0].len();

    // Get coordinates for every galaxy
    let mut galaxies: Vec<Point> = Vec::new();
    for r in 0..rows {
        for c in 0..cols {
            if lines[r][c] == '#' {
                galaxies.push(Point {
                    x: c,
                    y: r
                });
            }
        }
    }

    let total_galaxies = galaxies.len();
    let mut total_distance = 0;


    // Loop over every galaxy
    for i in 0..total_galaxies {
        for j in i + 1..total_galaxies {
            total_distance += further_distance(&lines, &galaxies[i], &galaxies[j]);
        }
    }

    // TODO: Calculate shortest Path between all galaxy combinations
    Some(total_distance as u32)
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
    fn far_distance() {
        let input: Vec<Vec<char>> = 
"....1........
.........2...
3............".lines().map(|line| line.chars().collect()).collect();
        let p1 = Point {
            x: 4,
            y: 0
        };

        let p2 = Point {
            x: 9,
            y: 1
        };

        let result = further_distance(&input, &p1, &p2);
        assert_eq!(result, 105);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8410));
    }
}