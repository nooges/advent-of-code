use itertools::Itertools;
//use rayon::prelude::*;
use regex::Regex;
use rustc_hash::FxHashMap as HashMap;

#[derive(Debug)]
struct Gate {
    in1: String,
    in2: String,
    op: String,
    out: String,
    solved: bool,
}

impl Gate {
    fn output(&self, wires: HashMap<String, u64>) -> Option<u64> {
        if let (Some(v1), Some(v2)) = (wires.get(&self.in1), wires.get(&self.in2)) {
            match self.op.as_str() {
                "AND" => Some(v1 & v2),
                "OR" => Some(v1 | v2),
                "XOR" => Some(v1 ^ v2),
                _ => panic!("Invalid op"),
            }
        } else {
            None
        }
    }
}

fn parse(input: &str) -> (HashMap<String, u64>, Vec<Gate>) {
    let (init_str, gates_str) = input.split_once("\n\n").unwrap();
    let mut init_wires: HashMap<String, u64> = HashMap::default();

    for l in init_str.lines() {
        let wire = l.split_once(": ").unwrap();
        init_wires.insert(wire.0.to_string(), wire.1.parse().unwrap());
    }

    let re = Regex::new(r"(.*?) (.*?) (.*?) -> (.*)").unwrap();

    let gates = gates_str
        .lines()
        .map(|l| {
            let c = re.captures(l).unwrap();
            Gate {
                in1: c[1].to_string(),
                in2: c[3].to_string(),
                op: c[2].to_string(),
                out: c[4].to_string(),
                solved: false,
            }
        })
        .collect_vec();

    (init_wires, gates)
}

fn part1(input: &str) -> u64 {
    let (mut wires, mut gates) = parse(input);
    loop {
        for gate in &mut gates {
            if !gate.solved {
                if let Some(output) = gate.output(wires.clone()) {
                    &wires.insert(gate.out.clone(), output);
                    gate.solved = true;
                }
            }
        }

        let num_left = gates.iter().filter(|g| !g.solved).count();
        //dbg!(num_left);
        if num_left == 0 {
            break;
        }
    }
    // Form z value
    wires
        .iter()
        .filter(|(wire, _)| wire.starts_with("z"))
        .map(|(wire, val)| *val << wire.split("z").last().unwrap().parse::<u32>().unwrap())
        .sum()
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");

    aoc2024_utils::timeit_u64("Part 1", || part1(input));
    //aoc2024_utils::timeit_u64("Part 2", || part2(input));
    Ok(())
}
