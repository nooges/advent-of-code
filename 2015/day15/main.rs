fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input.lines().map(utils::extract_i32s).collect()
}

fn score(ingredients: &[Vec<i32>], amounts: &[i32]) -> u32 {
    let mut attributes = [0, 0, 0, 0];
    ingredients.iter().enumerate().for_each(|(i, ingredient)| {
        (0..4).for_each(|j| {
            attributes[j] += ingredient[j] * amounts[i];
        });
    });
    (0..4).for_each(|i| {
        attributes[i] = attributes[i].max(0);
    });
    attributes.iter().product::<i32>() as u32
}

fn calories(ingredients: &[Vec<i32>], amounts: &[i32]) -> i32 {
    ingredients
        .iter()
        .enumerate()
        .map(|(i, v)| v[4] * amounts[i])
        .sum()
}

fn combos(n: i32, rem: i32) -> Vec<Vec<i32>> {
    match (n, rem) {
        (1, _) => vec![vec![rem]],
        (_, 0) => vec![],
        (_, _) => {
            let mut ret = vec![];
            for i in 0..=rem {
                let mut v = combos(n - 1, rem - i);
                v.iter_mut().for_each(|v| v.push(i));
                ret.append(&mut v);
            }
            ret
        }
    }
}

fn part1(input: &str) -> u32 {
    let ingredients = parse_input(input);
    combos(ingredients.len() as i32, 100)
        .iter()
        .map(|v| score(&ingredients, v))
        .max()
        .unwrap()
}

fn part2(input: &str) -> u32 {
    let ingredients = parse_input(input);
    combos(ingredients.len() as i32, 100)
        .iter()
        .filter(|v| calories(&ingredients, v) == 500)
        .map(|v| score(&ingredients, v))
        .max()
        .unwrap()
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");
    utils::timeit("Part 1", || part1(&input));
    utils::timeit("Part 2", || part2(&input));

    Ok(())
}
