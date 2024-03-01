use std::fs;

fn part1(input: &str) -> i32 {
    return input
        .lines()
        .map(|line| {
            let dims = line
                .split('x')
                .map(|n| n.parse().unwrap())
                .collect::<Vec<i32>>();
            let areas = vec![dims[0] * dims[1], dims[0] * dims[2], dims[1] * dims[2]];
            return 2 * areas.iter().sum::<i32>() + areas.iter().min().unwrap();
        })
        .sum();
}

fn part2(input: &str) -> i32 {
    return 0;
}

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let part1_result = part1(&input);
    println!("Part 1: {}", part1_result);
    let part2_result = part2(&input);
    println!("Part 2: {}", part2_result);
    Ok(())
}
