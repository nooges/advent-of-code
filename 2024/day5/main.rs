use itertools::Itertools;

fn part1(input: &str) -> u32 {
    let (rules, updates) = input.split_once("\n\n").unwrap();
    updates
        .lines()
        .map(|l| l.split(",").collect::<Vec<_>>())
        .filter(|pages| {
            pages.windows(2).all(|w| {
                let search = format!("{}|{}", w[0], w[1]);
                rules.contains(&search)
            })
        })
        .map(|pages| pages[pages.len() / 2].parse::<u32>().unwrap())
        .sum()
}

fn part2(input: &str) -> u32 {
    let (rules, updates) = input.split_once("\n\n").unwrap();
    updates
        .lines()
        .map(|l| l.split(",").collect::<Vec<_>>())
        .filter(|pages| {
            pages
                .windows(2)
                .any(|w| !rules.contains(&format!("{}|{}", w[0], w[1])))
        })
        .map(|pages| {
            let sorted_pages = pages
                .iter()
                .sorted_by(|a, b| {
                    if rules.contains(&format!("{a}|{b}")) {
                        std::cmp::Ordering::Less
                    } else {
                        std::cmp::Ordering::Greater
                    }
                })
                .collect::<Vec<_>>();
            sorted_pages[pages.len() / 2].parse::<u32>().unwrap()
        })
        .sum()
}

fn main() -> std::io::Result<()> {
    let input = include_str!("sample.txt");
    aoc2024_utils::timeit("Part 1", || part1(input));
    aoc2024_utils::timeit("Part 2", || part2(input));
    Ok(())
}
