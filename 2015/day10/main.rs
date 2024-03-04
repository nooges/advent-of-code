use std::fs;

fn say_number(s: String) -> String {
    let mut res: String = String::new();
    let mut count = 0;
    let mut curr = '\0';
    s.chars().for_each(|c| {
        if c == curr {
            count += 1;
        } else {
            if count != 0 {
                res.push_str(&format!("{}{}", count, curr));
            }
            count = 1;
            curr = c;
        }
    });
    res.push_str(&format!("{}{}", count, curr));
    res
}

fn part1(input: &str) -> u32 {
    let mut input = input.to_string();
    for _ in 0..40 {
        input = say_number(input);
    }
    input.len() as u32
}

fn part2(input: &str) -> u32 {
    let mut input = input.to_string();
    for _ in 0..50 {
        input = say_number(input);
    }
    input.len() as u32
}

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    utils::timeit("Part 1", || part1(&input));
    utils::timeit("Part 2", || part2(&input));

    Ok(())
}
