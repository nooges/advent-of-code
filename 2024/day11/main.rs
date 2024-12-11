use cached::proc_macro::cached;
use fxhash::FxHashMap as HashMap;
use num::pow;

fn num_digits(n: u64) -> u32 {
    n.ilog10() + 1
}

fn solve(input: &str, blinks: u32) -> u64 {
    let mut stones: HashMap<_, _> = aoc2024_utils::extract_u64s(input)
        .into_iter()
        .map(|n| (n, 1))
        .collect();

    (0..blinks).for_each(|_| {
        let mut new_stones: HashMap<_, _> = HashMap::default();
        for (n, count) in &stones {
            if *n == 0 {
                *new_stones.entry(1).or_default() += count;
            } else if num_digits(*n) % 2 == 0 {
                let m = 10u64.pow(num_digits(*n) / 2);
                *new_stones.entry(*n / m).or_default() += count;
                *new_stones.entry(*n % m).or_default() += count;
            } else {
                *new_stones.entry(*n * 2024).or_default() += count;
            }
        }

        stones = new_stones;
    });
    stones.into_values().sum::<u64>()
}

fn solve_cached(input: &str, blinks: u32) -> u64 {
    #[cached]
    fn helper(n: u64, blinks: u32) -> u64 {
        if blinks == 0 {
            1
        } else if n == 0 {
            helper(1, blinks - 1)
        } else if num_digits(n) % 2 == 0 {
            let m = 10u64.pow(num_digits(n) / 2);
            helper(n / m, blinks - 1) + helper(n % m, blinks - 1)
        } else {
            helper(n * 2024, blinks - 1)
        }
    }
    aoc2024_utils::extract_u64s(input)
        .iter()
        .map(|n| helper(*n, blinks))
        .sum()
}

fn part1(input: &str) -> u64 {
    solve(input, 25)
}
fn part2(input: &str) -> u64 {
    solve(input, 75)
}

fn part1_cached(input: &str) -> u64 {
    solve_cached(input, 25)
}
fn part2_cached(input: &str) -> u64 {
    solve_cached(input, 75)
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");

    aoc2024_utils::timeit_u64("Part 1", || part1(input));
    aoc2024_utils::timeit_u64("Part 2", || part2(input));
    println!("(Cached Version)");
    aoc2024_utils::timeit_u64("Part 1", || part1_cached(input));
    aoc2024_utils::timeit_u64("Part 2", || part2_cached(input));
    Ok(())
}
