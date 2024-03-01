use std::{collections::HashSet, fs};

fn visit(input: &str) -> HashSet<(i32, i32)> {
    let mut pos = (0, 0);
    let mut visited = HashSet::new();
    visited.insert(pos);
    for c in input.chars() {
        match c {
            '<' => pos.0 += 1,
            '>' => pos.0 -= 1,
            '^' => pos.1 += 1,
            'v' => pos.1 -= 1,
            _ => (),
        }
        visited.insert(pos);
    }
    return visited;
}

fn part1(input: &str) -> usize {
    return visit(input).len();
}

fn part2(input: &str) -> usize {
    return 0;
}

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let part1_result = part1(&input);
    println!("Part 1: {}", part1_result);
    let part2_result = part2(&input);
    println!("Part 2: {}", part2_result);
    Ok(())
}
