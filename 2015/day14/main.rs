use std::{cmp::min, fs};

#[derive(Debug)]
struct Reindeer {
    speed: u32,
    move_time: u32,
    rest_time: u32,
}

fn extract_u32s(line: &str) -> Vec<u32> {
    line.split_whitespace()
        .filter_map(|s| s.parse::<u32>().ok())
        .collect()
}

fn parse_input(input: &str) -> Vec<Reindeer> {
    input
        .lines()
        .map(|line| {
            let parts = extract_u32s(line);
            Reindeer {
                speed: parts[0],
                move_time: parts[1],
                rest_time: parts[2],
            }
        })
        .collect()
}

fn part1(input: &str) -> u32 {
    let race_time = 2503;
    parse_input(input)
        .iter()
        .map(|r| {
            let cycles = race_time / (r.move_time + r.rest_time);
            cycles * r.move_time * r.speed
                + min(r.move_time, race_time % (r.move_time + r.rest_time)) * r.speed
        })
        .max()
        .unwrap()
}

fn part2(input: &str) -> u32 {
    let reindeers = parse_input(input);
    //dbg!(reindeers);
    0
}

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    utils::timeit("Part 1", || part1(&input));
    utils::timeit("Part 2", || part2(&input));

    Ok(())
}
