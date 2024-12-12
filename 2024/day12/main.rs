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

fn part1(input: &str) -> u32 {
    let values = input.lines().map(|l| l.as_bytes().to_vec()).collect_vec();
    let grid = Grid {
        values: values.clone(),
        nrows: values.len() as i32,
        ncols: values[0].len() as i32,
    };

    // Generate all possible start points to check
    let mut points_to_check: HashSet<Complex<i32>> = iproduct!(0..grid.nrows, 0..grid.ncols)
        .map(|(r, c)| Complex::new(r, c))
        .collect();

    let mut regions = Vec::new();
    while let Some(&start) = points_to_check.iter().next() {
        let mut visited = HashSet::default();
        visited.insert(start);
        let region = traverse(&grid, start, visited);
        regions.push(region.clone());
        //points_to_check = points_to_check.difference(&region).copied().collect();
        points_to_check.retain(|p| !region.contains(p));
    }

    // For each region, count number of neighbors and do (4*points - neighbors)
    regions
        .iter()
        .map(|region| {
            let perimeter = 4 * region.len() as u32
                - region
                    .iter()
                    .map(|pos| num_neighbors(pos, region))
                    .sum::<u32>();
            perimeter * region.len() as u32
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    0
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");

    aoc2024_utils::timeit("Part 1", || part1(input));
    aoc2024_utils::timeit("Part 2", || part2(input));
    Ok(())
}
