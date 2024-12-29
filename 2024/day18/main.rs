use itertools::Itertools;
use num::complex::Complex;
use rustc_hash::FxHashSet as HashSet;
use std::collections::VecDeque;

fn parse(input: &str) -> Vec<Complex<i32>> {
    input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            Complex::new(x.parse().unwrap(), y.parse().unwrap())
        })
        .collect_vec()
}

const DIRS: [Complex<i32>; 4] = [
    Complex::new(1, 0),
    Complex::new(-1, 0),
    Complex::new(0, 1),
    Complex::new(0, -1),
];

fn inbounds(pos: &Complex<i32>, max_xy: i32) -> bool {
    pos.re >= 0 && pos.re <= max_xy && pos.im >= 0 && pos.im <= max_xy
}

fn bfs(points: &[Complex<i32>], max_xy: i32, start: Complex<i32>, end: Complex<i32>) -> u32 {
    let mut queue = VecDeque::new();
    let mut seen = HashSet::default();
    queue.push_back((start, 0));
    while let Some((pos, dist)) = queue.pop_front() {
        if pos == end {
            return dist;
        }
        for dir in DIRS {
            let new_pos = pos + dir;
            if inbounds(&new_pos, max_xy) && !points.contains(&new_pos) && !seen.contains(&new_pos)
            {
                seen.insert(new_pos);
                queue.push_back((new_pos, dist + 1));
            }
        }
    }
    unreachable!()
}

fn shortest_path_len(points: &[Complex<i32>], max_xy: i32) -> u32 {
    let start = Complex::new(0, 0);
    let end = Complex::new(max_xy, max_xy);
    bfs(points, max_xy, start, end)
}

fn part1(input: &str, max_xy: i32, load: usize) -> u32 {
    let points = parse(input);
    dbg!(&points[..load]);
    shortest_path_len(&points[..load], max_xy)
}

fn main() -> std::io::Result<()> {
    //let (input, max_xy, load) = (include_str!("sample.txt"), 6, 12);
    let (input, max_xy, load) = (include_str!("input.txt"), 70, 1024);

    aoc2024_utils::timeit("Part 1", || part1(input, max_xy, load));
    //aoc2024_utils::timeit("Part 2", || part2(input));
    Ok(())
}
