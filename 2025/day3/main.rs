use itertools::Itertools;

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect()
}

fn part1(input: &str) -> u32 {
    let data = parse_input(input);
    data.iter()
        .map(|v| {
            let len = v.len();
            let (max_i, max_left) = &v[0..len - 1]
                .iter()
                .enumerate()
                .rev() // Reverse because max_by_key finds the last max value
                .max_by_key(|(_, val)| **val)
                .unwrap();
            let max_right = &v[max_i + 1..].iter().max().unwrap();
            10 * *max_left + *max_right
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    0
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");

    aoc2025_utils::timeit("Part 1", || part1(input));
    //aoc2025_utils::timeit_u64("Part 2", || part2(input));
    Ok(())
}
