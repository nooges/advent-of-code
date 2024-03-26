use std::collections::HashSet;

fn visit(chars: impl Iterator<Item = char>) -> HashSet<(i32, i32)> {
    let mut pos = (0, 0);
    let mut visited = HashSet::new();
    visited.insert(pos);
    for c in chars {
        match c {
            '<' => pos.0 += 1,
            '>' => pos.0 -= 1,
            '^' => pos.1 += 1,
            'v' => pos.1 -= 1,
            _ => (),
        }
        visited.insert(pos);
    }
    visited
}

fn part1(input: &str) -> u32 {
    visit(input.chars()).len() as u32
}

fn part2(input: &str) -> u32 {
    let v1 = visit(input.chars().step_by(2));
    let v2 = visit(input.chars().skip(1).step_by(2));
    v1.union(&v2).count() as u32
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");
    utils::timeit("Part 1", || part1(&input));
    utils::timeit("Part 2", || part2(&input));
    Ok(())
}
