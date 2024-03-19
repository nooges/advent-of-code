use itertools::Itertools;
use std::fs;

fn parse_input(input: &str) -> Vec<u32> {
    utils::extract_u32s(input)
}

fn num_combos(containers: Vec<u32>, n: usize) -> u32 {
    containers
        .into_iter()
        .combinations(n)
        .filter(|v| v.iter().sum::<u32>() == 150)
        .count() as u32
}

fn part1(input: &str) -> u32 {
    let containers = parse_input(input);

    (0..containers.len())
        .map(|n| num_combos(containers.clone(), n + 1))
        .sum()
}

fn part2(input: &str) -> u32 {
    0
}

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    utils::timeit("Part 1", || part1(&input));
    utils::timeit("Part 2", || part2(&input));

    Ok(())
}
