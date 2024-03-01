use std::fs;

fn part1(dims: &Vec<Vec<i32>>) -> i32 {
    return dims
        .iter()
        .map(|dim| {
            let areas = vec![dim[0] * dim[1], dim[0] * dim[2], dim[1] * dim[2]];
            return 2 * areas.iter().sum::<i32>() + areas.iter().min().unwrap();
        })
        .sum();
}

fn part2(dims: &Vec<Vec<i32>>) -> i32 {
    dims.iter()
        .map(|dim| {
            let mut s = dim.clone();
            s.sort();
            return 2 * (s[0] + s[1]) + dim.iter().product::<i32>();
        })
        .sum()
}

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let dims = input
        .lines()
        .map(|line| {
            return line
                .split('x')
                .map(|n| n.parse().unwrap())
                .collect::<Vec<i32>>();
        })
        .collect();
    let part1_result = part1(&dims);
    println!("Part 1: {}", part1_result);
    let part2_result = part2(&dims);
    println!("Part 2: {}", part2_result);
    Ok(())
}
