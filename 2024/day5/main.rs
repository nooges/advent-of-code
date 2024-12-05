use itertools::Itertools;
use std::cmp::Ordering;

fn cmp_pages(a: &str, b: &str, rules: &str) -> bool {
    let search = format!("{}|{}", a, b);
    rules.contains(&search)
}

fn middle_page(pages: Vec<&str>) -> u32 {
    pages[pages.len() / 2].parse::<u32>().unwrap()
}

fn part1(input: &str) -> u32 {
    let (rules, updates) = input.split_once("\n\n").unwrap();
    updates
        .lines()
        .map(|l| l.split(",").collect::<Vec<_>>())
        .filter(|pages| pages.is_sorted_by(|a, b| cmp_pages(a, b, rules)))
        .map(middle_page)
        .sum()
}

fn part2(input: &str) -> u32 {
    let (rules, updates) = input.split_once("\n\n").unwrap();
    updates
        .lines()
        .map(|l| l.split(",").collect::<Vec<_>>())
        .filter(|pages| !pages.is_sorted_by(|a, b| cmp_pages(a, b, rules)))
        .map(|pages| {
            let sorted_pages: Vec<&str> = pages
                .into_iter()
                .sorted_by(|a, b| {
                    if cmp_pages(a, b, rules) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                })
                .collect::<Vec<_>>();
            middle_page(sorted_pages)
        })
        .sum()
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");
    aoc2024_utils::timeit("Part 1", || part1(input));
    aoc2024_utils::timeit("Part 2", || part2(input));
    Ok(())
}
