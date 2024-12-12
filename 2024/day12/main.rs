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
    region
        .iter()
        .map(|pos| 4 - num_neighbors(pos, region))
        .sum::<u32>()
}

fn find_edges(positions: HashSet<i32>, max: i32) -> (HashSet<i32>, HashSet<i32>) {
    let mut starts: HashSet<i32> = HashSet::default();
    let mut ends: HashSet<i32> = HashSet::default();

    let mut inside = false;
    for i in 0..max {
        let has_pos = positions.contains(&i);
        if has_pos && !inside {
            starts.insert(i);
            inside = true;
        } else if !has_pos && inside {
            ends.insert(i);
            inside = false;
        }
    }
    if inside {
        ends.insert(max);
    }

    (starts, ends)
}

fn sides(region: &HashSet<Complex<i32>>, grid: &Grid) -> u32 {
    let mut num_sides = 0;

    let mut prev_edges: (HashSet<i32>, HashSet<i32>) = (HashSet::default(), HashSet::default());
    // Find vertical sides
    for r in 0..grid.nrows {
        let line = region.iter().filter(|p| p.re == r).map(|p| p.im).collect();
        let edges = find_edges(line, grid.ncols);

        num_sides += edges.0.difference(&prev_edges.0).count();
        num_sides += edges.1.difference(&prev_edges.1).count();
        prev_edges = edges;
    }

    // Find horizontal sides
    prev_edges = (HashSet::default(), HashSet::default());
    for c in 0..grid.ncols {
        let line = region.iter().filter(|p| p.im == c).map(|p| p.re).collect();
        let edges = find_edges(line, grid.nrows);

        num_sides += edges.0.difference(&prev_edges.0).count();
        num_sides += edges.1.difference(&prev_edges.1).count();
        prev_edges = edges;
    }

    num_sides as u32
}

fn find_regions(grid: &Grid) -> Vec<HashSet<Complex<i32>>> {
    let mut points_to_check: HashSet<Complex<i32>> = iproduct!(0..grid.nrows, 0..grid.ncols)
        .map(|(r, c)| Complex::new(r, c))
        .collect();

    let mut regions = Vec::new();
    while let Some(&start) = points_to_check.iter().next() {
        let mut visited = HashSet::default();
        visited.insert(start);
        let region = traverse(grid, start, visited);
        regions.push(region.clone());
        points_to_check.retain(|p| !region.contains(p));
    }
    regions
}

fn part1(grid: &Grid) -> u32 {
    find_regions(grid)
        .iter()
        .map(|region| perimeter(region) * region.len() as u32)
        .sum()
}

fn part2(grid: &Grid) -> u32 {
    find_regions(grid)
        .iter()
        .map(|region| sides(region, grid) * region.len() as u32)
        .sum()
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");
    let values = input.lines().map(|l| l.as_bytes().to_vec()).collect_vec();
    let grid = Grid {
        values: values.clone(),
        nrows: values.len() as i32,
        ncols: values[0].len() as i32,
    };

    aoc2024_utils::timeit("Part 1", || part1(&grid));
    aoc2024_utils::timeit("Part 2", || part2(&grid));
    Ok(())
}
