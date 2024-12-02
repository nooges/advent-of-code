//use itertools::Itertools;

fn is_safe_report(report: &[i32]) -> bool {
    report.iter().all(|n| (1..=3).contains(n)) || report.iter().all(|n| (-3..=-1).contains(n))
}

fn part1(reports: &[Vec<u32>]) -> u32 {
    reports
        .into_iter()
        .map(|row| {
            row.windows(2)
                .map(|p| p[1] as i32 - p[0] as i32)
                .collect::<Vec<_>>()
        })
        .filter(|row| is_safe_report(row))
        .count() as u32
}

fn part2(reports: &[Vec<u32>]) -> u32 {
    0
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");
    let reports: Vec<Vec<u32>> = input.lines().map(aoc2024_utils::extract_u32s).collect();
    aoc2024_utils::timeit("Part 1", || part1(&reports));
    //aoc2024_utils::timeit("Part 2", || part2(&pairs));
    Ok(())
}
