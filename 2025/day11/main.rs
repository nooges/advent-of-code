use itertools::Itertools;
use rustc_hash::FxHashMap as HashMap;
use rustc_hash::FxHashSet as HashSet;

fn parse_input(input: &str) -> HashMap<String, HashSet<String>> {
    HashMap::from_iter(
        input
            .lines()
            .map(|l| {
                let (name, outputs) = l.split_once(": ").unwrap();

                (
                    name.to_string(),
                    outputs.split(" ").map(|s| s.to_string()).collect(),
                )
            })
            .collect_vec(),
    )
}

fn traverse(nodes: &HashMap<String, HashSet<String>>, node_name: &str) -> u64 {
    if node_name == "out" {
        return 1;
    }
    nodes[node_name].iter().map(|n| traverse(nodes, n)).sum()
}

fn part1(input: &str) -> u64 {
    let devices = parse_input(input);
    traverse(&devices, "you")
}

fn part2(input: &str) -> u64 {
    let devices = parse_input(input);
    0
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");

    aoc2025_utils::timeit_u64("Part 1", || part1(input));
    //aoc2025_utils::timeit_u64("Part 2", || part2(input));
    Ok(())
}
