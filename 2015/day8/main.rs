use std::fs;

fn decoded_str_len(s: &str) -> usize {
    let mut len: i32 = 0;
    let mut escaped = false;
    for i in s.chars() {
        match (i, escaped) {
            ('\\', false) => escaped = true,
            ('x', true) => {
                len -= 1;
                escaped = false;
            }
            _ => {
                len += 1;
                escaped = false;
            }
        }
    }
    (len - 2) as usize
}

fn encoded_str_len(s: &str) -> usize {
    let mut len = 0;
    for i in s.chars() {
        len += match i {
            '"' | '\\' => 2,
            _ => 1,
        };
    }
    len + 2
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.len() - decoded_str_len(line))
        .sum::<usize>() as u32
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| encoded_str_len(line) - line.len())
        .sum::<usize>() as u32
}

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    utils::timeit("Part 1", || part1(&input));
    utils::timeit("Part 2", || part2(&input));

    Ok(())
}
