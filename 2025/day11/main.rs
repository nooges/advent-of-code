use cached::proc_macro::cached;
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

#[cached(key = "String", convert = r#"{ format!("{}-{}", node_name, goal) }"#)]
fn dfs(nodes: &HashMap<String, HashSet<String>>, node_name: &str, goal: &str) -> u64 {
    match node_name {
        n if n == goal => 1,
        "out" => 0,
        _ => nodes[node_name].iter().map(|n| dfs(nodes, n, goal)).sum(),
    }
}

fn part1(input: &str) -> u64 {
    let devices = parse_input(input);
    dfs(&devices, "you", "out")
}

fn part2(input: &str) -> u64 {
    let devices = parse_input(input);
    let fft_to_dac = dfs(&devices, "fft", "dac");
    if fft_to_dac != 0 {
        dfs(&devices, "svr", "fft") * fft_to_dac * dfs(&devices, "dac", "out")
    } else {
        dfs(&devices, "svr", "dac") * dfs(&devices, "dac", "fft") * dfs(&devices, "fft", "out")
    }
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");

    aoc2025_utils::timeit_u64("Part 1", || part1(input));
    aoc2025_utils::timeit_u64("Part 2", || part2(input));
    Ok(())
}
