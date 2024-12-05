use itertools::Itertools;

fn page_lt(a: &str, b: &str, rules: &str) -> bool {
    let search = format!("{}|{}", a, b);
    rules.contains(&search)
}

fn middle_page(pages: Vec<&str>) -> u32 {
    pages[pages.len() / 2].parse::<u32>().unwrap()
}

fn part1(input: &str) -> u32 {
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let cmp_pages = |a: &&str, b: &&str| page_lt(a, b, rules);
    updates
        .lines()
        .map(|l| l.split(",").collect::<Vec<_>>())
        .filter(|pages| pages.is_sorted_by(cmp_pages))
        .map(middle_page)
        .sum()
}

fn part2(input: &str) -> u32 {
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let cmp_pages = |a: &&str, b: &&str| page_lt(a, b, rules);
    updates
        .lines()
        .map(|l| l.split(",").collect::<Vec<_>>())
        .filter(|pages| !pages.is_sorted_by(cmp_pages))
        .map(|pages| {
            pages
                .into_iter()
                .sorted_by(|a, b| true.cmp(&cmp_pages(a, b)))
                .collect::<Vec<_>>()
        })
        .map(middle_page)
        .sum()
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");
    aoc2024_utils::timeit("Part 1", || part1(input));
    aoc2024_utils::timeit("Part 2", || part2(input));
    Ok(())
}
