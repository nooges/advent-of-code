use faer::prelude::*;
//use itertools::Itertools;

fn part1(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|m| {
            m.lines()
                .map(aoc2024_utils::extract_u32s)
                .collect::<Vec<_>>()
        })
        .map(|m| {
            let a = Mat::from_fn(2, 2, |i, j| m[j][i] as f64);
            let b = Mat::from_fn(2, 1, |i, _| m[2][i] as f64);
            let plu = a.partial_piv_lu();
            let x = plu.solve(&b);
            (x.get(0, 0).round() as u32, x.get(1, 0).round() as u32, m)
        })
        .filter(|(a, b, m)| {
            a * m[0][0] + b * m[1][0] == m[2][0] && a * m[0][1] + b * m[1][1] == m[2][1]
        })
        .map(|(a, b, _)| 3 * a + b)
        .sum()
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");

    aoc2024_utils::timeit("Part 1", || part1(&input));
    //aoc2024_utils::timeit("Part 2", || part2(&input));
    Ok(())
}
