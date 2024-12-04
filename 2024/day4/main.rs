fn look_for_xmas(r: usize, c: usize, rows: &Vec<&str>) -> u32 {
    let look_dirs = [
        [(0, 0), (0, 1), (0, 2), (0, 3)], // horizontal
        [(0, 0), (1, 0), (2, 0), (3, 0)], // vertical
        [(0, 0), (1, 1), (2, 2), (3, 3)], // diagonal down
        [(3, 0), (2, 1), (1, 2), (0, 3)], // diagonal up
    ];
    let ncols = rows[0].len();
    let nrows = rows.len();

    look_dirs
        .iter()
        .map(|dirs| {
            dirs.iter()
                .map(|(dr, dc)| {
                    if r + dr < nrows && c + dc < ncols {
                        rows[r + dr].as_bytes()[c + dc]
                    } else {
                        b' '
                    }
                })
                .collect::<Vec<u8>>()
        })
        .filter(|s| s == "XMAS".as_bytes() || s == "SAMX".as_bytes())
        .count() as u32
}

fn look_for_x_mas(r: usize, c: usize, rows: &Vec<&str>) -> u32 {
    let look_dirs = [
        [(0, 0), (1, 1), (2, 2)], // diagonal down
        [(2, 0), (1, 1), (0, 2)], // diagonal up
    ];
    let ncols = rows[0].len();
    let nrows = rows.len();

    look_dirs
        .iter()
        .map(|dirs| {
            dirs.iter()
                .map(|(dr, dc)| {
                    if r + dr < nrows && c + dc < ncols {
                        rows[r + dr].as_bytes()[c + dc]
                    } else {
                        b' '
                    }
                })
                .collect::<Vec<u8>>()
        })
        .filter(|s| s == "MAS".as_bytes() || s == "SAM".as_bytes())
        .count() as u32
}

fn part1(input: &str) -> u32 {
    let rows = input.lines().collect::<Vec<_>>();
    let ncols = rows[0].len();
    let nrows = rows.len();

    (0..ncols)
        .map(|c| (0..nrows).map(|r| look_for_xmas(r, c, &rows)).sum::<u32>())
        .sum()
}

fn part2(input: &str) -> u32 {
    let rows = input.lines().collect::<Vec<_>>();
    let ncols = rows[0].len();
    let nrows = rows.len();

    (0..ncols)
        .map(|c| {
            (0..nrows)
                .filter(|&r| look_for_x_mas(r, c, &rows) == 2)
                .count()
        })
        .sum::<usize>() as u32
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");
    aoc2024_utils::timeit("Part 1", || part1(&input));
    aoc2024_utils::timeit("Part 2", || part2(&input));
    Ok(())
}
