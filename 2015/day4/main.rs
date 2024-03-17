use std::fs;

fn md5hash(key: &str, num: u32) -> String {
    let res = md5::compute(format!("{}{}", key, num));
    format!("{:x}", res)
}

fn part1(input: &str) -> u32 {
    (0..)
        .find(|n| md5hash(input, *n).starts_with("00000"))
        .unwrap()
}

fn part2(input: &str) -> u32 {
    (0..)
        .find(|n| md5hash(input, *n).starts_with("000000"))
        .unwrap()
}

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    utils::timeit("Part 1", || part1(&input));
    utils::timeit("Part 2", || part2(&input));
    Ok(())
}
