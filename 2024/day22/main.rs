use itertools::Itertools;
use rayon::prelude::*;
use rustc_hash::FxHashMap as HashMap;
use std::collections::VecDeque;

fn next_secret(n: u64) -> u64 {
    let mut next = n;
    next = ((next << 6) ^ n) & 0xFFFFFF;
    next = ((next >> 5) ^ next) & 0xFFFFFF;
    ((next << 11) ^ next) & 0xFFFFFF
}

fn index(seq: &VecDeque<i64>) -> usize {
    let mut index = 0;
    for i in 0..seq.len() {
        index += (seq[i] + 9) * 19i64.pow(i as u32);
    }
    index as usize
}

fn sequence_prices(n: u64) -> HashMap<usize, u64> {
    // Generate price change sequence and associated price for a 4-number sequence
    let mut seq_prices = HashMap::default();
    let mut i = 0;
    let mut deque: VecDeque<_> = VecDeque::new();
    let mut prev = n;
    while i < 2000 {
        let next = next_secret(prev);
        deque.push_back((next % 10) as i64 - (prev % 10) as i64);
        if deque.len() == 4 {
            let idx = index(&deque);
            if !seq_prices.contains_key(&idx) {
                seq_prices.insert(idx, next % 10);
            }
            deque.pop_front();
        }
        prev = next;
        i += 1;
    }

    seq_prices
}

fn part1(input: &str) -> u64 {
    let secret_numbers = input
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .collect_vec();
    secret_numbers
        .iter()
        .map(|n| {
            let mut next = *n;
            for _ in 0..2000 {
                next = next_secret(next);
            }
            next
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    let secret_numbers = input
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .collect_vec();
    let seq_prices = secret_numbers
        .iter()
        .map(|n| sequence_prices(*n))
        .collect_vec();
    seq_prices
        .iter()
        .map(|m| m.keys())
        .flatten()
        .unique()
        .map(|s| seq_prices.iter().filter_map(|m| m.get(s)).sum::<u64>())
        .max()
        .unwrap()
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");

    aoc2024_utils::timeit_u64("Part 1", || part1(input));
    aoc2024_utils::timeit_u64("Part 2", || part2(input));
    Ok(())
}
