use itertools::{iproduct, Itertools};
use num::complex::Complex;
use std::collections::HashSet;

struct Grid {
    obstacles: HashSet<Complex<i32>>,
    nrows: i32,
    ncols: i32,
}

fn find_symbol(grid: &[&[u8]], symbol: u8) -> HashSet<Complex<i32>> {
    iproduct!(0..grid.len(), 0..grid[0].len())
        .filter(|&(r, c)| grid[r][c] == symbol)
        .map(|(r, c)| Complex::new(r as i32, c as i32))
        .collect()
}

fn parse_input(input: &str) -> (Complex<i32>, Grid) {
    let grid = input.lines().map(|l| l.as_bytes()).collect_vec();
    let nrows = grid.len() as i32;
    let ncols = grid[0].len() as i32;
    let start = *find_symbol(&grid, b'^').iter().next().unwrap();
    let obstacles = find_symbol(&grid, b'#');
    (
        start,
        Grid {
            obstacles,
            nrows,
            ncols,
        },
    )
}

const DIRS: [Complex<i32>; 4] = [
    Complex::new(-1, 0),
    Complex::new(0, 1),
    Complex::new(1, 0),
    Complex::new(0, -1),
];

fn traverse(start: Complex<i32>, grid: &Grid) -> (bool, HashSet<(Complex<i32>, i32)>) {
    let mut traversed = HashSet::from([(start, 0)]);
    let mut pos = start;
    let mut dir = 0;
    loop {
        let next = pos + DIRS[dir as usize];
        if next.re < 0 || next.re >= grid.nrows || next.im < 0 || next.im >= grid.ncols {
            return (false, traversed);
        } else if grid.obstacles.contains(&next) {
            dir = (dir + 1) % 4;
        } else if traversed.contains(&(next, dir)) {
            return (true, traversed);
        } else {
            pos = next;
            traversed.insert((pos, dir));
        }
    }
}

// Remove direction from traversal results and remove starting position
fn traversed_positions(
    traversed: &HashSet<(Complex<i32>, i32)>,
    start: Complex<i32>,
) -> HashSet<Complex<i32>> {
    traversed
        .iter()
        .map(|(pos, _)| *pos)
        .unique()
        .filter(|p| p != &start)
        .collect()
}

fn part1(input: &str) -> u32 {
    let (start, grid) = parse_input(input);
    traversed_positions(&traverse(start, &grid).1, start).len() as u32
}

fn part2(input: &str) -> u32 {
    let (start, grid) = parse_input(input);
    traversed_positions(&traverse(start, &grid).1, start)
        .into_iter()
        .filter(|pos| {
            let mut test_grid = Grid {
                obstacles: grid.obstacles.clone(),
                ..grid
            };
            test_grid.obstacles.insert(*pos);
            traverse(start, &test_grid).0
        })
        .count() as u32

    // For each of those positions, check if they are looped by placing obstacle in grid
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");
    aoc2024_utils::timeit("Part 1", || part1(input));
    aoc2024_utils::timeit("Part 2", || part2(input));
    Ok(())
}
