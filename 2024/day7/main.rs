use itertools::Itertools;
use std::ops::{Add, Mul};

#[derive(Debug)]
struct Equation {
    test_value: u64,
    items: Vec<u64>,
}

fn parse_input(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|l| {
            let (h, t) = l.split_once(": ").unwrap();
            Equation {
                test_value: h.parse().unwrap(),
                items: aoc2024_utils::extract_u64s(t),
            }
        })
        .collect_vec()
}

fn equation_possible(test_value: u64, items: &[u64]) -> bool {
    let possible_ops: [fn(u64, u64) -> u64; 2] = [u64::add, u64::mul];
    let combinations = std::iter::repeat(possible_ops)
        .take(items.len() - 1)
        .multi_cartesian_product();

    for combination in combinations {
        let result = combination
            .into_iter()
            .zip(items.iter().skip(1))
            .fold(items[0], |acc, (op, item)| op(acc, *item));
        if result == test_value {
            return true;
        }
    }

    false
}

fn part1(input: &str) -> u64 {
    let equations = parse_input(input);
    equations
        .iter()
        .filter(|eq| equation_possible(eq.test_value, &eq.items))
        .map(|eq| eq.test_value)
        .sum()
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");
    aoc2024_utils::timeit_u64("Part 1", || part1(input));
    //aoc2024_utils::timeit("Part 2", || part2(input));
    Ok(())
}
