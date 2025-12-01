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

fn clicks_helper(v: i32, pos: i32, c: u32) -> (i32, i32, u32) {
    //println!("{} {} {}", v, pos, c);
    let mut new_pos = pos;
    let mut new_v = v;
    let mut new_c = c;
    if v > 0 {
        new_v = v - 1;
        new_pos += 1;
    } else if v < 0 {
        new_v = v + 1;
        new_pos -= 1;
    } else {
        return (v, pos, c);
    }
    if new_pos % 100 == 0 {
        new_pos = 0;
        new_c += 1;
    }
    clicks_helper(new_v, new_pos, new_c)
}

fn clicks(v: i32, pos: i32, c: u32) -> (i32, u32) {
    let (_, pos, c) = clicks_helper(v, pos, c);
    (pos, c)
}

fn part2(input: &str) -> u32 {
    let data = parse_input(input);
    data.iter()
        .fold((50, 0), |(pos, acc), v| clicks(*v, pos, acc))
        .1
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");

    aoc2025_utils::timeit("Part 1", || part1(input));
    aoc2025_utils::timeit("Part 2", || part2(input));
    Ok(())
}
