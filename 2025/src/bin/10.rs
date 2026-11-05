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
struct Button(u16);

impl FromStr for Button {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner = delimited(s, '(', ')')?;

        let mut mask: u16 = 0;
        inner.split(',').for_each(|num| {
            mask |= 1 << num.parse::<u16>().expect("Invalid Number");
        });

        Ok(Button(mask))
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

fn turn_on_machine(machine: &Machine) -> Option<u64> {
    let mut seen: HashSet<u16> = HashSet::new();
    let mut queue: VecDeque<(u64, u16)> = VecDeque::new();
    queue.push_back((0, 0));
    seen.insert(0);

    while let Some((depth, state)) = queue.pop_front() {
        if state == machine.indicator.0 {
            return Some(depth);
        }
        for btn in &machine.buttons {
            let next = state ^ btn.0;
            if seen.contains(&next) {
                continue;
            }
            seen.insert(next);
            queue.push_back((depth + 1, state ^ btn.0));
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<u64> {
    // [Indicator] (wiring schematics) {joltage reqs}
    // The order in which you press two buttons does not matter
    // Pressing the same button twice in a row will never be optimal
    input
        .lines()
        .map(|line| {
            let machine = line.parse::<Machine>().expect("Invalid");
            turn_on_machine(&machine)
        })
        .sum()
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
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
