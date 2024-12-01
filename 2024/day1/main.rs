use itertools::Itertools;

fn dist(a: i32, b: i32) -> u32 {
    (a - b).unsigned_abs()
}

fn part1(pairs: &Vec<Vec<i32>>) -> u32 {
    let col1: Vec<i32> = pairs.iter().map(|p| p[0]).sorted().collect();
    let col2: Vec<i32> = pairs.iter().map(|p| p[1]).sorted().collect();

    col1.iter()
        .zip(col2.iter())
        .map(|(a, b)| dist(*a, *b))
        .sum()
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");
    let pairs = input
        .lines()
        .map(|line| aoc2024_utils::extract_i32s(line))
        .collect();
    aoc2024_utils::timeit("Part 1", || part1(&pairs));
    Ok(())
}
