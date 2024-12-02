fn part1(input: &str) -> u32 {
    0
}

fn part2(input: &str) -> u32 {
    0
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");
    aoc2024_utils::timeit("Part 1", || part1(&input));
    aoc2024_utils::timeit("Part 2", || part2(&input));

    Ok(())
}
