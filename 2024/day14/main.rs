use itertools::Itertools;
use num::complex::Complex;
use rustc_hash::FxHashMap as HashMap;

#[derive(Debug)]
struct Robot {
    p: Complex<i32>,
    v: Complex<i32>,
}

fn final_pos(robot: &Robot, time: i32, size: Complex<i32>) -> Complex<i32> {
    let mut pos = robot.p + robot.v * time;
    pos.re = pos.re.rem_euclid(size.re);
    pos.im = pos.im.rem_euclid(size.im);
    pos
}

fn quadrant(pos: Complex<i32>, size: Complex<i32>) -> Option<u32> {
    let midpoint = size / 2;
    let mut quad = 0;
    if pos.re == midpoint.re || pos.im == midpoint.im {
        return None;
    }
    if pos.re < midpoint.re {
        quad += 1;
    }
    if pos.im < midpoint.im {
        quad += 2;
    }
    Some(quad)
}

fn count_frequencies(values: &[u32]) -> HashMap<u32, u32> {
    let mut freq: HashMap<u32, u32> = HashMap::default();
    for val in values {
        *freq.entry(*val).or_insert(0) += 1;
    }
    freq
}

fn part1(input: &str, size: Complex<i32>) -> u64 {
    let quadrants = input
        .lines()
        .filter_map(|l| {
            let data = aoc2024_utils::extract_i32s(l);
            let p = Complex::new(data[0], data[1]);
            let v = Complex::new(data[2], data[3]);
            let fp = final_pos(&Robot { p, v }, 100, size);
            quadrant(fp, size)
        })
        .collect_vec();
    count_frequencies(&quadrants).values().product::<u32>() as u64
}

fn print_points(points: &[Complex<i32>], size: Complex<i32>) {
    for r in 0..size.re {
        for c in 0..size.im {
            if points.contains(&Complex::new(r, c)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn part2(input: &str, size: Complex<i32>) -> u32 {
    let robots = input
        .lines()
        .map(|l| {
            let data = aoc2024_utils::extract_i32s(l);
            let p = Complex::new(data[0], data[1]);
            let v = Complex::new(data[2], data[3]);
            Robot { p, v }
        })
        .collect_vec();

    let mut time: i32 = 0;
    loop {
        let cur_pos = robots
            .iter()
            .map(|r| final_pos(r, time, size))
            .collect_vec();

        // Check if line 33 has a bunch of consecutive points in a row
        if (21..40).all(|y| cur_pos.contains(&Complex::new(33, y))) {
            print_points(&cur_pos, size);
            return time as u32;
        }
        time += 1;
    }
}

fn main() -> std::io::Result<()> {
    //let (input, size) = (include_str!("sample.txt"), Complex::new(11, 7));
    let (input, size) = (include_str!("input.txt"), Complex::new(101, 103));

    aoc2024_utils::timeit_u64("Part 1", || part1(input, size));
    aoc2024_utils::timeit("Part 2", || part2(input, size));
    Ok(())
}
