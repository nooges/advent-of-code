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
    println!("{ranges:?}");

    let mut i = 1;
    let mut a = ranges[0];
    let mut sum = 0;
    loop {
        let b = ranges[i];
        if a.1 >= b.0 {
            if a.1 < b.1 {
                a = (a.0, b.1);
            }
        } else {
            sum += a.1 - a.0 + 1;
            a = b;
        }
        i += 1;
        println!("i: {i}");
        if i >= ranges.len() {
            sum += a.1 - a.0 + 1;
            break;
        }
    }
    sum
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");

    aoc2025_utils::timeit("Part 1", || part1(input));
    aoc2025_utils::timeit_u64("Part 2", || part2(input));
    Ok(())
}

// Low: 348512121918910
