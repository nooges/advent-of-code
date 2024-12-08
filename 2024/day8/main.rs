use fxhash::FxHashMap as HashMap;
use fxhash::FxHashSet as HashSet;
use itertools::{iproduct, Itertools};
use num::complex::Complex;

fn antinode_pair(p: &[Complex<i32>], nrows: i32, ncols: i32) -> Vec<Complex<i32>> {
    vec![2 * p[0] - p[1], 2 * p[1] - p[0]]
        .into_iter()
        .filter(|p| inbounds(p, nrows, ncols))
        .collect()
}

fn group_by_freq(antennas: &[(u8, Complex<i32>)]) -> HashMap<u8, HashSet<Complex<i32>>> {
    let mut groups: HashMap<u8, HashSet<Complex<i32>>> = HashMap::default();
    for (freq, pos) in antennas {
        groups.entry(*freq).or_default().insert(*pos);
    }
    groups
}

fn inbounds(pos: &Complex<i32>, nrows: i32, ncols: i32) -> bool {
    pos.re >= 0 && pos.re < nrows && pos.im >= 0 && pos.im < ncols
}

fn antinodes(p: &[Complex<i32>], nrows: i32, ncols: i32) -> Vec<Complex<i32>> {
    let mut positions = p.to_vec();
    let diff = p[1] - p[0];

    let mut curr = p[0] - diff;
    while inbounds(&curr, nrows, ncols) {
        positions.push(curr);
        curr -= diff;
    }
    curr = p[1] + diff;
    while inbounds(&curr, nrows, ncols) {
        positions.push(curr);
        curr += diff;
    }

    positions
}

fn solve(input: &str, antinode_gen: fn(&[Complex<i32>], i32, i32) -> Vec<Complex<i32>>) -> u32 {
    let grid = input.lines().map(|l| l.as_bytes()).collect_vec();
    let nrows = grid.len() as i32;
    let ncols = grid[0].len() as i32;

    let antennas = iproduct!(0..grid.len(), 0..grid[0].len())
        .filter(|&(r, c)| grid[r][c] != b'.')
        .map(|(r, c)| (grid[r][c], Complex::new(r as i32, c as i32)))
        .collect_vec();
    group_by_freq(&antennas)
        .into_iter()
        .flat_map(|group| {
            group
                .1
                .into_iter()
                .combinations(2)
                .map(|p| antinode_gen(&p, ncols, nrows))
                .collect_vec()
        })
        .flatten()
        .collect::<HashSet<Complex<i32>>>()
        .len() as u32
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");

    aoc2024_utils::timeit("Part 1", || solve(input, antinode_pair));
    aoc2024_utils::timeit("Part 2", || solve(input, antinodes));
    Ok(())
}
