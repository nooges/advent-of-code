use bitvec::prelude::*;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
struct Machine {
    indicator: BitVec,
    buttons: Vec<BitVec>,
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
            let groups = re
                .captures_iter(buttons_str)
                .map(|c| aoc2025_utils::extract_u32s(c.get(1).unwrap().as_str()))
                .collect_vec();
            let buttons = groups
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
                joltages,
            }
        })
        .collect_vec()
}

fn fewest_presses(machine: &Machine) -> u64 {
    let mut presses = 1;
    loop {
        if machine.buttons.iter().combinations(presses).any(|c| {
            c.iter()
                .fold(machine.indicator.clone(), |acc, button| acc ^ *button)
                == bitvec![0; 16]
        }) {
            return presses as u64;
        }
        presses += 1;
    }
}

fn part1(input: &str) -> u64 {
    let machines = parse_input(input);
    machines.iter().map(fewest_presses).sum()
}

fn part2(input: &str) -> u64 {
    0
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");

    aoc2025_utils::timeit_u64("Part 1", || part1(input));
    //aoc2025_utils::timeit_u64("Part 2", || part2(input));
    Ok(())
}
