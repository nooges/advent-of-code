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

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");
    aoc2024_utils::timeit("Part 1", || part1(input));
    //aoc2024_utils::timeit("Part 2", || part2(input));
    Ok(())
}
