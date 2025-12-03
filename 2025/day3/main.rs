use itertools::Itertools;

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect()
}

fn find_first_max(v: &[u32]) -> (usize, u32) {
    let (idx, &val) = v
        .iter()
        .enumerate()
        .rev() // Reverse because max_by_key finds the last max value
        .max_by_key(|(_, val)| *val)
        .unwrap();
    (idx, val)
}

fn solve(data: Vec<Vec<u32>>, n: usize) -> u64 {
    data.iter()
        .map(|v| {
            let len = v.len();
            (0..n)
                .rev()
                .fold((0, 0), |(acc, pos), i| {
                    let (max_i, d) = find_first_max(&v[pos..len - i]);
                    (acc * 10 + d as u64, pos + max_i + 1)
                })
                .0
        })
        .sum()
}

fn part1(input: &str) -> u64 {
    let data = parse_input(input);
    solve(data, 2)
}

fn part2(input: &str) -> u64 {
    let data = parse_input(input);
    solve(data, 12)
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");

    aoc2025_utils::timeit_u64("Part 1", || part1(input));
    aoc2025_utils::timeit_u64("Part 2", || part2(input));
    Ok(())
}
