fn part1(input: &str) -> u64 {
    input
        .split("\n\n")
        .last()
        .unwrap()
        .lines()
        .filter(|l| {
            let v = aoc2025_utils::extract_u32s(l);
            v[0] / 3 * v[1] / 3 >= v[2..].iter().sum::<u32>()
        })
        .count() as u64
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");

    aoc2025_utils::timeit_u64("Part 1", || part1(input));
    Ok(())
}
