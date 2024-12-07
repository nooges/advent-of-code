use itertools::Itertools;
use rayon::prelude::*;
use std::ops::{Add, Mul};

#[derive(Debug)]
struct Equation {
    test_value: u64,
    operands: Vec<u64>,
}

fn parse_input(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|l| {
            let (h, t) = l.split_once(": ").unwrap();
            Equation {
                test_value: h.parse().unwrap(),
                operands: aoc2024_utils::extract_u64s(t),
            }
        })
        .collect_vec()
}

fn concat(a: u64, b: u64) -> u64 {
    format!("{}{}", a, b).parse().unwrap()
}

fn equation_possible(
    test_value: u64,
    operands: &[u64],
    possible_ops: &[fn(u64, u64) -> u64],
) -> bool {
    let mut combinations = std::iter::repeat(possible_ops)
        .take(operands.len() - 1)
        .multi_cartesian_product();

    combinations.any(|combination| {
        let mut result = operands[0];
        for (op, item) in combination.iter().zip(operands.iter().skip(1)) {
            result = op(result, *item);
            if result > test_value {
                return false;
            }
        }
        result == test_value
    })
}

fn part1(input: &str) -> u64 {
    let possible_ops: [fn(u64, u64) -> u64; 2] = [u64::add, u64::mul];
    parse_input(input)
        .par_iter()
        .filter(|eq| equation_possible(eq.test_value, &eq.operands, &possible_ops))
        .map(|eq| eq.test_value)
        .sum()
}

fn part2(input: &str) -> u64 {
    let possible_ops: [fn(u64, u64) -> u64; 3] = [u64::add, u64::mul, concat];
    parse_input(input)
        .par_iter()
        .filter(|eq| equation_possible(eq.test_value, &eq.operands, &possible_ops))
        .map(|eq| eq.test_value)
        .sum()
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");
    aoc2024_utils::timeit_u64("Part 1", || part1(input));
    aoc2024_utils::timeit_u64("Part 2", || part2(input));
    Ok(())
}
