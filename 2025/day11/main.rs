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

fn dfs(
    nodes: &HashMap<String, HashSet<String>>,
    node_name: &str,
    goal: &str,
    mem: &mut HashMap<String, u64>,
) -> u64 {
    if let Some(v) = mem.get(node_name) {
        *v
    } else if node_name == goal {
        1
    } else if node_name == "out" {
        0
    } else {
        let sum = nodes[node_name]
            .iter()
            .map(|n| dfs(nodes, n, goal, mem))
            .sum();
        mem.insert(node_name.to_string(), sum);
        sum
    }
}

fn part1(input: &str) -> u64 {
    let devices = parse_input(input);
    dfs(&devices, "you", "out", &mut HashMap::default())
}

fn part2(input: &str) -> u64 {
    let devices = parse_input(input);
    // No paths from dac to fft, so just do svr -> fft -> dac -> out
    dfs(&devices, "svr", "fft", &mut HashMap::default())
        * dfs(&devices, "fft", "dac", &mut HashMap::default())
        * dfs(&devices, "dac", "out", &mut HashMap::default())
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");

    aoc2025_utils::timeit_u64("Part 1", || part1(input));
    aoc2025_utils::timeit_u64("Part 2", || part2(input));
    Ok(())
}
