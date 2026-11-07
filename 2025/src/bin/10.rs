use good_lp::{Expression, Solution, SolverModel, constraint, microlp, variable, variables};
use std::collections::{HashSet, VecDeque};
use std::fmt::Display;
use std::str::FromStr;

advent_of_code::solution!(10);

type LightMask = u16;

struct Button {
    wiring: Vec<usize>,
}

impl Button {
    fn light_mask(&self) -> LightMask {
        self.wiring.iter().fold(0, |mask, &i| mask | (1 << i))
    }

    fn affects(&self, index: usize) -> bool {
        self.wiring.contains(&index)
    }
}

struct Machine {
    target_lights: LightMask,
    buttons: Vec<Button>,
    target_joltages: Vec<u16>,
}

fn parse_machines(input: &str) -> impl Iterator<Item = Machine> + '_ {
    input
        .lines()
        .map(|line| line.parse().expect("invalid machine"))
}

impl FromStr for Machine {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut tokens = line.split_whitespace();
        let lights = tokens.next().ok_or("missing indicator lights")?;
        let joltages = tokens.next_back().ok_or("missing joltages")?;
        // Everything left between the first and last token is a button.
        let buttons = tokens.map(parse_button).collect::<Result<_, _>>()?;

        Ok(Machine {
            target_lights: parse_lights(lights)?,
            buttons,
            target_joltages: parse_list(unwrap_brackets(joltages, '{', '}')?)?,
        })
    }
}

/// `[.##.]` -> bits 1 and 2 set.
fn parse_lights(s: &str) -> Result<LightMask, String> {
    let mask = unwrap_brackets(s, '[', ']')?
        .chars()
        .enumerate()
        .filter(|&(_, c)| c == '#')
        .fold(0, |mask, (i, _)| mask | (1 << i));
    Ok(mask)
}

/// `(1,3)` -> a button wired to indices 1 and 3.
fn parse_button(s: &str) -> Result<Button, String> {
    let wiring = parse_list(unwrap_brackets(s, '(', ')')?)?;
    Ok(Button { wiring })
}

/// Strips `open` and `close` from the ends of `s`.
fn unwrap_brackets(s: &str, open: char, close: char) -> Result<&str, String> {
    s.strip_prefix(open)
        .and_then(|inner| inner.strip_suffix(close))
        .ok_or_else(|| format!("expected {open}...{close}, got {s:?}"))
}

/// Parses a comma-separated list such as `3,5,4,7`.
fn parse_list<T>(s: &str) -> Result<Vec<T>, String>
where
    T: FromStr,
    T::Err: Display,
{
    s.split(',')
        .map(|n| n.parse().map_err(|e| format!("invalid number {n:?}: {e}")))
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    parse_machines(input)
        .map(|machine| fewest_presses_for_lights(&machine))
        .sum()
}

fn fewest_presses_for_lights(machine: &Machine) -> Option<u64> {
    let button_masks: Vec<LightMask> = machine.buttons.iter().map(Button::light_mask).collect();

    let mut seen: HashSet<LightMask> = HashSet::from([0]);
    let mut queue: VecDeque<(LightMask, u64)> = VecDeque::from([(0, 0)]);

    while let Some((lights, presses)) = queue.pop_front() {
        if lights == machine.target_lights {
            return Some(presses);
        }
        for &mask in &button_masks {
            let next = lights ^ mask;
            if seen.insert(next) {
                queue.push_back((next, presses + 1));
            }
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut total = 0;
    for machine in parse_machines(input) {
        total += fewest_presses_for_joltage(&machine)?;
    }
    Some(total)
}

fn fewest_presses_for_joltage(machine: &Machine) -> Option<u64> {
    let mut vars = variables!();
    let presses = vars.add_vector(variable().integer().min(0), machine.buttons.len());

    let mut total_presses = Expression::from(0);
    for &press_count in &presses {
        total_presses += press_count;
    }

    let mut problem = vars.minimise(total_presses.clone()).using(microlp);

    for (counter, &target) in machine.target_joltages.iter().enumerate() {
        let mut counter_value = Expression::from(0);
        for (button, &press_count) in machine.buttons.iter().zip(&presses) {
            if button.affects(counter) {
                counter_value += press_count;
            }
        }
        problem = problem.with(constraint!(counter_value == f64::from(target)));
    }

    let solution = problem.solve().ok()?;

    Some(solution.eval(total_presses).round() as u64)
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
