fn look(look_dirs: &[Vec<(usize, usize)>], search: &str, r: usize, c: usize, rows: &[&str]) -> u32 {
    let ncols = rows[0].len();
    let nrows = rows.len();
    let search_fwd = search.as_bytes();
    let search_rev = &search_fwd.iter().rev().copied().collect::<Vec<_>>();

    look_dirs
        .iter()
        .map(|dirs| {
            dirs.iter()
                .map(|(dr, dc)| {
                    let (nr, nc) = (r + dr, c + dc);
                    if nr < nrows && nc < ncols {
                        rows[nr].as_bytes()[nc]
                    } else {
                        b' '
                    }
                })
                .collect::<Vec<u8>>()
        })
        .filter(|s| s == search_fwd || s == search_rev)
        .count() as u32
}

fn part1(input: &str) -> u32 {
    let rows = input.lines().collect::<Vec<_>>();

    let look_dirs = [
        vec![(0, 0), (0, 1), (0, 2), (0, 3)], // horizontal
        vec![(0, 0), (1, 0), (2, 0), (3, 0)], // vertical
        vec![(0, 0), (1, 1), (2, 2), (3, 3)], // diagonal down
        vec![(3, 0), (2, 1), (1, 2), (0, 3)], // diagonal up
    ];
    let search = "XMAS";

    (0..rows[0].len())
        .map(|c| {
            (0..rows.len())
                .map(|r| look(&look_dirs, search, r, c, &rows[..]))
                .sum::<u32>()
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let rows = input.lines().collect::<Vec<_>>();

    let look_dirs = [
        vec![(0, 0), (1, 1), (2, 2)], // diagonal down
        vec![(2, 0), (1, 1), (0, 2)], // diagonal up
    ];
    let search = "MAS";

    (0..rows[0].len())
        .map(|c| {
            (0..rows.len())
                .filter(|&r| look(&look_dirs, search, r, c, &rows[..]) == 2)
                .count()
        })
        .sum::<usize>() as u32
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");
    aoc2024_utils::timeit("Part 1", || part1(input));
    aoc2024_utils::timeit("Part 2", || part2(input));
    Ok(())
}
