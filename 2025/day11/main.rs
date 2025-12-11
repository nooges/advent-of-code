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

fn dfs(nodes: &HashMap<String, HashSet<String>>, node_name: &str, goal: &str) -> u64 {
    dfs_helper(nodes, node_name, goal, &mut HashMap::default())
}

fn dfs_helper(
    nodes: &HashMap<String, HashSet<String>>,
    node_name: &str,
    goal: &str,
    mem: &mut HashMap<String, u64>,
) -> u64 {
    match mem.get(node_name) {
        Some(v) => *v,
        None => match node_name {
            n if n == goal => 1,
            "out" => 0,
            _ => {
                let sum = nodes[node_name]
                    .iter()
                    .map(|n| dfs_helper(nodes, n, goal, mem))
                    .sum();
                mem.insert(node_name.to_string(), sum);
                sum
            }
        },
    }
}

fn part1(input: &str) -> u64 {
    let devices = parse_input(input);
    dfs(&devices, "you", "out")
}

fn part2(input: &str) -> u64 {
    let devices = parse_input(input);
    let dac_to_fft = dfs(&devices, "dac", "fft");
    if dac_to_fft == 0 {
        dfs(&devices, "svr", "fft") * dfs(&devices, "fft", "dac") * dfs(&devices, "dac", "out")
    } else {
        dfs(&devices, "svr", "dac") * dac_to_fft * dfs(&devices, "fft", "out")
    }
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");

    aoc2025_utils::timeit_u64("Part 1", || part1(input));
    aoc2025_utils::timeit_u64("Part 2", || part2(input));
    Ok(())
}
