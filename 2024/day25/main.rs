use itertools::Itertools;

fn process_block(block: &str, is_lock: bool) -> Vec<usize> {
    (0..5)
        .map(|c| {
            let rows = if is_lock {
                (1..6).collect_vec()
            } else {
                (1..6).rev().collect_vec()
            };
            for (i, r) in rows.iter().enumerate() {
                if block.chars().nth(6 * r + c).unwrap() == '.' {
                    return i;
                }
            }
            5
        })
        .collect_vec()
}

fn part1(input: &str) -> u32 {
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    input.split("\n\n").for_each(|block| {
        if block.starts_with("#") {
            locks.push(process_block(block, true));
        } else {
            keys.push(process_block(block, false))
        }
    });
    locks
        .iter()
        .map(|lock| {
            keys.iter()
                .filter(|key| key.iter().zip(lock.iter()).all(|(a, b)| a + b < 6))
                .count()
        })
        .sum::<usize>() as u32
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");

    aoc2024_utils::timeit("Part 1", || part1(input));
    Ok(())
}
