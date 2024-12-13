use faer::prelude::*;

fn is_close(n: f64) -> bool {
    (n - n.round()).abs() < 1e-4
}

fn solve(input: &str, offset: f64) -> u64 {
    input
        .split("\n\n")
        .map(|m| {
            m.lines()
                .map(aoc2024_utils::extract_u64s)
                .collect::<Vec<_>>()
        })
        .map(|m| {
            let a = Mat::from_fn(2, 2, |i, j| m[j][i] as f64);
            let b = Mat::from_fn(2, 1, |i, _| offset + m[2][i] as f64);
            let plu = a.partial_piv_lu();
            let x = plu.solve(&b);
            (*x.get(0, 0), *x.get(1, 0))
        })
        .filter(|(a, b)| is_close(*a) && is_close(*b))
        .map(|(a, b)| 3 * a.round() as u64 + b.round() as u64)
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
