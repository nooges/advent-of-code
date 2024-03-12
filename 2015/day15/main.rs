use itertools::Itertools;
use std::fs;

#[derive(Debug)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

fn parse_input(input: &str) -> Vec<Ingredient> {
    input
        .lines()
        .map(|line| {
            let parts = utils::extract_i32s(line);
            Ingredient {
                capacity: parts[0],
                durability: parts[1],
                flavor: parts[2],
                texture: parts[3],
                calories: parts[4],
            }
        })
        .collect()
}

fn score(ingredients: &Vec<Ingredient>, amounts: &Vec<i32>) -> u32 {
    let mut capacity = 0;
    let mut durability = 0;
    let mut flavor = 0;
    let mut texture = 0;
    for i in 0..ingredients.len() {
        capacity += ingredients[i].capacity * amounts[i];
        durability += ingredients[i].durability * amounts[i];
        flavor += ingredients[i].flavor * amounts[i];
        texture += ingredients[i].texture * amounts[i];
    }
    capacity = capacity.max(0);
    durability = durability.max(0);
    flavor = flavor.max(0);
    texture = texture.max(0);
    (capacity * durability * flavor * texture) as u32
}

fn part1(input: &str) -> u32 {
    let ingredients = parse_input(input);
    (0..=100)
        .permutations(ingredients.len())
        .filter(|v| v.iter().sum::<i32>() == 100)
        .map(|v| score(&ingredients, &v))
        .max()
        .unwrap()
}

fn part2(input: &str) -> u32 {
    0
}

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    utils::timeit("Part 1", || part1(&input));
    utils::timeit("Part 2", || part2(&input));

    Ok(())
}
