use std::fs;

fn part1(input: &str) -> u32 {
    let open_chars = input.chars().filter(|&c| c == '(').count() as i32;
    (2 * open_chars - input.len() as i32) as u32
}

fn part2(input: &str) -> u32 {
    let mut floor: i32 = 0;
    for (i, c) in input.chars().enumerate() {
        floor += if c == '(' { 1 } else { -1 };
        if floor < 0 {
            return i as u32 + 1;
        }
    }
    floor as u32
}

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    utils::timeit("Part 1", || part1(&input));
    utils::timeit("Part 2", || part2(&input));
    Ok(())
}
