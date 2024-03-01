use std::fs;

fn part1(input: &str) -> i32 {
    let open_chars = input.chars().filter(|&c| c == '(').count() as i32;
    return 2 * open_chars - input.len() as i32;
}

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let part1_result = part1(&input);
    println!("Part 1: {}", part1_result);
    Ok(())
}
