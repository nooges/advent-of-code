use cached::proc_macro::cached;
use itertools::Itertools;
use rustc_hash::FxHashMap as HashMap;
use rustc_hash::FxHashSet as HashSet;

fn parse(input: &str) -> (HashSet<&str>, Vec<&str>) {
    let (patterns_str, designs_str) = input.split_once("\n\n").unwrap();
    let patterns = HashSet::from_iter(
        patterns_str
            .split(", ")
            .sorted_by(|a, b| b.len().cmp(&a.len())),
    );
    let designs = designs_str.lines().collect_vec();

    println!("Patterns: {patterns:?}");

    (patterns, designs)
}

fn part1(patterns: HashSet<&str>, designs: Vec<&str>) -> u32 {
    let mut possible = patterns.clone();
    let mut impossible = HashSet::default();
    impossible.insert("bwgb");
    impossible.insert("wwugbwgb");
    impossible.insert("bggb");
    impossible.insert("wbburwwbggbbgrrbbwwrruuwrrbuguwugwwugbwgb");
    impossible.insert("bbrbgrgrbguurgwuurrgwuurbrgwrugwbrgwbbuwgbubrwugbggb");
    impossible.insert("rgbugugurwgurrwuuuguggwggrbrbubgwrwuwbbwgbbgwuuugbggb");
    impossible.insert("gbbwuwwbbgwurgwguuwrbgbbgbguruuurrrgbwugwwbrgrgwgrwugbbgb");

    fn is_possible<'a>(
        design: &'a str,
        patterns: &HashSet<&str>,
        possible: &mut HashSet<&'a str>,
        impossible: &mut HashSet<&'a str>,
    ) -> bool {
        if design.is_empty() || possible.contains(design) {
            return true;
        }
        if impossible.contains(design) {
            return false;
        }
        println!("Checking: {design}");
        let possible_prefixes = patterns
            .iter()
            .filter(|p| design.starts_with(*p))
            .collect_vec();
        if possible_prefixes.is_empty() {
            impossible.insert(design);
            return false;
        }
        let possible_suffixes = patterns
            .iter()
            .filter(|p| design.ends_with(*p))
            .collect_vec();
        if possible_suffixes.is_empty() {
            impossible.insert(design);
            return false;
        }

        let result = possible_prefixes.iter().try_for_each(|p| {
            if is_possible(&design[p.len()..], patterns, possible, impossible) {
                possible.insert(design);
                println!("Inserting: {design}");
                return None;
            }
            Some(())
        });

        result.is_none()
    }

    designs
        .iter()
        .filter(|design| {
            dbg!(is_possible(
                design,
                &patterns,
                &mut possible,
                &mut impossible
            ))
        })
        .count() as u32
}

fn main() -> std::io::Result<()> {
    let input = include_str!("sample.txt");
    let (patterns, designs) = parse(input);

    aoc2024_utils::timeit("Part 1", || part1(patterns.clone(), designs.clone()));
    //aoc2024_utils::timeit("Part 2", || part2(patterns.clone(), designs.clone()));
    Ok(())
}
