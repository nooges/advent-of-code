use fxhash::FxHashSet as HashSet;
use itertools::{iproduct, Itertools};
use num::complex::Complex;

#[derive(Debug)]
struct Grid {
    values: Vec<Vec<u8>>,
    nrows: i32,
    ncols: i32,
}

impl Grid {
    fn inbounds(&self, pos: &Complex<i32>) -> bool {
        pos.re >= 0 && pos.re < self.nrows && pos.im >= 0 && pos.im < self.ncols
    }

    fn v(&self, pos: &Complex<i32>) -> i32 {
        self.values[pos.re as usize][pos.im as usize] as i32
    }
}

fn traverse(
    grid: &Grid,
    pos: Complex<i32>,
    traversed: HashSet<Complex<i32>>,
) -> HashSet<Complex<i32>> {
    let dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let cur_value = grid.v(&pos);
    let next_positions = dirs
        .into_iter()
        .map(|(dr, dc)| pos + Complex::new(dr, dc))
        .filter(|p| !traversed.contains(p) && grid.inbounds(p) && grid.v(p) - cur_value == 1)
        .collect_vec();
    if next_positions.is_empty() {
        traversed
    } else {
        next_positions.into_iter().fold(traversed, |mut acc, next| {
            acc.insert(next);
            traverse(grid, next, acc)
        })
    }
}

fn traverse2(grid: &Grid, pos: Complex<i32>, traversed: HashSet<Complex<i32>>) -> u32 {
    let dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let cur_value = grid.v(&pos);
    let mut new_seen = traversed.clone();
    new_seen.insert(pos);

    dirs.into_iter()
        .map(|(dr, dc)| pos + Complex::new(dr, dc))
        .filter(|p| !traversed.contains(p) && grid.inbounds(p) && grid.v(p) - cur_value == 1)
        .map(|new_pos| {
            if grid.v(&new_pos) == 9 {
                return 1;
            }
            traverse2(grid, new_pos, new_seen.clone())
        })
        .sum()
}

fn part1(grid: &Grid) -> u32 {
    let trailheads = iproduct!(0..grid.nrows, 0..grid.ncols)
        .map(|(r, c)| Complex::new(r, c))
        .filter(|pos| grid.v(pos) == 0)
        .collect_vec();

    let paths = trailheads
        .into_iter()
        .map(|start| {
            let mut init = HashSet::default();
            init.insert(start);
            traverse(grid, start, init)
        })
        .collect_vec();

    paths
        .iter()
        .map(|path| path.iter().filter(|pos| grid.v(pos) == 9).count() as u32)
        .sum()
}

fn part2(grid: &Grid) -> u32 {
    let trailheads = iproduct!(0..grid.nrows, 0..grid.ncols)
        .map(|(r, c)| Complex::new(r, c))
        .filter(|pos| grid.v(pos) == 0)
        .collect_vec();

    trailheads
        .into_iter()
        .map(|start| {
            let mut init = HashSet::default();
            init.insert(start);
            traverse2(grid, start, init)
        })
        .sum()
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");
    let values = input
        .lines()
        .map(|l| l.chars().map(|c| c as u8 - b'0').collect_vec())
        .collect_vec();
    let grid = Grid {
        values: values.clone(),
        nrows: values.len() as i32,
        ncols: values[0].len() as i32,
    };

    aoc2024_utils::timeit("Part 1", || part1(&grid));
    aoc2024_utils::timeit("Part 2", || part2(&grid));
    Ok(())
}
