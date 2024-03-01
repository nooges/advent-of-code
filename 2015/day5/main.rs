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

fn has_bad_pair(s: &str) -> bool {
    return BAD_PAIRS.iter().any(|p| s.contains(p));
}

fn part1(input: &str) -> u32 {
    return input
        .lines()
        .filter(|l| has_three_vowels(l) && has_double_letter(l) && !has_bad_pair(l))
        .count() as u32;
}

fn part2(input: &str) -> u32 {
    return 0;
}

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let part1_result = part1(&input);
    println!("Part 1: {}", part1_result);
    let part2_result = part2(&input);
    println!("Part 2: {}", part2_result);
    Ok(())
}
