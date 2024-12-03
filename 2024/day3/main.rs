use regex::Regex;

fn part1(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input)
        .map(|c| c[1].parse::<u32>().unwrap() * c[2].parse::<u32>().unwrap())
        .sum()
}

fn part2(input: &str) -> u32 {
    let re = Regex::new(r"mul\(\d+,\d+\)|don't\(\)|do\(\)").unwrap();
    let matches = re.find_iter(input).map(|m| m.as_str()).collect::<Vec<_>>();
    let mut flag = true;
    let mut sum = 0;
    for m in matches {
        match m {
            "don't" => flag = false,
            "do" => flag = true,
            _ if flag => sum += aoc2024_utils::extract_u32s(m).iter().product::<u32>(),
            _ => (),
        }
    }
    sum
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");
    aoc2024_utils::timeit("Part 1", || part1(&input));
    aoc2024_utils::timeit("Part 2", || part2(&input));
    Ok(())
}
