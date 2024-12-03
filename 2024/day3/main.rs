use regex::Regex;

fn part1(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input)
        .map(|c| c[1].parse::<u32>().unwrap() * c[2].parse::<u32>().unwrap())
        .sum()
}

fn part2(input: &str) -> u32 {
    let re = Regex::new(r"mul\(\d+,\d+\)|don't\(\)|do\(\)").unwrap();
    re.find_iter(input)
        .fold((0, true), |(acc, enabled), m| match m.as_str() {
            "don't()" => (acc, false),
            "do()" => (acc, true),
            s if enabled => (
                acc + aoc2024_utils::extract_u32s(s).iter().product::<u32>(),
                enabled,
            ),
            _ => (acc, enabled),
        })
        .0
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");
    aoc2024_utils::timeit("Part 1", || part1(&input));
    aoc2024_utils::timeit("Part 2", || part2(&input));
    Ok(())
}
