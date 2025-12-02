use itertools::Itertools;

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    input
        .split(',')
        .map(|s| {
            let p = s.split_once('-').unwrap();
            (p.0.parse().unwrap(), p.1.parse().unwrap())
        })
        .collect()
}

fn num_digits(n: u64) -> u32 {
    n.ilog10() + 1
}

fn part1(input: &str) -> u64 {
    let ranges = parse_input(input);
    let possible_bad_ids = (1..=99999).map(|n| n + n * 10_u64.pow(num_digits(n)));
    possible_bad_ids
        .filter(|i| ranges.iter().any(|(start, end)| i >= start && i <= end))
        .sum()
}

fn repeat_digits(n: u64, repeats: usize) -> Option<u64> {
    if num_digits(n) * repeats as u32 > 10 {
        return None;
    }
    Some(n.to_string().repeat(repeats).parse().unwrap())
}

fn part2(input: &str) -> u64 {
    let ranges = parse_input(input);
    let possible_bad_ids = (1..=99999).flat_map(|n| (2..=7).flat_map(move |r| repeat_digits(n, r)));
    possible_bad_ids
        .filter(|i| ranges.iter().any(|(start, end)| i >= start && i <= end))
        .unique()
        .sum()
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");

    aoc2025_utils::timeit_u64("Part 1", || part1(input));
    aoc2025_utils::timeit_u64("Part 2", || part2(input));
    Ok(())
}
