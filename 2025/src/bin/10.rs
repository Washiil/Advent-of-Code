use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

advent_of_code::solution!(10);

fn delimited(s: &str, open: char, close: char) -> Result<&str, String> {
    s.strip_prefix(open)
        .and_then(|s| s.strip_suffix(close))
        .ok_or_else(|| format!("expected {open}...{close}, got {s:?}"))
}

#[derive(Debug)]
struct Indicator(u16);

impl FromStr for Indicator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner = delimited(s, '[', ']')?;

        let mut mask: u16 = 0;
        for (i, c) in inner.chars().enumerate() {
            if c == '#' {
                // bit shift 1 to the left by i places
                mask |= 1 << i;
            }
        }

        Ok(Indicator(mask))
    }
}

#[derive(Debug)]
struct Button(Vec<usize>);

impl Button {
    fn mask(&self) -> u16 {
        self.0.iter().fold(0, |mask, &i| mask | (1 << i))
    }
}

impl FromStr for Button {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        delimited(s, '(', ')')?
            .split(',')
            .map(|n| {
                n.parse::<usize>()
                    .map_err(|e| format!("invalid number {n:?}: {e}"))
            })
            .collect::<Result<Vec<_>, _>>()
            .map(Button)
    }
}

#[derive(Debug)]
struct Joltage(Vec<u16>);

impl FromStr for Joltage {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner = delimited(s, '{', '}')?;

        inner
            .split(',')
            .map(|n| {
                n.parse::<u16>()
                    .map_err(|e| format!("invalid number {n:?}: {e}"))
            })
            .collect::<Result<Vec<_>, _>>()
            .map(Joltage)
    }
}

struct Machine {
    indicator: Indicator,
    buttons: Vec<Button>,
    joltage: Joltage,
}

impl std::str::FromStr for Machine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace();

        let indicator = tokens.next().ok_or("missing indicators")?.parse()?;
        let joltage = tokens.next_back().ok_or("missing joltage")?.parse()?;
        let buttons = tokens.map(str::parse).collect::<Result<_, _>>()?;

        Ok(Machine {
            indicator,
            buttons,
            joltage,
        })
    }
}

fn constraint_matrix(buttons: &[Button], targets: &[u16]) -> Vec<Vec<i64>> {
    let rows = targets.len();
    let cols = buttons.len();
    let mut matrix = vec![vec![0; cols + 1]; rows];

    for (col, btn) in buttons.iter().enumerate() {
        for &row in &btn.0 {
            matrix[row][col] = 1;
        }
    }

    for (row, &target) in targets.iter().enumerate() {
        matrix[row][cols] = i64::from(target);
    }

    matrix
}

fn reduce(matrix: &mut [Vec<i64>]) -> Vec<usize> {
    let rows = matrix.len();
    let cols = matrix.first().map_or(0, |row| row.len() - 1);

    let mut pivots = Vec::new();
    for col in 0..cols {
        let r = pivots.len();

        if r == rows {
            break;
        }

        let Some(found) = (r..rows).find(|&i| matrix[i][col] != 0) else {
            continue;
        };

        matrix.swap(r, found);
        clear_column(matrix, r, col);
        pivots.push(col);
    }

    pivots
}

fn clear_column(matrix: &mut [Vec<i64>], pivot_row: usize, col: usize) {
    let pivot = matrix[pivot_row].clone();
    let p = pivot[col];

    for (i, row) in matrix.iter_mut().enumerate() {
        let x = row[col];

        // Skip the pivot row and cleared rows
        if i == pivot_row || x == 0 {
            continue;
        }

        // Perform the row * p − pivot * x simplification as on paper
        for (value, &pivot_value) in row.iter_mut().zip(&pivot) {
            *value = *value * p - pivot_value * x;
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut seen: HashSet<u16> = HashSet::new();
    let mut queue: VecDeque<(u64, u16)> = VecDeque::new();
    input
        .lines()
        .map(|line| {
            let machine = line.parse::<Machine>().expect("Invalid");
            queue.clear();
            seen.clear();
            queue.push_back((0, 0));
            seen.insert(0);

            while let Some((depth, state)) = queue.pop_front() {
                if state == machine.indicator.0 {
                    return Some(depth);
                }
                for btn in &machine.buttons {
                    let next = state ^ btn.mask();
                    if seen.contains(&next) {
                        continue;
                    }
                    seen.insert(next);
                    queue.push_back((depth + 1, next));
                }
            }

            None
        })
        .sum()
}

fn search(
    matrix: &[Vec<i64>],
    pivots: &[usize],
    free: &[usize],
    limit: i64,
    presses: &mut [i64],
    depth: usize,
) -> Option<i64> {
    if depth < free.len() {
        return (0..=limit)
            .filter_map(|value| {
                presses[free[depth]] = value;
                search(matrix, pivots, free, limit, presses, depth + 1)
            })
            .min();
    }

    for (row, &col) in matrix.iter().zip(pivots) {
        let target = row[row.len() - 1];
        let free_terms: i64 = free.iter().map(|&f| row[f] * presses[f]).sum();
        let remaining = target - free_terms;
        let pivot = row[col];

        if remaining % pivot != 0 || remaining / pivot < 0 {
            return None;
        }
        presses[col] = remaining / pivot;
    }

    Some(presses.iter().sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    input
        .lines()
        .map(|line| {
            let machine = line.parse::<Machine>().expect("Invalid");
            let targets = &machine.joltage.0;
            let buttons = machine.buttons.len();

            let mut matrix = constraint_matrix(&machine.buttons, targets);
            let pivots = reduce(&mut matrix);

            // Every button without a pivot is free
            let free: Vec<usize> = (0..buttons).filter(|c| !pivots.contains(c)).collect();

            // No button needs more presses than the largest target
            let limit = i64::from(*targets.iter().max()?);

            let mut presses = vec![0; buttons];
            let total = search(&matrix, &pivots, &free, limit, &mut presses, 0)?;
            u64::try_from(total).ok()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
