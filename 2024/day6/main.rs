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
    dbg!(start, &obstacles);
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

fn part1(input: &str) -> u32 {
    let (start, grid) = parse_input(input);
    let mut traversed = HashSet::from([start]);

    fn traverse(
        pos: Complex<i32>,
        dir: u32,
        traversed: &mut HashSet<Complex<i32>>,
        grid: &Grid,
    ) -> u32 {
        let next = pos + DIRS[dir as usize];
        if next.re < 0 || next.re >= grid.nrows || next.im < 0 || next.im >= grid.ncols {
            traversed.len() as u32
        } else if grid.obstacles.contains(&next) {
            traverse(pos, (dir + 1) % 4, traversed, grid)
        } else {
            traversed.insert(next);
            traverse(next, dir, traversed, grid)
        }
    }
    traverse(start, 0, &mut traversed, &grid)
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");
    aoc2024_utils::timeit("Part 1", || part1(input));
    //aoc2024_utils::timeit("Part 2", || part2(input));
    Ok(())
}
