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
    fn get(&self, pos: &Complex<i32>) -> Option<u8> {
        self.values
            .get(pos.re as usize)?
            .get(pos.im as usize)
            .copied()
    }
}

const DIRS: [Complex<i32>; 4] = [
    Complex::new(0, 1),
    Complex::new(0, -1),
    Complex::new(1, 0),
    Complex::new(-1, 0),
];

fn traverse(
    grid: &Grid,
    pos: Complex<i32>,
    traversed: HashSet<Complex<i32>>,
) -> HashSet<Complex<i32>> {
    let cur_id = grid.get(&pos).unwrap();
    let next_positions = DIRS
        .into_iter()
        .map(|dir| pos + dir)
        .filter(|p| !traversed.contains(p) && grid.get(p).is_some_and(|v| v == cur_id))
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

fn num_neighbors(pos: &Complex<i32>, region: &HashSet<Complex<i32>>) -> u32 {
    DIRS.iter()
        .filter(|dir| region.contains(&(*pos + *dir)))
        .count() as u32
}

fn perimeter(region: &HashSet<Complex<i32>>) -> u32 {
    4 * region.len() as u32
        - region
            .iter()
            .map(|pos| num_neighbors(pos, region))
            .sum::<u32>()
}

fn sides(region: &HashSet<Complex<i32>>, grid: &Grid) -> u32 {
    // Go through lines horizontally
    let mut num_sides = 0;

    let mut prev_starts: HashSet<i32> = HashSet::default();
    let mut prev_ends: HashSet<i32> = HashSet::default();
    for r in 0..grid.nrows {
        let mut inside = false;
        let mut cur_starts: HashSet<i32> = HashSet::default();
        let mut cur_ends: HashSet<i32> = HashSet::default();
        for c in 0..grid.ncols {
            let pos = Complex::new(r, c);
            let has_pos = region.contains(&pos);
            if inside && !has_pos {
                inside = false;
                cur_ends.insert(c);
            } else if !inside && has_pos {
                inside = true;
                cur_starts.insert(c);
            }
        }
        if inside {
            cur_ends.insert(grid.ncols);
        }
        num_sides += cur_starts.difference(&prev_starts).count() as u32;
        num_sides += cur_ends.difference(&prev_ends).count() as u32;
        prev_starts = cur_starts;
        prev_ends = cur_ends;
    }

    // Go through lines vertically
    prev_starts = HashSet::default();
    prev_ends = HashSet::default();
    for c in 0..grid.ncols {
        let mut inside = false;
        let mut cur_starts: HashSet<i32> = HashSet::default();
        let mut cur_ends: HashSet<i32> = HashSet::default();
        for r in 0..grid.nrows {
            let pos = Complex::new(r, c);
            let has_pos = region.contains(&pos);
            if inside && !has_pos {
                inside = false;
                cur_ends.insert(r);
            } else if !inside && has_pos {
                inside = true;
                cur_starts.insert(r);
            }
        }
        if inside {
            cur_ends.insert(grid.nrows);
        }
        num_sides += cur_starts.difference(&prev_starts).count() as u32;
        num_sides += cur_ends.difference(&prev_ends).count() as u32;
        prev_starts = cur_starts;
        prev_ends = cur_ends;
    }

    num_sides
}

fn find_regions(grid: &Grid) -> Vec<HashSet<Complex<i32>>> {
    let mut points_to_check: HashSet<Complex<i32>> = iproduct!(0..grid.nrows, 0..grid.ncols)
        .map(|(r, c)| Complex::new(r, c))
        .collect();

    let mut regions = Vec::new();
    while let Some(&start) = points_to_check.iter().next() {
        let mut visited = HashSet::default();
        visited.insert(start);
        let region = traverse(&grid, start, visited);
        regions.push(region.clone());
        points_to_check.retain(|p| !region.contains(p));
    }
    regions
}

fn part1(input: &str) -> u32 {
    let values = input.lines().map(|l| l.as_bytes().to_vec()).collect_vec();
    let grid = Grid {
        values: values.clone(),
        nrows: values.len() as i32,
        ncols: values[0].len() as i32,
    };

    find_regions(&grid)
        .iter()
        .map(|region| perimeter(region) * region.len() as u32)
        .sum()
}

fn part2(input: &str) -> u32 {
    let values = input.lines().map(|l| l.as_bytes().to_vec()).collect_vec();
    let grid = Grid {
        values: values.clone(),
        nrows: values.len() as i32,
        ncols: values[0].len() as i32,
    };

    find_regions(&grid)
        .iter()
        .map(|region| sides(region, &grid) * region.len() as u32)
        .sum()
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");

    aoc2024_utils::timeit("Part 1", || part1(input));
    aoc2024_utils::timeit("Part 2", || part2(input));
    Ok(())
}
