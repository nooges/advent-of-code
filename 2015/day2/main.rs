use std::fs;

fn part1(dims: &Vec<Vec<u32>>) -> u32 {
    return dims
        .iter()
        .map(|dim| {
            let areas = vec![dim[0] * dim[1], dim[0] * dim[2], dim[1] * dim[2]];
            return 2 * areas.iter().sum::<u32>() + areas.iter().min().unwrap();
        })
        .sum();
}

fn part2(dims: &Vec<Vec<u32>>) -> u32 {
    dims.iter()
        .map(|dim| {
            let mut s = dim.clone();
            s.sort();
            return 2 * (s[0] + s[1]) + dim.iter().product::<u32>();
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
                .collect::<Vec<u32>>();
        })
        .collect();

    utils::timeit("Part 1", || part1(&dims));
    utils::timeit("Part 2", || part2(&dims));
    Ok(())
}
