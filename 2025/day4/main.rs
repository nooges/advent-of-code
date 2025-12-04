use num::complex::Complex;
use rustc_hash::FxHashSet as HashSet;

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

fn parse_input(input: &str) -> HashSet<Complex<i32>> {
    input
        .lines()
        .enumerate()
        .flat_map(|(r, l)| {
            l.chars().enumerate().filter_map(move |(c, v)| match v {
                '@' => Some(Complex::new(r as i32, c as i32)),
                _ => None,
            })
        })
        .collect()
}

fn removable(p: Complex<i32>, points: &HashSet<Complex<i32>>) -> bool {
    let mut cnt = 0;
    for d in DIRS.iter() {
        if points.contains(&(d + p)) {
            cnt += 1;
            if cnt >= 4 {
                return false;
            }
        }
    }
    true
}

fn part1(input: &str) -> u32 {
    let points = parse_input(input);
    points
        .iter()
        .filter_map(|p| removable(*p, &points).then_some(()))
        .count() as u32
}

fn remove_rolls(points: &HashSet<Complex<i32>>) -> HashSet<Complex<i32>> {
    points
        .iter()
        .copied()
        .filter(|p| !removable(*p, points))
        .collect()
}

fn part2(input: &str) -> u32 {
    let mut points = parse_input(input);
    let num_rolls = points.len();

    // Cycle through removing rolls until points stay the same
    loop {
        let new_points = remove_rolls(&points);
        println!("{} -> {}", points.len(), new_points.len());
        if new_points.len() == points.len() {
            return num_rolls as u32 - new_points.len() as u32;
        }
        points = new_points;
    }
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");

    aoc2025_utils::timeit("Part 1", || part1(input));
    aoc2025_utils::timeit("Part 2", || part2(input));
    Ok(())
}
