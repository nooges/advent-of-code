fn part1(input: &str) -> u32 {
    let num_presents_limit: u32 = input.parse().unwrap();
    for house in 0.. {
        let presents: u32 = (1..=house).filter(|elf| house % elf == 0).sum::<u32>() * 10;
        if presents >= num_presents_limit {
            return house;
        }
    }
    0
}

fn part2(input: &str) -> u32 {
    0
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");
    utils::timeit("Part 1", || part1(&input));
    utils::timeit("Part 2", || part2(&input));

    Ok(())
}
