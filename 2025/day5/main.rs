use itertools::Itertools;

fn parse_input(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let (ranges_str, ids_str) = input.split_once("\n\n").unwrap();
    let ranges = ranges_str
        .lines()
        .map(|l| {
            let (a, b) = l.split_once("-").unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect_vec();
    let ids = ids_str.lines().map(|l| l.parse().unwrap()).collect_vec();
    (ranges, ids)
}

fn part1(input: &str) -> u32 {
    let (ranges, ids) = parse_input(input);
    //println!("{ranges:?} {ids:?}");
    ids.iter()
        .filter(|id| ranges.iter().any(|(start, end)| *id >= start && *id <= end))
        .count() as u32
}

fn part2(input: &str) -> u32 {
    0
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");

    aoc2025_utils::timeit("Part 1", || part1(input));
    //aoc2025_utils::timeit("Part 2", || part2(input));
    Ok(())
}
