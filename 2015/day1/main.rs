use std::fs;

fn part1(input: &str) -> i32 {
    let open_chars = input.chars().filter(|&c| c == '(').count() as i32;
    return 2 * open_chars - input.len() as i32;
}

fn part2(input: &str) -> i32 {
    let mut floor = 0;
    for (i, c) in input.chars().enumerate() {
        floor += if c == '(' { 1 } else { -1 };
        if floor < 0 {
            return i as i32 + 1;
        }
    }
    return floor;
}

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let part1_result = utils::timeit("Part 1", || part1(&input));
    println!("Part 1: {}", part1_result);

    let part2_result = utils::timeit("Part 2", || part2(&input));
    println!("Part 2: {}", part2_result);
    Ok(())
}
