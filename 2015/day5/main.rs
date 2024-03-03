use std::fs;

const BAD_PAIRS: [&str; 4] = ["ab", "cd", "pq", "xy"];

fn is_vowel(c: &char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}

fn has_three_vowels(s: &str) -> bool {
    return s.chars().filter(|c| is_vowel(c)).count() >= 3;
}

fn has_double_letter(s: &str) -> bool {
    let mut last = '\0';
    for c in s.chars() {
        if c == last {
            return true;
        }
        last = c;
    }
    return false;
}

fn has_no_bad_pairs(s: &str) -> bool {
    return !BAD_PAIRS.iter().any(|p| s.contains(p));
}

fn part1(input: &str) -> u32 {
    let rules = [has_three_vowels, has_double_letter, has_no_bad_pairs];
    return input.lines().filter(|l| rules.iter().all(|f| f(l))).count() as u32;
}

fn has_non_overlapping_pairs(s: &str) -> bool {
    for i in 0..s.len() - 2 {
        if s[i + 2..].contains(&s[i..i + 2]) {
            return true;
        }
    }
    return false;
}

fn has_sandwiched_letter(s: &str) -> bool {
    return s.chars().skip(2).zip(s.chars()).any(|(a, b)| a == b);
}

fn part2(input: &str) -> u32 {
    let rules = [has_non_overlapping_pairs, has_sandwiched_letter];
    return input.lines().filter(|l| rules.iter().all(|f| f(l))).count() as u32;
}

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    utils::timeit("Part 1", || part1(&input));
    utils::timeit("Part 2", || part2(&input));
    Ok(())
}
