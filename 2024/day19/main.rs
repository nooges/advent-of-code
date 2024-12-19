use cached::proc_macro::cached;
use itertools::Itertools;
use rustc_hash::FxHashSet as HashSet;

fn parse(input: &str) -> (HashSet<&str>, Vec<&str>) {
    let (patterns_str, designs_str) = input.split_once("\n\n").unwrap();
    let patterns = HashSet::from_iter(patterns_str.split(", "));
    let designs = designs_str.lines().collect_vec();

    (patterns, designs)
}

#[cached(key = "String", convert = r#"{ design.to_string() }"#)]
fn num_possibilities(design: &str, patterns: &HashSet<&str>) -> u64 {
    if design.is_empty() {
        return 1;
    }
    let possible_prefixes = &patterns
        .iter()
        .filter(|p| design.starts_with(**p))
        .collect_vec();

    possible_prefixes
        .iter()
        .map(|d| num_possibilities(&design[d.len()..], patterns))
        .sum()
}

fn part1(patterns: HashSet<&str>, designs: Vec<&str>) -> u32 {
    designs
        .iter()
        .filter(|design| num_possibilities(design, &patterns) > 0)
        .count() as u32
}

fn part2(patterns: HashSet<&str>, designs: Vec<&str>) -> u64 {
    designs
        .iter()
        .map(|design| num_possibilities(design, &patterns))
        .sum::<u64>()
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");
    let (patterns, designs) = parse(input);

    aoc2024_utils::timeit("Part 1", || part1(patterns.clone(), designs.clone()));
    aoc2024_utils::timeit_u64("Part 2", || part2(patterns.clone(), designs.clone()));
    Ok(())
}
