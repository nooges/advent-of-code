use fxhash::FxHashSet;
use itertools::{iproduct, Itertools};
use num::complex::Complex;
use rayon::prelude::*;

#[derive(Clone)]
struct Grid {
    obstacles: FxHashSet<Complex<i32>>,
    nrows: i32,
    ncols: i32,
}

fn find_symbol(grid: &[&[u8]], symbol: u8) -> FxHashSet<Complex<i32>> {
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

fn traverse(start: Complex<i32>, grid: &Grid) -> (bool, FxHashSet<(Complex<i32>, Complex<i32>)>) {
    let mut p = start;
    let mut dir = Complex::new(-1, 0);
    let mut traversed = FxHashSet::default();
    traversed.insert((start, dir));
    loop {
        let next = p + dir;
        if next.re < 0 || next.re >= grid.nrows || next.im < 0 || next.im >= grid.ncols {
            return (false, traversed); // Out of bounds
        } else if grid.obstacles.contains(&next) {
            dir *= Complex::new(0, -1); // Rotate right
        } else if traversed.contains(&(next, dir)) {
            return (true, traversed); // Loop detected
        } else {
            p = next;
            traversed.insert((p, dir));
        }
    }
}

// Remove direction from traversal results and remove starting position
fn traversed_positions(start: Complex<i32>, grid: &Grid) -> FxHashSet<Complex<i32>> {
    traverse(start, grid)
        .1
        .iter()
        .map(|(p, _)| *p)
        .unique()
        .filter(|p| p != &start)
        .collect()
}

fn part1(input: &str) -> u32 {
    let (start, grid) = parse_input(input);
    traversed_positions(start, &grid).len() as u32
}

fn part2(input: &str) -> u32 {
    let (start, grid) = parse_input(input);
    traversed_positions(start, &grid)
        .into_par_iter()
        .filter(|p| {
            let mut test_grid = grid.clone();
            test_grid.obstacles.insert(*p);
            traverse(start, &test_grid).0
        })
        .count() as u32
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");
    aoc2024_utils::timeit("Part 1", || part1(input));
    aoc2024_utils::timeit("Part 2", || part2(input));
    Ok(())
}
