use bitvec::prelude::*;
use good_lp::microlp;
use good_lp::{constraint, default_solver, variable, variables, Expression, Solution, SolverModel};
use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
struct Machine {
    indicator: BitVec,
    buttons: Vec<Vec<u32>>,
    buttons_bv: Vec<BitVec>,
    joltages: Vec<u32>,
}

fn parse_input(input: &str) -> Vec<Machine> {
    input
        .lines()
        .map(|l| {
            let re = Regex::new(r"^\[(.*?)\]\s+(.*?)\s+\{(.*?)\}$").unwrap();
            let caps = re.captures(l).unwrap();

            let ind_str = &caps[1];
            let buttons_str = &caps[2];
            let jolt_str = &caps[3];

            let mut indicator = bitvec![0; 16];
            ind_str.chars().enumerate().for_each(|(i, c)| {
                if c == '#' {
                    indicator.set(i, true);
                }
            });

            let re = Regex::new(r"\(([^)]*)\)").unwrap();
            let buttons = re
                .captures_iter(buttons_str)
                .map(|c| aoc2025_utils::extract_u32s(c.get(1).unwrap().as_str()))
                .collect_vec();
            let buttons_bv = buttons
                .iter()
                .map(|g| {
                    let mut button = bitvec![0; 16];
                    g.iter().for_each(|n| button.set(*n as usize, true));
                    button
                })
                .collect_vec();

            let joltages = aoc2025_utils::extract_u32s(jolt_str);
            Machine {
                indicator,
                buttons,
                buttons_bv,
                joltages,
            }
        })
        .collect_vec()
}

fn fewest_presses_indicator(machine: &Machine) -> u64 {
    let mut presses = 1;
    loop {
        if machine.buttons_bv.iter().combinations(presses).any(|c| {
            c.iter()
                .fold(machine.indicator.clone(), |acc, button| acc ^ *button)
                == bitvec![0; 16]
        }) {
            return presses as u64;
        }
        presses += 1;
    }
}

fn fewest_presses_joltage(machine: &Machine) -> u64 {
    let num_cols = machine.buttons.len();
    let mut A = vec![vec![0; machine.buttons.len()]; machine.joltages.len()];
    machine.buttons.iter().enumerate().for_each(|(c, b)| {
        b.iter().for_each(|r| {
            A[*r as usize][c] = 1;
        });
    });

    let upper_bound = 1000;

    let mut vars = variables!();
    let x: Vec<_> = (0..num_cols)
        .map(|_| vars.add(variable().integer().min(0).max(upper_bound)))
        .collect();

    let objective: Expression = x.iter().cloned().sum();

    let constraints: Vec<_> = A
        .iter()
        .zip(machine.joltages.iter())
        .map(|(row, &rhs)| {
            let expr: Expression = row
                .iter()
                .zip(x.iter())
                .map(|(&a, &xi)| a as f64 * xi)
                .sum();
            constraint!(expr == rhs as f64)
        })
        .collect();

    let problem = vars
        .minimise(objective.clone())
        .using(microlp)
        .with_all(constraints);

    let solution = problem.solve().unwrap();
    solution.eval(&objective) as u64
}

fn part1(input: &str) -> u64 {
    let machines = parse_input(input);
    machines.iter().map(fewest_presses_indicator).sum()
}

fn part2(input: &str) -> u64 {
    let machines = parse_input(input);
    machines.iter().map(fewest_presses_joltage).sum()
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");

    aoc2025_utils::timeit_u64("Part 1", || part1(input));
    aoc2025_utils::timeit_u64("Part 2", || part2(input));
    Ok(())
}
