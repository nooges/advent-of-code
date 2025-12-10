use itertools::Itertools;
use rustc_hash::FxHashSet as HashSet;

fn parse_input(input: &str) -> HashSet<(u32, u32)> {
    input
        .lines()
        .map(|l| {
            let (a, b) = l.split_once(",").unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect()
}

fn part1(input: &str) -> u64 {
    let points = parse_input(input);
    points
        .iter()
        .combinations(2)
        .map(|combo| {
            let a = combo[0];
            let b = combo[1];
            (((a.0 as i64 - b.0 as i64).abs() + 1) * ((a.1 as i64 - b.1 as i64) + 1).abs()) as u64
        })
        .max()
        .unwrap() as u64
}

fn part2(input: &str) -> u64 {
    // Solved visually via Google Sheet
    0
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");

    aoc2025_utils::timeit_u64("Part 1", || part1(input));
    aoc2025_utils::timeit_u64("Part 2", || part2(input));
    Ok(())
}
