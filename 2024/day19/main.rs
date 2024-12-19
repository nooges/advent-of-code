use std::cmp::min;

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
fn num_possibilities_old(design: &str, patterns: &HashSet<&str>) -> u64 {
    if design.is_empty() {
        return 1;
    }
    patterns
        .iter()
        .filter(|p| design.starts_with(**p))
        .map(|d| num_possibilities_old(&design[d.len()..], patterns))
        .sum()
}

#[cached(key = "String", convert = r#"{ design.to_string() }"#)]
fn num_possibilities(design: &str, patterns: &HashSet<&str>, max_len: usize) -> u64 {
    let mut poss = 0;
    if patterns.contains(&design) {
        poss = 1;
    }

    (1..min(max_len, design.len())).for_each(|i| {
        if patterns.contains(&design[0..i]) {
            poss += num_possibilities(&design[i..], patterns, max_len);
        }
    });
    poss
}

fn part1(patterns: HashSet<&str>, designs: Vec<&str>) -> u32 {
    let max_len = patterns.iter().map(|p| p.len()).max().unwrap();
    designs
        .iter()
        .filter(|design| num_possibilities(design, &patterns, max_len) > 0)
        .count() as u32
}

fn part2(patterns: HashSet<&str>, designs: Vec<&str>) -> u64 {
    let max_len = patterns.iter().map(|p| p.len()).max().unwrap();
    designs
        .iter()
        .map(|design| num_possibilities(design, &patterns, max_len))
        .sum::<u64>()
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");
    let (patterns, designs) = parse(input);

    aoc2024_utils::timeit("Part 1", || part1(patterns.clone(), designs.clone()));
    aoc2024_utils::timeit_u64("Part 2", || part2(patterns.clone(), designs.clone()));
    Ok(())
}
