use itertools::Itertools;

fn parse_input(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let (ranges_str, ids_str) = input.split_once("\n\n").unwrap();
    let ranges = ranges_str
        .lines()
        .map(|l| {
            let (a, b) = l.split_once("-").unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .sorted()
        .collect_vec();
    let ids = ids_str.lines().map(|l| l.parse().unwrap()).collect_vec();
    (ranges, ids)
}

fn part1(input: &str) -> u32 {
    let (ranges, ids) = parse_input(input);
    ids.iter()
        .filter(|id| ranges.iter().any(|(start, end)| *id >= start && *id <= end))
        .count() as u32
}

fn part2(input: &str) -> u64 {
    let (ranges, _) = parse_input(input);

    let (sum, a) = ranges[1..].iter().fold((0, ranges[0]), |(acc, a), b| {
        if a.1 >= b.0 {
            if a.1 < b.1 {
                (acc, (a.0, b.1)) // partial overlap
            } else {
                (acc, a) // b is fully contained in a
            }
        } else {
            (acc + a.1 - a.0 + 1, *b) // no overlap
        }
    });
    sum + a.1 - a.0 + 1
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");

    aoc2025_utils::timeit("Part 1", || part1(input));
    aoc2025_utils::timeit_u64("Part 2", || part2(input));
    Ok(())
}

// Low: 348512121918910
