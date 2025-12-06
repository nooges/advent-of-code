use itertools::Itertools;

enum Op {
    Add,
    Multiply,
}

fn parse_input(input: &str) -> (Vec<Vec<u64>>, Vec<Op>) {
    let lines = input.lines().collect_vec();
    let operators = lines
        .last()
        .unwrap()
        .chars()
        .filter_map(|c| match c {
            '+' => Some(Op::Add),
            '*' => Some(Op::Multiply),
            _ => None,
        })
        .collect_vec();
    let values = lines[..lines.len() - 1]
        .iter()
        .map(|s| aoc2025_utils::extract_u64s(s))
        .collect_vec();
    (values, operators)
}

fn part1(input: &str) -> u64 {
    let (values, operators) = parse_input(input);
    (0..operators.len())
        .map(|i| {
            let vals: Vec<u64> = values.iter().map(|v| v[i]).collect();
            match operators[i] {
                Op::Add => vals.iter().sum::<u64>(),
                Op::Multiply => vals.iter().product::<u64>(),
            }
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    0
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");

    aoc2025_utils::timeit_u64("Part 1", || part1(input));
    //aoc2025_utils::timeit_u64("Part 2", || part2(input));
    Ok(())
}
