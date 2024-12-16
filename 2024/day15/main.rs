use itertools::Itertools;
use num::complex::Complex;
use rustc_hash::FxHashMap as HashMap;
use std::sync::OnceLock;

#[derive(Debug)]
struct GridRC {
    values: Vec<Vec<char>>,
    nrows: i32,
    ncols: i32,
}

impl GridRC {
    fn get(&self, pos: &Complex<i32>) -> Option<char> {
        self.values
            .get(pos.re as usize)?
            .get(pos.im as usize)
            .copied()
    }

    fn set(&mut self, pos: &Complex<i32>, c: char) {
        self.values[pos.re as usize][pos.im as usize] = c;
    }

    fn print(&self) {
        for row in &self.values {
            println!("{}", row.iter().collect::<String>());
        }
    }

    fn find_char(
        &self,
        c: char,
        pos: Complex<i32>,
        dir: Complex<i32>,
    ) -> Option<(Complex<i32>, i32)> {
        let mut p = pos;
        let mut i = 0;
        loop {
            p += dir;
            i += 1;
            if p.re < 0 || p.re >= self.nrows || p.im < 0 || p.im >= self.ncols {
                return None;
            }
            if self.get(&p) == Some(c) {
                return Some((p, i));
            }
        }
    }
}

fn dirs() -> &'static HashMap<char, Complex<i32>> {
    static HASHMAP: OnceLock<HashMap<char, Complex<i32>>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        let mut m = HashMap::default();
        m.insert('>', Complex::new(0, 1));
        m.insert('<', Complex::new(0, -1));
        m.insert('v', Complex::new(1, 0));
        m.insert('^', Complex::new(-1, 0));
        m
    })
}

fn parse(input: &str) -> (GridRC, Complex<i32>, Vec<Complex<i32>>) {
    let (grid_str, cmd_str) = input.split_once("\n\n").unwrap();
    let grid_lines = grid_str.lines().collect_vec();
    let nrows = grid_lines.len() as i32;
    let ncols = grid_lines[0].len() as i32;
    let grid = GridRC {
        values: grid_lines
            .iter()
            .map(|l| l.chars().collect_vec())
            .collect_vec(),
        nrows,
        ncols,
    };
    let p = grid
        .values
        .iter()
        .flatten()
        .position(|c| *c == '@')
        .unwrap();
    let start = Complex::new(p as i32 / ncols, p as i32 % ncols);

    let commands = cmd_str
        .chars()
        .filter_map(|c| dirs().get(&c).copied())
        .collect_vec();
    (grid, start, commands)
}

fn part1(input: &str) -> u32 {
    let (mut grid, start, commands) = parse(input);
    grid.print();
    grid.set(&start, '.');

    let mut bot_pos = start;
    for cmd in commands {
        let next_wall = grid.find_char('#', bot_pos, cmd);
        let next_space = grid.find_char('.', bot_pos, cmd);
        let next_box = grid.find_char('O', bot_pos, cmd);

        match (next_wall, next_space, next_box) {
            (Some((_, 1)), _, _) => {
                // Don't move bot
            }
            (_, Some((sp, 1)), _) => {
                bot_pos = sp;
            }
            (Some((_, wd)), Some((sp, sd)), Some((bp, 1))) => {
                // There's space, so push boxes
                if sd < wd {
                    grid.set(&bp, '.');
                    grid.set(&sp, 'O');
                    bot_pos = bp;
                }
            }
            _ => {
                println!("Unreachable");
            }
        }

        //grid.print();
    }

    (0..grid.nrows)
        .flat_map(|r| (0..grid.ncols).map(move |c| Complex::new(r, c)))
        .filter(|p| grid.get(p) == Some('O'))
        .map(|p| 100 * p.re + p.im)
        .sum::<i32>() as u32
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");

    aoc2024_utils::timeit("Part 1", || part1(input));
    //aoc2024_utils::timeit("Part 2", || part2(input, size));
    Ok(())
}
