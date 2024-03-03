use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    fs,
};

enum Command {
    TurnOn,
    TurnOff,
    Toggle,
}

struct Instruction {
    command: Command,
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

fn parse_input(input: &str) -> Vec<Instruction> {
    let t: Vec<Instruction> = input
        .lines()
        .map(|line| {
            let line = line.replace("turn ", "");
            let s = line.split_whitespace().collect::<Vec<&str>>();
            let p1 = s[1].split(',').collect::<Vec<_>>();
            let p2 = s[3].split(',').collect::<Vec<_>>();
            Instruction {
                command: match s[0] {
                    "on" => Command::TurnOn,
                    "off" => Command::TurnOff,
                    "toggle" => Command::Toggle,
                    _ => panic!("Invalid command"),
                },
                x1: p1[0].parse().unwrap(),
                y1: p1[1].parse().unwrap(),
                x2: p2[0].parse().unwrap(),
                y2: p2[1].parse().unwrap(),
            }
        })
        .collect();

    return t;
}

fn part1(data: &Vec<Instruction>) -> u32 {
    let mut lights_on = HashSet::new();
    data.iter().for_each(|i| {
        let xr = min(i.x1, i.x2)..=max(i.x1, i.x2);
        let yr = min(i.y1, i.y2)..=max(i.y1, i.y2);
        for p in itertools::iproduct!(xr, yr) {
            match i.command {
                Command::TurnOn => {
                    lights_on.insert(p);
                }
                Command::TurnOff => {
                    lights_on.remove(&p);
                }
                Command::Toggle => {
                    if lights_on.contains(&p) {
                        lights_on.remove(&p);
                    } else {
                        lights_on.insert(p);
                    }
                }
            }
        }
    });
    return lights_on.len() as u32;
}

fn part2(data: &Vec<Instruction>) -> u32 {
    let mut levels = HashMap::new();
    data.iter().for_each(|i| {
        let xr = min(i.x1, i.x2)..=max(i.x1, i.x2);
        let yr = min(i.y1, i.y2)..=max(i.y1, i.y2);
        for p in itertools::iproduct!(xr, yr) {
            let d = match i.command {
                Command::TurnOn => 1,
                Command::TurnOff => -1,
                Command::Toggle => 2,
            };
            let cur = levels.entry(p).or_insert(0);
            *cur = (*cur + d).max(0);
        }
    });
    return levels.values().sum::<i32>() as u32;
}

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let data = parse_input(&input);

    let part1_result = utils::timeit("Part 1", || part1(&data));
    println!("Part 1: {}", part1_result);

    let part2_result = utils::timeit("Part 2", || part2(&data));
    println!("Part 2: {}", part2_result);

    Ok(())
}
