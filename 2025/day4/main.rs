use itertools::Itertools;
use num::complex::Complex;

#[derive(Debug)]
struct GridPoints {
    points: Vec<Complex<i32>>,
    nrows: i32,
    ncols: i32,
}

const DIRS: [Complex<i32>; 8] = [
    Complex::new(-1, -1),
    Complex::new(-1, 0),
    Complex::new(-1, 1),
    Complex::new(0, 1),
    Complex::new(0, -1),
    Complex::new(1, -1),
    Complex::new(1, 0),
    Complex::new(1, 1),
];

fn parse_input(input: &str) -> GridPoints {
    let grid_lines = input.lines().collect_vec();
    let nrows = grid_lines.len() as i32;
    let ncols = grid_lines[0].len() as i32;
    let grid = GridPoints {
        points: grid_lines
            .iter()
            .enumerate()
            .flat_map(|(r, l)| {
                l.chars().enumerate().filter_map(move |(c, v)| match v {
                    '@' => Some(Complex::new(r as i32, c as i32)),
                    _ => None,
                })
            })
            .collect_vec(),
        nrows,
        ncols,
    };

    grid
}

fn around_roll(p: Complex<i32>, grid: &GridPoints) -> usize {
    DIRS.iter()
        .filter_map(|d| grid.points.contains(&(d + p)).then_some(()))
        .count()
}

fn part1(input: &str) -> u32 {
    let grid = parse_input(input);
    grid.points
        .iter()
        .filter_map(|p| (around_roll(*p, &grid) < 4).then_some(()))
        .count() as u32
}

fn part2(input: &str) -> u32 {
    let data = parse_input(input);
    0
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");

    aoc2025_utils::timeit("Part 1", || part1(input));
    //aoc2025_utils::timeit("Part 2", || part2(input));
    Ok(())
}
