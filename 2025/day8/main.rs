use itertools::Itertools;
use rustc_hash::FxHashSet as HashSet;

fn parse_input(input: &str) -> Vec<(usize, Vec<u64>)> {
    input
        .lines()
        .map(aoc2025_utils::extract_u64s)
        .enumerate()
        .collect_vec()
}

fn solve(input: &str, num_connections: usize) -> (u32, u64) {
    let boxes = parse_input(input);
    let distances = boxes
        .iter()
        .combinations(2)
        .map(|combo| {
            let (a_idx, a) = combo[0];
            let (b_idx, b) = combo[1];
            let dist = (a[0] - b[0]) * (a[0] - b[0])
                + (a[1] - b[1]) * (a[1] - b[1])
                + (a[2] - b[2]) * (a[2] - b[2]);
            (dist, a_idx, b_idx)
        })
        .sorted_by_key(|(d, _, _)| *d)
        .collect_vec();

    let mut circuits: Vec<HashSet<usize>> = Vec::new();
    let mut i = 0;
    let mut res1 = 0;
    let mut res2 = 0;

    for (_, a, b) in &distances {
        let a_idx = circuits.iter().position(|c| c.contains(a));
        let b_idx = circuits.iter().position(|c| c.contains(b));
        i += 1;
        match (a_idx, b_idx) {
            (Some(a_idx), Some(b_idx)) => {
                if a_idx == b_idx {
                    continue;
                }

                let circuit1 = circuits.remove(a_idx.max(b_idx));
                let circuit2 = circuits.remove(a_idx.min(b_idx));
                circuits.push(circuit1.union(&circuit2).cloned().collect());
            }
            (Some(a_idx), None) => {
                circuits[a_idx].insert(**b);
            }
            (None, Some(b_idx)) => {
                circuits[b_idx].insert(**a);
            }
            _ => {
                circuits.push(HashSet::from_iter(vec![**a, **b]));
            }
        }
        if i == num_connections {
            res1 = circuits
                .iter()
                .map(|c| c.len() as u32)
                .sorted()
                .rev()
                .take(3)
                .product();
        }
        if circuits.len() == 1 && circuits[0].len() == boxes.len() {
            res2 = boxes[**a].1[0] * boxes[**b].1[0];
            break;
        }
    }
    (res1, res2)
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");

    println!("{:?}", solve(input, 1000));
    Ok(())
}
