use itertools::Itertools;

#[derive(Debug)]
struct Tree {
    rows: u32,
    cols: u32,
    counts: Vec<u32>,
}

fn parse_input(input: &str) -> (Vec<&str>, Vec<Tree>) {
    let groups = input.split("\n\n").collect_vec();
    let shapes = groups[..groups.len() - 1].to_vec();
    let trees = groups[groups.len() - 1]
        .lines()
        .map(|l| {
            let (dims_str, counts_str) = l.split_once(": ").unwrap();
            let dims = aoc2025_utils::extract_u32s(dims_str);
            let counts = aoc2025_utils::extract_u32s(counts_str);
            Tree {
                rows: dims[0],
                cols: dims[1],
                counts,
            }
        })
        .collect();
    (shapes, trees)
}

fn part1(input: &str) -> u64 {
    let (shapes, trees) = parse_input(input);
    println!("{:?}", shapes);
    println!("{:?}", trees);

    trees
        .iter()
        .filter(|tree| tree.counts.iter().sum::<u32>() * 9 <= tree.rows * tree.cols)
        .count() as u64
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
