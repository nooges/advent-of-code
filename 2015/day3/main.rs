use std::{collections::HashSet, fs};

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
    return visited;
}

fn part1(input: &str) -> usize {
    return visit(input.chars()).len();
}

fn part2(input: &str) -> usize {
    let v1 = visit(input.chars().step_by(2));
    let v2 = visit(input.chars().skip(1).step_by(2));
    return v1.union(&v2).count();
}

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let part1_result = utils::timeit("Part 1", || part1(&input));
    println!("Part 1: {}", part1_result);

    let part2_result = utils::timeit("Part 2", || part2(&input));
    println!("Part 2: {}", part2_result);
    Ok(())
}
