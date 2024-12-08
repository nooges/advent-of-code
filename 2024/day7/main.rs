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
    a * 10u64.pow(b.ilog10() + 1) + b
}

fn equation_possible(
    test_value: u64,
    operands: &[u64],
    possible_ops: &[fn(u64, u64) -> u64],
) -> bool {
    fn helper(
        test_value: u64,
        result: u64,
        operands: &[u64],
        possible_ops: &[fn(u64, u64) -> u64],
    ) -> bool {
        if result > test_value {
            return false;
        } else if operands.is_empty() {
            return result == test_value;
        }
        possible_ops.iter().any(|op| {
            helper(
                test_value,
                op(result, operands[0]),
                &operands[1..],
                possible_ops,
            )
        })
    }

    helper(test_value, operands[0], &operands[1..], possible_ops)
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
