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
    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let num_cols = grid[0].len();
    let max_digits = grid.len() - 1;
    (0..num_cols)
        .rev()
        .fold((0, vec![]), |(sum, mut numbers), i| {
            // Get digits in column
            let digits: Vec<char> = grid[..max_digits]
                .iter()
                .filter_map(|row| if row[i] != ' ' { Some(row[i]) } else { None })
                .collect_vec();
            if !digits.is_empty() {
                let n = digits
                    .iter()
                    .fold(0, |acc, d| acc * 10 + d.to_digit(10).unwrap() as u64);
                numbers.push(n);
            }
            if grid.last().unwrap()[i] == '+' {
                (sum + numbers.iter().sum::<u64>(), vec![])
            } else if grid.last().unwrap()[i] == '*' {
                (sum + numbers.iter().product::<u64>(), vec![])
            } else {
                (sum, numbers)
            }
        })
        .0
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");

    aoc2025_utils::timeit_u64("Part 1", || part1(input));
    aoc2025_utils::timeit_u64("Part 2", || part2(input));
    Ok(())
}
