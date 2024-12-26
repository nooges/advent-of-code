use itertools::Itertools;
//use rayon::prelude::*;
use rustc_hash::FxHashMap as HashMap;
use rustc_hash::FxHashSet as HashSet;

fn parse(input: &str) -> Vec<(String, String)> {
    input
        .lines()
        .map(|l| {
            let (a, b) = l.split_once("-").unwrap();
            (a.to_string(), b.to_string())
        })
        .collect()
}

fn part1(input: &str) -> u32 {
    let pairs = parse(input);
    //dbg!(&pairs);

    let mut neighbors: HashMap<String, HashSet<String>> = HashMap::default();
    for (a, b) in pairs {
        neighbors.entry(a.clone()).or_default().insert(b.clone());
        neighbors.entry(b.clone()).or_default().insert(a.clone());
    }
    //dbg!(&neighbors);

    neighbors
        .keys()
        .combinations(3)
        .filter(|c| {
            c.iter().any(|node| node.starts_with("t"))
                && neighbors[c[0]].contains(c[1])
                && neighbors[c[0]].contains(c[2])
                && neighbors[c[1]].contains(c[2])
        })
        .count() as u32
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");

    aoc2024_utils::timeit("Part 1", || part1(input));
    //aoc2024_utils::timeit_u64("Part 2", || part2(input));
    Ok(())
}
