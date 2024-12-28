use itertools::Itertools;

fn process_lock(block: &str) -> Vec<usize> {
    (0..5)
        .map(|c| {
            for r in 0..5 {
                if block.chars().nth(6 * (r + 1) + c).unwrap() == '.' {
                    return r;
                }
            }
            5
        })
        .collect_vec()
}

fn process_key(block: &str) -> Vec<usize> {
    (0..5)
        .map(|c| {
            for r in 0..5 {
                if block.chars().nth(6 * (r + 1) + c).unwrap() == '#' {
                    return 5 - r;
                }
            }
            0
        })
        .collect_vec()
}

fn parse(input: &str) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    input.split("\n\n").for_each(|block| {
        if block.starts_with("#") {
            locks.push(process_lock(block));
        } else {
            keys.push(process_key(block));
        }
    });
    (locks, keys)
}

fn part1(input: &str) -> u32 {
    let (locks, keys) = parse(input);
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
