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
    let mut neighbors: HashMap<String, HashSet<String>> = HashMap::default();
    for (a, b) in pairs {
        neighbors.entry(a.clone()).or_default().insert(b.clone());
        neighbors.entry(b.clone()).or_default().insert(a.clone());
    }

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

fn bron_kerbosch(
    r: &HashSet<String>,
    p: &mut HashSet<String>,
    x: &mut HashSet<String>,
    neighbors: &HashMap<String, HashSet<String>>,
) -> () {
    if p.is_empty() && x.is_empty() {
        //dbg!(&r);
        println!("{}", r.iter().sorted().join(","));
        //return Some(r.clone());
    }

    let mut p_del = HashSet::default();
    for v in p.iter() {
        let mut sv = HashSet::default();
        sv.insert(v.clone());
        let nv = neighbors[v].clone();
        let mut p_new: HashSet<String> = p.difference(&p_del).cloned().collect();
        p_new = p_new.intersection(&nv).cloned().collect();
        bron_kerbosch(
            &r.union(&sv).cloned().collect(),
            &mut p_new,
            &mut x.intersection(&nv).cloned().collect(),
            neighbors,
        );
        p_del.insert(v.clone());
        x.insert(v.clone());
    }
}

fn part2(input: &str) -> String {
    let pairs = parse(input);
    let mut neighbors: HashMap<String, HashSet<String>> = HashMap::default();
    for (a, b) in pairs {
        neighbors.entry(a.clone()).or_default().insert(b.clone());
        neighbors.entry(b.clone()).or_default().insert(a.clone());
    }

    let mut nodes = HashSet::from_iter(neighbors.clone().keys().map(|n| n.clone()));

    let res = bron_kerbosch(
        &HashSet::default(),
        &mut nodes,
        &mut HashSet::default(),
        &neighbors,
    );

    "".to_string()
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");

    //aoc2024_utils::timeit("Part 1", || part1(input));
    aoc2024_utils::timeit_str("Part 2", || part2(input));
    Ok(())
}
