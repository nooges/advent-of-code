use itertools::Itertools;
use rustc_hash::FxHashSet as HashSet;

fn parse_input(input: &str) -> Vec<HashSet<usize>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .enumerate()
                .filter_map(|(i, c)| match c {
                    '.' => None,
                    _ => Some(i),
                })
                .collect()
        })
        .collect()
}

fn part1(input: &str) -> u32 {
    let rows = parse_input(input);
    let mut beams: HashSet<usize> = rows[0].clone();
    let mut splits = 0;
    rows[2..].iter().for_each(|row| {
        // For each splitter in row, if beam is there, split and insert into beams
        let splitters_hit = row.iter().filter(|s| beams.contains(s)).collect_vec();
        splits += splitters_hit.len();
        splitters_hit.iter().for_each(|&&s| {
            beams.remove(&s);
            beams.extend([s - 1, s + 1]);
        });
    });
    splits as u32
}

fn part2(input: &str) -> u64 {
    0
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");

    aoc2025_utils::timeit("Part 1", || part1(input));
    //aoc2025_utils::timeit_u64("Part 2", || part2(input));
    Ok(())
}
