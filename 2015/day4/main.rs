use std::fs;

fn md5hash(key: &str, num: u32) -> String {
    let res = md5::compute(format!("{}{}", key, num));
    return format!("{:x}", res);
}

fn part1(input: &str) -> u32 {
    return (0..)
        .find(|n| md5hash(input, *n).starts_with("00000"))
        .unwrap();
}

fn part2(input: &str) -> u32 {
    return (0..)
        .find(|n| md5hash(input, *n).starts_with("000000"))
        .unwrap();
}

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let part1_result = utils::timeit("Part 1", || part1(&input));
    println!("Part 1: {}", part1_result);

    let part2_result = utils::timeit("Part 2", || part2(&input));
    println!("Part 2: {}", part2_result);
    Ok(())
}
