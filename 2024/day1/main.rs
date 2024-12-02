use itertools::Itertools;

fn part1(pairs: &[Vec<u32>]) -> u32 {
    let col1: Vec<u32> = pairs.iter().map(|p| p[0]).sorted().collect();
    let col2: Vec<u32> = pairs.iter().map(|p| p[1]).sorted().collect();

    col1.iter()
        .zip(col2.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum()
}

fn similarity(n: u32, v: &[u32]) -> u32 {
    v.iter().filter(|&x| *x == n).count() as u32 * n
}

fn part2(pairs: &[Vec<u32>]) -> u32 {
    let col2: Vec<u32> = pairs.iter().map(|p| p[1]).collect();
    pairs.iter().map(|p| similarity(p[0], &col2)).sum()
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");
    let pairs: Vec<Vec<u32>> = input.lines().map(aoc2024_utils::extract_u32s).collect();
    aoc2024_utils::timeit("Part 1", || part1(&pairs));
    aoc2024_utils::timeit("Part 2", || part2(&pairs));
    Ok(())
}
