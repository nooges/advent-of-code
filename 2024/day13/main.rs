extern crate nalgebra as na;
use na::{Matrix2, Matrix2x1};

fn is_close(n: f64) -> bool {
    (n - n.round()).abs() < 1e-4
}

fn solve(input: &str, offset: f64) -> u64 {
    input
        .split("\n\n")
        .map(|m| {
            let data = m
                .lines()
                .map(aoc2024_utils::extract_u64s)
                .collect::<Vec<_>>();
            let a = Matrix2::from_fn(|i, j| data[j][i] as f64);
            let b = Matrix2x1::from_fn(|i, _| offset + data[2][i] as f64);
            let x = a.lu().solve(&b).unwrap();
            x.column(0).as_slice().to_vec()
        })
        .filter(|x| is_close(x[0]) && is_close(x[1]))
        .map(|x| 3 * x[0].round() as u64 + x[1].round() as u64)
        .sum()
}

fn part1(input: &str) -> u64 {
    solve(input, 0.0)
}

fn part2(input: &str) -> u64 {
    solve(input, 10000000000000.0)
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");

    aoc2024_utils::timeit_u64("Part 1", || part1(input));
    aoc2024_utils::timeit_u64("Part 2", || part2(input));
    Ok(())
}
