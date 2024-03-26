use std::cmp::min;

#[derive(Debug)]
struct Reindeer {
    speed: u32,
    move_time: u32,
    rest_time: u32,
}

fn parse_input(input: &str) -> Vec<Reindeer> {
    input
        .lines()
        .map(|line| {
            let parts = utils::extract_u32s(line);
            Reindeer {
                speed: parts[0],
                move_time: parts[1],
                rest_time: parts[2],
            }
        })
        .collect()
}

fn position(r: &Reindeer, t: u32) -> u32 {
    let cycles = t / (r.move_time + r.rest_time);
    cycles * r.move_time * r.speed + min(r.move_time, t % (r.move_time + r.rest_time)) * r.speed
}

fn part1(input: &str) -> u32 {
    let race_time = 2503;
    parse_input(input)
        .iter()
        .map(|r| position(r, race_time))
        .max()
        .unwrap()
}

fn part2(input: &str) -> u32 {
    let race_time = 2503;
    let reindeer = parse_input(input);
    let distances = reindeer
        .iter()
        .map(|r| (1..=race_time).map(|t| position(r, t)).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let max_distances = (0..distances[0].len())
        .map(|i| distances.iter().map(|d| d[i]).max().unwrap())
        .collect::<Vec<_>>();
    distances
        .iter()
        .map(|d| {
            d.iter()
                .zip(max_distances.iter())
                .filter(|(a, b)| a == b)
                .count()
        })
        .max()
        .unwrap() as u32
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");
    utils::timeit("Part 1", || part1(&input));
    utils::timeit("Part 2", || part2(&input));

    Ok(())
}
