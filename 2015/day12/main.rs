fn part1(input: &str) -> u32 {
    utils::extract_i32s(input).iter().sum::<i32>() as u32
}

enum Mode {
    Object,
    Array,
    Prop,
    None,
}

fn part2(input: &str) -> u32 {
    // TODO: Search for {}[]"":
    // { -> mode = dict
    // }] -> mode = prev mode (pop)
    // [ -> mode = array
    // " -> mode = prev mode if 2nd "
    // : -> mode = prop
    let mut modes = vec![Mode::None];
    let mut num_chars = vec![];
    let mut in_string = false;
    let mut sum = 0;
    input.windows(3).for_each(|w| {
        if w[0] == '{' {
            modes.push(Mode::Object);
        }
        if w[0] == '}' || w[0] == ']' {
            modes.pop();
        }
        if w[0] == '[' {
            modes.push(Mode::Array);
        }
        if w[0] == ':' && !in_string {
            modes.push(Mode::Prop);

    })
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");
    utils::timeit("Part 1", || part1(&input));
    utils::timeit("Part 2", || part2(&input));

    Ok(())
}
