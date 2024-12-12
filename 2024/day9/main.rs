use itertools::Itertools;
use rustc_hash::FxHashMap as HashMap;

fn part1(input: &str) -> u64 {
    let mut disk: Vec<i64> = Vec::with_capacity(100000);
    let mut chars = input.chars().collect_vec();
    chars.push('0');

    let diskmap = chars
        .iter()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .tuples::<(usize, usize)>()
        .collect_vec();
    diskmap.iter().enumerate().for_each(|(id, (len, space))| {
        disk.extend(vec![id as i64; *len]);
        disk.extend(vec![-1; *space]);
    });

    // Rearrange
    let mut cur_l = diskmap[0].0;
    let mut cur_r = disk.len() - 1;
    loop {
        while disk[cur_l] != -1 {
            cur_l += 1;
        }
        while disk[cur_r] == -1 {
            cur_r -= 1;
        }
        if cur_l > cur_r {
            break;
        }
        disk.swap(cur_l, cur_r);
        cur_l += 1;
        cur_r -= 1;
    }

    (0..cur_r + 1)
        .map(|i| i as u64 * disk[i] as u64)
        .sum::<u64>()
}

fn part2(input: &str) -> u64 {
    let mut chars = input.chars().collect_vec();
    chars.push('0');

    let diskmap = chars
        .iter()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .tuples::<(usize, usize)>()
        .collect_vec();
    let mut files: HashMap<usize, (&usize, usize)> = HashMap::default();
    let mut freespace: Vec<(usize, usize)> = Vec::default();

    let mut pos = 0;
    diskmap.iter().enumerate().for_each(|(id, (len, space))| {
        files.insert(id, (len, pos));
        freespace.push((*space, pos + len));
        pos += len + space;
    });

    (0..diskmap.len()).rev().for_each(|id| {
        if let Some(index) = freespace
            .iter()
            .position(|i| i.1 < files[&id].1 && i.0 >= *files[&id].0)
        {
            let update = (files[&id].0, freespace[index].1);
            files.insert(id, update);
            if freespace[index].0 == *files[&id].0 {
                freespace.remove(index);
            } else {
                freespace[index].0 -= files[&id].0;
                freespace[index].1 += files[&id].0;
            }
        }
    });

    files
        .iter()
        .map(|(id, (&size, pos))| {
            (0..size)
                .map(|i| *id as u64 * (pos + i) as u64)
                .sum::<u64>()
        })
        .sum()
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");

    aoc2024_utils::timeit_u64("Part 1", || part1(input));
    aoc2024_utils::timeit_u64("Part 2", || part2(input));
    Ok(())
}
