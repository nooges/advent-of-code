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

fn get_value(wires: &HashMap<String, u64>, prefix: &str) -> u64 {
    wires
        .iter()
        .filter(|(wire, _)| wire.starts_with(prefix))
        .map(|(wire, val)| *val << wire.split(prefix).last().unwrap().parse::<u32>().unwrap())
        .sum()
}

fn test_gates(wires: HashMap<String, u64>, gates: Vec<Gate>) -> (u64, u64, u64) {
    let mut wires = wires;
    let mut gates = gates;

    loop {
        for gate in &mut gates {
            if !gate.solved {
                if let Some(output) = gate.output(wires.clone()) {
                    let _ = &wires.insert(gate.out.clone(), output);
                    gate.solved = true;
                }
            }
        }

        let num_left = gates.iter().filter(|g| !g.solved).count();
        if num_left == 0 {
            break;
        }
    }
    (
        get_value(&wires, "x"),
        get_value(&wires, "y"),
        get_value(&wires, "z"),
    )
}

fn part1(input: &str) -> u64 {
    let (wires, gates) = parse(input);
    let (_, _, z) = test_gates(wires, gates);
    z
}

fn part2(input: &str) -> u64 {
    0
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");

    aoc2024_utils::timeit_u64("Part 1", || part1(input));
    //aoc2024_utils::timeit_u64("Part 2", || part2(input));
    Ok(())
}
