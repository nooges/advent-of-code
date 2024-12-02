fn is_safe_report(report: &[u32]) -> bool {
    let diffs = report
        .windows(2)
        .map(|p| (p[1] - p[0]) as i32)
        .collect::<Vec<_>>();
    diffs.iter().all(|n| (1..=3).contains(n)) || diffs.iter().all(|n| (-3..=-1).contains(n))
}

fn is_safe_dampened_report(report: &[u32]) -> bool {
    is_safe_report(report)
        || (0..report.len()).any(|i| is_safe_report(&[&report[..i], &report[i + 1..]].concat()))
}

fn part1(reports: &[Vec<u32>]) -> u32 {
    reports.iter().filter(|row| is_safe_report(row)).count() as u32
}

fn part2(reports: &[Vec<u32>]) -> u32 {
    reports
        .iter()
        .filter(|row| is_safe_dampened_report(row))
        .count() as u32
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");
    let reports: Vec<Vec<u32>> = input.lines().map(aoc2024_utils::extract_u32s).collect();
    aoc2024_utils::timeit("Part 1", || part1(&reports));
    aoc2024_utils::timeit("Part 2", || part2(&reports));
    Ok(())
}
