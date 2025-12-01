fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|l| {
            let p = l.split_at(1);
            let v: i32 = p.1.parse().unwrap();
            if p.0 == "L" {
                -v
            } else {
                v
            }
        })
        .collect()
}

fn part1(input: &str) -> u32 {
    let data = parse_input(input);
    data.iter()
        .fold((50, 0), |(pos, acc), v| {
            let new_pos = (pos + v + 100) % 100;
            if new_pos == 0 {
                (new_pos, acc + 1)
            } else {
                (new_pos, acc)
            }
        })
        .1
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");

    aoc2025_utils::timeit("Part 1", || part1(input));
    Ok(())
}
