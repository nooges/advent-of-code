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
    let (init_problem, first_op) = if grid.last().unwrap()[0] == '+' {
        (0, Op::Add)
    } else {
        (1, Op::Multiply)
    };
    (0..=num_cols)
        .fold(
            (0, init_problem, first_op),
            |(total_sum, curr_problem, curr_op), i| {
                if i == num_cols {
                    return (total_sum + curr_problem, 0, curr_op);
                }
                // Get digits in column
                let digits: Vec<char> = grid[..max_digits]
                    .iter()
                    .filter_map(|row| if row[i] != ' ' { Some(row[i]) } else { None })
                    .collect_vec();
                if digits.is_empty() {
                    if grid.last().unwrap()[i + 1] == '+' {
                        (total_sum + curr_problem, 0, Op::Add)
                    } else {
                        (total_sum + curr_problem, 1, Op::Multiply)
                    }
                } else {
                    let n = digits
                        .iter()
                        .fold(0, |acc, d| acc * 10 + d.to_digit(10).unwrap() as u64);
                    match curr_op {
                        Op::Add => (total_sum, curr_problem + n, curr_op),
                        Op::Multiply => (total_sum, curr_problem * n, curr_op),
                    }
                }
            },
        )
        .0
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");

    aoc2025_utils::timeit_u64("Part 1", || part1(input));
    aoc2025_utils::timeit_u64("Part 2", || part2(input));
    Ok(())
}
